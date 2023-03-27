use crate::BlockStore;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use libipld::{cbor::DagCborCodec, ipld, prelude::Codec, Cid, Ipld, IpldCodec};
use serde::{Deserialize, Serialize, Serializer};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    str::FromStr,
    sync::RwLock,
    vec,
};

pub struct CarBlockStore {
    /// The version number and list of root dir CIDs
    carhead: CarHeader,
    /// The number of bytes that each CAR file can hold.
    max_size: Option<usize>,
    /// Index of which blocks are in which files (by CAR number), and the offset in the file.
    index: RwLock<HashMap<Cid, LocationInCar>>,
    /// The current state of the CAR files.
    car_factory: RwLock<DiskCarFactory>,
}

impl Serialize for CarBlockStore {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let header_bytes = self.carhead.to_bytes();
        let mut string_keyed_index: HashMap<String, &LocationInCar> = HashMap::new();
        let binding = self.index.read().unwrap();
        for (k, v) in binding.iter() {
            string_keyed_index.insert(k.to_string(), v);
        }

        (
            header_bytes,
            self.max_size,
            &string_keyed_index,
            &self.car_factory,
        )
            .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CarBlockStore {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        // Deserialize the path
        let (header_bytes, max_size, string_keyed_index, car_factory) =
            <(
                Vec<u8>,
                Option<usize>,
                HashMap<String, LocationInCar>,
                RwLock<DiskCarFactory>,
            )>::deserialize(deserializer)?;
        let carhead = CarHeader::from_bytes(&header_bytes);
        let mut index: HashMap<Cid, LocationInCar> = HashMap::new();
        for (k, v) in string_keyed_index.into_iter() {
            let cid = Cid::from_str(&k).unwrap();
            index.insert(cid, v);
        }

        let index = RwLock::new(index);

        // Return Ok status with the new DiskBlockStore
        Ok(Self {
            carhead,
            max_size,
            index,
            car_factory,
        })
    }
}

impl CarBlockStore {
    /// Create a new CarBlockStore at a given directory; overwrites all data
    pub fn new(directory: &Path, max_size: Option<usize>) -> Self {
        // Remove anything that might be there already
        let _ = std::fs::create_dir_all(directory);
        // Create the directory
        std::fs::create_dir_all(directory).unwrap();
        // Create the CAR Header
        let carhead = CarHeader::new();
        // Create the indexer
        let index = RwLock::new(HashMap::new());
        // Create the CAR file factory
        let car_factory = DiskCarFactory::new(directory);
        // Instantiate the block store
        Self {
            carhead,
            max_size,
            index,
            car_factory: RwLock::new(car_factory),
        }
    }

    /// Public function to change the directory in which CAR files are read
    pub fn change_dir(&mut self, new_directory: &Path) -> Result<()> {
        // Grab RW lock on CAR factory
        let factory: &mut DiskCarFactory = self.car_factory.get_mut().unwrap();
        // Update the directory
        factory.directory = new_directory.to_path_buf();
        // Return OK
        Ok(())
    }

    pub fn add_root(&mut self, cid: &Cid) {
        self.carhead.roots.push(*cid);
    }

    pub fn get_roots(&self) -> Vec<Cid> {
        self.carhead.roots.clone()
    }
}

#[async_trait(?Send)]
impl BlockStore for CarBlockStore {
    // TODO audit this for deadlocks.
    async fn get_block(&self, cid: &Cid) -> Result<Cow<Vec<u8>>> {
        // Get a read-only reference to the <Cid, LocationInCar> HashMap
        let index = self.index.read().unwrap();
        // Use that HashMap to look up the Cid provided to us
        let location: &LocationInCar = index.get(cid).ok_or(anyhow!("CID not found"))?;

        // Open the CAR file
        let mut car_file: File;
        {
            // Grab read-only
            let factory = self.car_factory.read().unwrap();
            // Open the CAR file using the CAR number as the filename
            car_file = File::open(
                factory
                    .directory
                    .join(format!("{}.car", location.car_number)),
            )?;
        }
        // Drop the read lock on the CAR Factory

        // Move to the correct offset point in the CAR file
        car_file.seek(SeekFrom::Start(location.offset as u64))?;

        // Create a buffer to store the Block Size
        let mut block_size_bytes = [0u8; 16];
        // Read the block size exactly, filling the buffer
        car_file.read_exact(&mut block_size_bytes)?;
        // Represent this as a number by interpreting the bytes as a Little Endian number
        let block_size = u128::from_le_bytes(block_size_bytes);
        // Create a buffer to store the actual block
        let mut block = vec![0u8; block_size.try_into().unwrap()];
        // Read in the block
        car_file.read_exact(&mut block)?;
        // Read the preliminary bytes of the block as a CID
        let cid1 = Cid::read_bytes(block.as_slice())?;
        // Exactract the non-cid block content from the block in totality
        let block_content = block[cid.encoded_len()..].to_vec();
        // Use the block content to generate another CID
        let cid2 = self.create_cid(&block_content, IpldCodec::try_from(cid.codec())?)?;
        // Return the block content if CIDs match; error otherwise
        if cid1 == cid2 {
            Ok(Cow::Owned(block_content))
        } else {
            Err(anyhow!("CID mismatch"))
        }
    }

    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // Get the CID for the block
        let cid = self.create_cid(&bytes, codec)?;
        // Represent the CID as bytes
        let cid_bytes = cid.to_bytes();
        // Determine the amount of space we need to allocate
        let block_size: u128 = cid_bytes.len() as u128 + bytes.len() as u128;
        // Represent that number as a Little Endian byte array
        let block_size_bytes = block_size.to_le_bytes();

