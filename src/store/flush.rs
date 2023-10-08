use fides::BloomFilter;
use crate::{Store, Table, TABLE_HEADER_SIZE, KEY_INDEX_SIZE};
use std::collections::HashMap;
use std::error::Error;
use std::{fs, vec};
use std::fs::OpenOptions;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use std::io::{Write, Seek, SeekFrom, Read};

impl<'a,K,V> Store<K,V> {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let level_path = format!("{}/levels/1", self.directory);
        
        fs::create_dir_all(&level_path)?;

        let flush_path_str = format!("{}/{}.bin", &level_path, &current_time);
        
        let flush_path = Path::new(&flush_path_str);

        let mut flush_file = OpenOptions::new().append(true).create(true).open(flush_path)?;
        
        let mut bloom_filter = BloomFilter::new(self.keys.len());

        let mut key_position: u64 = 0;

        let mut value_position: u64 = 0;

        let mut index: Vec<([u8;32], u64, u64, u64)> = Vec::new();

        let mut value_positions: HashMap<[u8;32],u64> = HashMap::new();

        let mut keys_size = 0;

        let mut value_data_order = Vec::new();

        for (key_hash, key_object) in &self.keys {

            keys_size += key_object.key_size as u64;

            bloom_filter.insert(key_hash);

            let value_position = match value_positions.get(&key_object.value_hash) {
                Some(r) => *r,
                None => {
                    match self.values.get(&key_object.value_hash) {
                        Some(value_object) => {
                            value_data_order.push(key_object.value_hash);
                            let current_val_pos = value_position;
                            value_positions.insert(key_object.value_hash, value_position);
                            value_position += value_object.value_size as u64;
                            current_val_pos
                        },
                        None => continue,
                    }
                },
            };

            index.push(
                (*key_hash, key_position, key_object.key_size.try_into()?, value_position)
            );

            key_position += 8 + key_object.key_size as u64;

        }

        let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

        let key_count = self.keys.len() as u64;

        let index_position = TABLE_HEADER_SIZE + bloom_filter_bytes.len() as u64;

        let index_size = key_count * KEY_INDEX_SIZE;

        let keys_position = index_position + index_size;

        let values_position = keys_position + keys_size;
        
        flush_file.write_all(&[1u8])?;

        flush_file.write_all(&key_count.to_le_bytes())?;

        flush_file.write_all(&index_position.to_le_bytes())?;

        flush_file.write_all(&keys_position.to_le_bytes())?;

        flush_file.write_all(&bloom_filter_bytes)?;

        for  (key_hash, key_position, key_size, value_position) in index {
            
            flush_file.write_all(&key_hash)?;

            flush_file.write_all(&(key_position + keys_position).to_le_bytes())?;

            flush_file.write_all(&key_size.to_le_bytes())?;

            flush_file.write_all(&(values_position + value_position).to_le_bytes())?;

        }

        for (_, key_object) in &self.keys {

            let mut key_bytes = vec![0u8; key_object.key_size];

            self.logs_file.seek(SeekFrom::Start(key_object.key_log_position))?;

            self.logs_file.read_exact(&mut key_bytes)?;

            flush_file.write_all(&key_bytes)?;

        }

        for value_hash in value_data_order {

            match self.values.get(&value_hash) {

                Some(value_object) => {

                    let mut value_bytes = vec![0u8; value_object.value_size];

                    self.logs_file.seek(SeekFrom::Start(value_object.value_log_position))?;

                    self.logs_file.read_exact(&mut value_bytes)?;

                    flush_file.write_all(&value_bytes)?;
                
                },

                None => (),

            }

        }

        let table = Table {
            bloom_filter,
            level: 1_u8,
            name: format!("{}", current_time),
            key_count,
            file_size: flush_file.metadata()?.len(),
            index_position,
            keys_position
        };

        self.tables.push(table);

        self.tables.sort_by(|a, b| b.name.cmp(&a.name));

        let graves_path_str = format!("{}/graves.bin", &self.directory);

        let graves_path = Path::new(&graves_path_str);

        if !self.graves.is_empty() {

            let mut graves_file = OpenOptions::new().append(true).create(true).open(graves_path)?;

            graves_file.set_len(0)?;

            for grave in &self.graves {
                graves_file.write_all(grave)?;
            }


        };

        Ok(())

    }

}
