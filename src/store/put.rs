use hex;
use crate::Store;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl<K,V> Store<K,V> {

    pub fn put(&mut self, key: &K, value: &V) -> Result<(), Box<dyn Error>>
    
        where
        
            K: Clone + std::cmp::Ord + From<Vec<u8>> + Into<Vec<u8>> + std::fmt::Debug,

            V: Clone + From<Vec<u8>> + Into<Vec<u8>> + std::fmt::Debug + std::cmp::Ord

            {

                let k_bytes: Vec<u8> = key.clone().into();

                let v_bytes: Vec<u8> = value.clone().into();

                self.cache.insert(key.clone(), value.clone());

                let logs_put: String = format!("put {} {}\n", hex::encode(&k_bytes), hex::encode(&v_bytes));

                let logs_path_str = format!("{}/logs", &self.directory);

                let logs_path = Path::new(&logs_path_str);

                let mut logs_file = OpenOptions::new().append(true).create(true).open(logs_path)?;

                write!(logs_file, "{}", &logs_put)?;

                if logs_file.metadata()?.len() > 1000000 {

                    self.flush()?;
                
                    fs::remove_file(logs_path)?;

                    self.cache.clear();

                    self.compaction()?;

                    

                }

                self.graves.retain(|x| x != key);

                Ok(())

            }
}