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
        // close the current CAR file
        self.current_car.take().unwrap().flush()?;
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
    /// index of which blocks are in which files (by CAR number), and the offset in the file.
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
        // get the CAR number from the index
        let index = self.index.read().unwrap();
        let location = index.get(cid).ok_or(anyhow!("CID not found"))?;

        // lock the inner state (this is a long lock!)
        let inner = self.car_factory.read().unwrap();
        // open the CAR file
        let mut car_file =
            File::open(inner.directory.join(format!("{}.car", location.car_number)))?;

        // read the block from the CAR file
        car_file.seek(SeekFrom::Start(location.offset as u64))?;

        // read the block length as a u128
        let mut block_length_bytes = [0u8; 16];
        car_file.read_exact(&mut block_length_bytes)?;
        let block_length = u128::from_le_bytes(block_length_bytes);

        // read in the block
        let mut block = vec![0u8; block_length.try_into().unwrap()];
        car_file.read_exact(&mut block)?;

        // get the CID out of the block
        let cid = Cid::read_bytes(block.as_slice())?;

        // compute the CID of the rest of the block
        let block_bytes = &block[cid.encoded_len()..];
        let cid2 = self.create_cid(block_bytes.to_vec(), IpldCodec::try_from(cid.codec())?)?;

        // check that the CID matches the one we were looking for
        if cid != cid2 {
            return Err(anyhow!("CID mismatch"));
        }

        // return the block
        Ok(Cow::Owned(block))
    }

    async fn put_block(&self, bytes: Vec<u8>, codec: IpldCodec) -> Result<Cid> {
        // get the CID for the block
        let cid = self.create_cid(bytes.clone(), codec)?;
        let cid_bytes = cid.to_bytes();

        // yeah screw varints
        let block_length: u128 = cid_bytes.len() as u128 + bytes.len() as u128;
        let block_length_bytes = block_length.to_le_bytes();

        let mut inner = self.car_factory.write().unwrap();

        // can this car file hold the block?
        if inner.current_car.is_none()
            || inner.current_size + block_length as usize + block_length_bytes.len() > self.max_size
        {
            // no, so create a new car file with rotate
            inner.rotate()?;
        }

        let loc = LocationInCar {
            car_number: inner.car_number,
            offset: inner.current_size,
        };

        // write the block to the current CAR file
        inner
            .current_car
            .as_mut()
            .unwrap()
            .write_all(&block_length_bytes)?;
        inner.current_car.as_mut().unwrap().write_all(&cid_bytes)?;
        inner.current_car.as_mut().unwrap().write_all(&bytes)?;

        // update block size and index
        inner.current_size += block_length as usize + block_length_bytes.len();
        self.index
            .write()
            .map_err(|e| anyhow!("{e}: couldn't get write lock"))?
            .insert(cid, loc);

        // return
        Ok(cid)
    }
}

// TODO test properly
// #[cfg(test)]
// mod tests {
//     use super::*;
//     use tempfile::tempdir;
//     use libipld::Ipld;
//     use std::io::BufReader;

//     // TODO WAY more tests on this- this is just a smoke test.
//     #[tokio::test]
//     async fn test_car_block_store() {
//         let dir = tempdir().unwrap();
//         let store = CarBlockStore::new(dir.path().to_path_buf(), 1000);
//         let cid = store.put_block(vec![1,2,3], IpldCodec::Raw).await.unwrap();
//         let block = store.get_block(&cid).unwrap();
//         assert_eq!(block, vec![1,2,3]);
//     }
// }
