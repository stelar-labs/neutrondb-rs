use crate::{Store, CacheObject, ValueObject};
use std::error::Error;
use std::hash::Hash;
use std::io::Write;

impl<'a,K,V> Store<K,V> {
    
    pub fn put(&mut self, key: &'a K, value: &'a V) -> Result<(), Box<dyn Error>>
    where
    K: Clone + Ord + TryFrom<Vec<u8>> + Into<Vec<u8>> + Hash + 'a,
    V: Clone + TryFrom<Vec<u8>> + Into<Vec<u8>> + 'a,
    &'a K: Into<Vec<u8>>,
    &'a V: Into<Vec<u8>>
    {

        let key_bytes: Vec<u8> = key.into();

        let value_bytes: Vec<u8> = value.into();
        
        let kv_astro = astro_format::encode(&[&key_bytes[..], &value_bytes[..]][..]);

        self.logs_file.write_all(&[0u8])?;

        self.logs_file.write_all(&kv_astro)?;

        let key_hash = fides::hash::blake_3(&key_bytes);

        let value_hash = fides::hash::blake_3(&value_bytes);

        // write key indicator
        // write key hash
        // write key size 
        
        let cache_object = CacheObject {
            key_hash,
            value_hash,
            key_size: key_bytes.len(),
            log_position: self.logs_file.metadata()?.len(),
        };

        self.cache.insert(key.clone(), cache_object);

        if !self.values.contains_key(&value_hash) {

            // write value indicator
            // write value hash
            // write value size

            let value_object = ValueObject {
                value: value.clone(),
                value_size: value_bytes.len(),
                log_position: self.logs_file.metadata()?.len()
            };

            self.values.insert(value_hash, value_object);
        }

        if self.logs_file.metadata()?.len() > self.cache_size {

            self.flush()?;

            self.logs_file.set_len(0)?;

            self.cache.clear();

            self.compaction()?;

        }

        self.graves.retain(|&x| x != key_hash);

        Ok(())

    }

}
