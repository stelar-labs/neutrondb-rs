use std::collections::BTreeMap;
use std::error::Error;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::fs::File;
use astro_format;
use std::str;

pub fn run<K, V>(keys: &[&K], table_path: &str) -> Result<BTreeMap<K, V>, Box<dyn Error>>
    
        where
        
            K: std::cmp::PartialEq + std::cmp::Ord + TryFrom<Vec<u8>> + Clone,

            V: TryFrom<Vec<u8>>,
        
            <K as TryFrom<Vec<u8>>>::Error: std::error::Error,

            <V as TryFrom<Vec<u8>>>::Error: std::error::Error
            
                {

                    let mut file = File::open(table_path)?;
                    
                    let mut bloom_len = [0; 8];

                    file.read_exact(&mut bloom_len)?;

                    let keys_index = u64::from_le_bytes(bloom_len) + 8;

                    file.seek(SeekFrom::Start(keys_index))?;

                    let mut keys_len = [0; 8];

                    file.read_exact(&mut keys_len)?;

                    let keys_buffer_size = usize::from_be_bytes(keys_len);

                    let mut keys_buffer = vec![0; keys_buffer_size];

                    file.read_exact(&mut keys_buffer)?;

                    let keys_bytes = astro_format::decode(&keys_buffer)?;

                    let mut key_indices = Vec::new();

                    for key_index_encoded in keys_bytes {
                        
                        let key_index_bytes = astro_format::decode(key_index_encoded)?;

                        if key_index_bytes.len() == 2 {

                            match K::try_from(key_index_bytes[0].to_vec()) {

                                Ok(k) => {

                                    let index = u64::from_be_bytes(key_index_bytes[1].try_into()?);

                                    key_indices.push((k, index))

                                },

                                _ => ()
                                
                            }

                        }

                    }

                    let mut key_values: BTreeMap<K, V> = BTreeMap::new();

                    for &key in keys {

                        match keys.iter().position(|&x| x == key) {

                            Some(i) => {
                                
                                let start = key_indices[i].1;

                                let end = key_indices[i + 1].1;

                                let value_buffer_size = (end - start) as usize;

                                let value_index = keys_index + keys_buffer_size as u64 + start;

                                file.seek(SeekFrom::Start(value_index))?;

                                let mut value_buffer = vec![0; value_buffer_size];

                                file.read_exact(&mut value_buffer)?;

                                match V::try_from(value_buffer) {

                                    Ok(v) => {key_values.insert(key.clone(), v);},

                                    _ => ()
                                    
                                }

                            },

                            None => ()

                        }

                    }

                    Ok(key_values)

}
