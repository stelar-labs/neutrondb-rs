use crate::types::into_bytes::IntoBytes;
use crate::{Store, KeyObject, ValueObject};
use std::error::Error;
use std::io::{Write, Seek, SeekFrom};
use std::mem;

impl<'a,K,V> Store<K,V> {
    
    pub fn put(&mut self, key: &'a K, value: &'a V) -> Result<(), Box<dyn Error>>
        
        where
            V: Clone,
            &'a K: IntoBytes,
            &'a V: IntoBytes

    {

        let key_bytes: Vec<u8> = key.into_bytes();

        let value_bytes: Vec<u8> = value.into_bytes();
       
        let key_hash = fides::hash::blake_3(&key_bytes);

        let value_hash = fides::hash::blake_3(&value_bytes);
        
        self.logs_file.write_all(&[1u8])?;

        self.logs_file.write_all(&key_hash)?;

        self.logs_file.write_all(&value_hash)?;

        let key_size_u64 = key_bytes.len() as u64;

        self.logs_file.write_all(&key_size_u64.to_le_bytes())?;
                    
        let key_object = KeyObject {
            value_hash,
            key_size: key_bytes.len(),
            key_log_position: self.logs_file.metadata()?.len(),
        };

        self.logs_file.write_all(&key_bytes)?;

        // if not in keys increase cache size

        self.cache_size += 32 + mem::size_of_val(&key_object) as u64;

        self.keys.insert(key_hash, key_object);

        if !self.values.contains_key(&value_hash) {

            self.logs_file.write_all(&[2u8])?;

            let value_object = ValueObject {
                value: value.clone(),
                value_size: 32 + 8 + value_bytes.len(),
                value_log_position: self.logs_file.metadata()?.len()
            };

            self.logs_file.write_all(&value_hash)?;

            let value_size_u64 = value_bytes.len() as u64;

            self.logs_file.write_all(&value_size_u64.to_le_bytes())?;

            self.logs_file.write_all(&value_bytes)?;

            self.cache_size += 32 + mem::size_of_val(&value_object) as u64;

            self.values.insert(value_hash, value_object);

        }

        if self.cache_size > self.cache_limit {

            self.flush()?;

            self.logs_file.set_len(0)?;

            self.logs_file.flush()?;

            self.logs_file.seek(SeekFrom::Start(0))?;

            self.keys.clear();

            self.values.clear();

            self.cache_size = 0;

            self.compaction()?;

        }

        self.graves.remove(&key_hash);

        Ok(())

    }

}
