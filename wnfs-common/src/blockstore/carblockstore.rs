use crate::BlockStore;
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use libipld::{Cid, IpldCodec};
use std::{
    borrow::Cow,
    collections::HashMap,
    fs::File,
    io::{BufWriter, Read, Seek, SeekFrom, Write},
    path::PathBuf,
    sync::RwLock,
};

struct LocationInCar {
    car_number: usize,
    offset: usize,
}
// TODO make sure most of the things go into the same local car file. hard. need to change blockstore interface. rip.
struct DiskCarFactory {
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
    // rotating the CAR file
    fn rotate(&mut self) -> Result<()> {
        // If there is a car to close
        if self.current_car.is_some() {
            // Close the current CAR file
            self.current_car.take().unwrap().flush()?;
        }
        // increment the car number
        self.car_number += 1;
        // reset the current size
        self.current_size = 0;
        // open the new CAR file
        let new_car = File::create(self.directory.join(format!("{}.car", self.car_number)))?;
        self.current_car = Some(BufWriter::new(new_car));
        Ok(())
    }
}

pub struct CarBlockStore {
    /// The number of bytes that each CAR file can hold.
    max_size: usize,
    /// Index of which blocks are in which files (by CAR number), and the offset in the file.
    index: RwLock<HashMap<Cid, LocationInCar>>,
    /// The current state of the CAR files.
    car_factory: RwLock<DiskCarFactory>,
}

impl CarBlockStore {
    pub fn new(directory: PathBuf, max_size: usize) -> Self {
        // create the directory if it doesn't exist
        std::fs::create_dir_all(&directory).unwrap();

        // create the CAR file factory
        let car_factory = DiskCarFactory {
            car_number: 0,
            current_size: 0,
            directory,
            current_car: None,
        };

        // create the index
        let index = RwLock::new(HashMap::new());

        // create the block store
        Self {
            max_size,
            index,
            car_factory: RwLock::new(car_factory),
        }
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

        // If there is no CAR or we don't have enough space left to fit this data
        if factory.current_car.is_none()
            || factory.current_size + block_size as usize + block_size_bytes.len() > self.max_size
        {
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

        // Return generated CID for future retrieval
        Ok(cid)
    }
}
