use std::fs::File;
use std::error::Error;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;

use crate::KEY_INDEX_SIZE;
use crate::Store;
use crate::types::into_bytes::IntoBytes;
use crate::types::try_from_bytes::TryFromBytes;

impl<'a, K,V> Store<K,V> {

    pub fn get(&self, key: &'a K) -> Result<V, Box<dyn Error>>
        
        where
            V: Clone + TryFromBytes,
            &'a K: IntoBytes
    
    {

        let key_bytes: Vec<u8> = key.into_bytes();

        let key_hash = fides::hash::blake_3(&key_bytes);
            
        match self.graves.iter().find(|&x| x == &key_hash) {

            Some(_) => Err("Not found!")?,

            None => {
                
                match self.keys.get(&key_hash) {

                    Some(r) => {

                        match self.values.get(&r.value_hash) {
                            Some(value_object) => Ok(value_object.value.clone()),
                            None => Err("Not found!")?,
                        }
                        
                    },

                    None => {

                        let mut res: Option<V> = None;
                        
                        for table in &self.tables {


                            match res {

                                None => {

                                    // match table.bloom_filter.search(&key_bytes) {

                                    match true {
                                        
                                        true => {

                                            let table_path = format!(
                                                "{}/levels/{}/{}.bin",
                                                &self.directory,
                                                table.level,
                                                table.name
                                            );

                                            let mut table_file = File::open(&table_path)?;

                                            let mut low = 0;

                                            let mut high = table.key_count;

                                            while low <= high {
                                                
                                                let mid = low + ((high - low) / 2);

                                                let key_index_position = table.index_position + (mid * KEY_INDEX_SIZE);

                                                let mut table_key_hash = [0u8;32];
                                                table_file.seek(SeekFrom::Start(key_index_position))?;
                                                table_file.read_exact(&mut table_key_hash)?;

                                                if key_hash == table_key_hash {

                                                    table_file.seek(SeekFrom::Current(16))?;

                                                    let mut value_position_bytes = [0u8;8];
                                                    table_file.read_exact(&mut value_position_bytes)?;

                                                    let value_position = u64::from_le_bytes(value_position_bytes);

                                                    table_file.seek(SeekFrom::Start(value_position))?;

                                                    table_file.seek(SeekFrom::Current(32))?;

                                                    let mut value_size_buffer = [0u8;8];
                                                    table_file.read_exact(&mut value_size_buffer)?;

                                                    let value_size = u64::from_le_bytes(value_size_buffer) as usize;
                                                    
                                                    let mut value_buffer = vec![0u8; value_size];
                                                    table_file.read_exact(&mut value_buffer)?;
                                                    
                                                    let value = V::try_from_bytes(value_buffer)?;
                                            
                                                    res = Some(value);
                                                    
                                                    break

                                                } else if key_hash > table_key_hash {
                                                    
                                                    low = mid + 1

                                                } else {
                                                    
                                                    high = mid - 1
                                                }
                                            }
                                        },
                                        false => ()
                                    }
                                },
                                _ => ()               
                            }
                        }
                        match res {
                            Some(r) => Ok(r),
                            None => Err("Not found!")?
                        }
                    }
                }
            }
        }
    }
}
