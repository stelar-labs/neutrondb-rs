use crate::Store;
use crate::types::into_bytes::IntoBytes;
use std::error::Error;
use std::io::Write;

impl<'a,K,V> Store<K,V> {

    pub fn delete(&mut self, key: &'a K) -> Result<(), Box<dyn Error>>
    
        where &'a K: IntoBytes
    
    {

        let key_bytes: Vec<u8> = key.into_bytes();

        let key_hash = fides::hash::blake_3(&key_bytes);

        if !self.graves.contains(&key_hash) {
            
            self.logs_file.write_all(&[3u8])?;

            self.logs_file.write_all(&key_hash)?;

            self.graves.insert(key_hash);

        }

        Ok(())

    }

}
