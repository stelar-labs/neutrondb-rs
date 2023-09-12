use crate::{Store, KeyObject, ValueObject};
use std::error::Error;
use std::io::Write;

impl<'a,K,V> Store<K,V> {
    
    pub fn put(&mut self, key: &'a K, value: &'a V) -> Result<(), Box<dyn Error>>
        
        where
            V: Clone + TryFrom<Vec<u8>, Error = Box<dyn Error>>,
            &'a K: Into<Vec<u8>>,
            &'a V: Into<Vec<u8>>

    {

        let key_bytes: Vec<u8> = key.into();

        let value_bytes: Vec<u8> = value.into();
       
        let key_hash = fides::hash::blake_3(&key_bytes);

        let value_hash = fides::hash::blake_3(&value_bytes);
        
        self.logs_file.write_all(&[1u8])?;

        self.logs_file.write_all(&key_hash)?;

        let key_size_u64 = key_bytes.len() as u64;
        
        self.logs_file.write_all(&key_size_u64.to_be_bytes())?;
                
        let cache_object = KeyObject {
            value_hash,
            key_size: key_bytes.len(),
            key_log_position: self.logs_file.metadata()?.len(),
        };

        self.cache.insert(key_hash, cache_object);

        if !self.values.contains_key(&value_hash) {

            self.logs_file.write_all(&[2u8])?;

            self.logs_file.write_all(&value_hash)?;

            let value_object = ValueObject {
                value: value.clone(),
                value_size: value_bytes.len(),
                value_log_position: self.logs_file.metadata()?.len()
            };

            let value_size_u64 = value_bytes.len() as u64;

            self.logs_file.write_all(&value_size_u64.to_be_bytes())?;

            self.values.insert(value_hash, value_object);
        }

        if self.cache_size > self.cache_limit {

            self.flush()?;

            self.logs_file.set_len(0)?;

            self.cache.clear();

            self.compaction()?;

        }

        self.graves.remove(&key_hash);

        Ok(())

    }

}