        // Grab a mutable reference to the CarFactory
        let mut factory = self.car_factory.write().unwrap();

        // Determine if the data being put will exceed CAR file limit
        let data_too_big = if let Some(max_size) = self.max_size {
            factory.current_size + block_size as usize + block_size_bytes.len() > max_size
        }
        // If there is no max size the data is never too big
        else {
            false
        };

        // If there is no CAR or we don't have enough space left to fit this data
        if factory.current_car.is_none() || data_too_big {
            // Rotate the CAR to make room
            factory.rotate()?;
        }

        // Grab a mutable reference to the CarFile's BufWriter
        let writable_car: &mut BufWriter<File> = factory.current_car.as_mut().unwrap();

        // Write the block size to the current CAR file
        writable_car.write_all(&block_size_bytes)?;
        // Write the CID of the block
        writable_car.write_all(&cid_bytes)?;
        // Write the contents of the block
        writable_car.write_all(&bytes)?;
        // Flush the Writer to ensure that those bytes are actually written
        writable_car.flush().unwrap();

        // Denote LocationInCar
        let loc = LocationInCar {
            car_number: factory.car_number,
            offset: factory.current_size,
        };

        // Increment the size of the current CAR
        factory.current_size += block_size as usize + block_size_bytes.len();

        // Grab write lock and insert the <Cid, LocationInCar> pairing into the HashMap
        self.index
            .write()
            .map_err(|e| anyhow!("{e}: couldn't get write lock"))?
            .insert(cid, loc);

        // TODO (organizedgrime)
        // might there be scenarios we want to do this given that we don't know when someone might serialize?
        // factory.finish()?;

        // Return generated CID for future retrieval
        Ok(cid)
    }
}

#[derive(Serialize, Deserialize)]
struct CarHeader {
    version: i128,
    roots: Vec<Cid>,
}

impl CarHeader {
    pub fn new() -> Self {
        Self {
            version: 1,
            roots: Vec::new(),
        }
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let roots = Ipld::List(self.roots.clone().into_iter().map(Ipld::Link).collect());
        let header_ipld: Ipld = ipld!({
          "version": self.version,
          "roots": roots,
        });
        DagCborCodec.encode(&header_ipld).unwrap()
    }

    pub fn from_bytes(bytes: &[u8]) -> Self {
        let header_ipld: Ipld = DagCborCodec.decode(bytes).unwrap();
        let Ipld::Integer(version) = header_ipld.get("version").unwrap() else { panic!() };
        let Ipld::List(roots) = header_ipld.get("roots").unwrap() else { panic!() };

        let roots: Vec<Cid> = roots
            .iter()
            .map(|ipld| {
                let Ipld::Link(cid) = ipld else { panic!() };
                *cid
            })
            .collect();

        CarHeader {
            version: *version,
            roots,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct LocationInCar {
    car_number: usize,
    offset: usize,
}
// TODO make sure most of the things go into the same local car file. hard. need to change blockstore interface. rip.
pub struct DiskCarFactory {
    /// car file number
    car_number: usize,
    /// The number of bytes currently stored in the current CAR file.
    current_size: usize,
    /// directory where the CAR files are stored
    directory: PathBuf,
    /// The current CAR file.
    current_car: Option<BufWriter<File>>,
}

impl DiskCarFactory {
    fn new(directory: &Path) -> Self {
        Self {
            car_number: 0,
            current_size: 0,
            directory: directory.to_path_buf(),
            current_car: None,
        }
    }

    // Flush the BufWriter
    fn finish(&mut self) -> Result<()> {
        // If there is a car to close
        if self.current_car.is_some() {
            // Close the current CAR file
            self.current_car.take().unwrap().flush()?;
            // Empty the Option
            self.current_car = None;
        }
        // Return OK status
        Ok(())
    }

    // rotating the CAR file
    fn rotate(&mut self) -> Result<()> {
        // Finish the current CAR file
        self.finish()?;
        // increment the car number
        self.car_number += 1;
        // reset the current size
        self.current_size = 0;
        // Construct the new CAR path
        let path = self.directory.join(format!("{}.car", self.car_number));
        // Create the new BufWriter
        self.current_car = Some(BufWriter::new(File::create(path)?));
        Ok(())
    }
}

impl Serialize for DiskCarFactory {
    fn serialize<S>(self: &DiskCarFactory, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        (&self.car_number, &self.current_size, &self.directory).serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for DiskCarFactory {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let (car_number, current_size, directory) =
            <(usize, usize, PathBuf)>::deserialize(deserializer).unwrap();

        Ok(Self {
            car_number,
            current_size,
            directory,
            current_car: None,
        })
    }
}
