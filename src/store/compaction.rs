use fides::BloomFilter;
use crate::{Table, Store};
use std::collections::{HashMap, BTreeMap};
use std::error::Error;
use std::fs::{self, File, OpenOptions};
use std::io::{SeekFrom, Seek, Read, Write};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl<K,V> Store<K,V> {
    
    pub fn compaction(&mut self) -> Result<(), Box<dyn Error>> {

        for level in 1..=10 {

            let tables: Vec<&Table> = self.tables.iter().filter(|x| x.level == level).collect();

            let level_size = tables.iter().fold(0, |acc, x| acc + x.file_size);

            if level_size > (10_u64.pow(level as u32) * 1000000) {

                let mut level_key_position = 0;

                let mut level_value_position = 0;

                // level index -> key_hash: value_hash, key_position, value_position
                let mut level_index:  BTreeMap<[u8;32], ([u8;32], u64, u64)> = BTreeMap::new();

                // file index, file position, key size
                let mut key_data_locations: Vec<(usize, u64, usize)> = Vec::new();

                let mut value_data_locations: Vec<(usize, u64, usize)> = Vec::new();

                let mut table_files: Vec<File> = Vec::new();

                for table in &tables {

                    let level_path = format!("{}/levels/{}", self.directory, table.level);

                    let table_path_str = format!("{}/{}.bin", &level_path, &table.name);
        
                    let table_path = Path::new(&table_path_str);

                    let table_file = OpenOptions::new().append(true).create(true).open(table_path)?;

                    table_files.push(table_file)

                }

                let mut level_value_positions: HashMap<[u8;32], u64> = HashMap::new();
                
                let mut value_data_offset = 0;

                for (i, table) in tables.iter().enumerate() {

                    match table_files.get_mut(i) {
                        Some(table_file) => {

                            while table_file.seek(SeekFrom::Current(0))? < table.key_data_position {
                                
                                let mut key_bytes = [0u8;32];

                                table_file.read_exact(&mut key_bytes)?;
                                
                                if !level_index.contains_key(&key_bytes) {

                                    let mut value_hash = [0u8;32];
                                    table_file.read_exact(&mut value_hash)?;

                                    let mut table_key_position_buffer = [0u8;8];
                                    table_file.read_exact(&mut table_key_position_buffer)?;
                                    let table_key_position = u64::from_be_bytes(table_key_position_buffer);

                                    let current_level_value_position = match level_value_positions.get(&value_hash) {
                                        Some(r) => *r,
                                        None => {

                                            let mut table_value_position_buffer = [0u8;8];
                                            table_file.read_exact(&mut table_value_position_buffer)?;
                                            let table_value_position = u64::from_be_bytes(table_value_position_buffer);

                                            // save seek position here

                                            table_file.seek(SeekFrom::Start(table_value_position))?;

                                            let mut table_value_size_buffer = [0u8;8];
                                            table_file.read_exact(&mut table_value_size_buffer)?;
                                            let table_value_size = u64::from_be_bytes(table_value_size_buffer);

                                            value_data_locations.push((
                                                i,
                                                table_value_position + 8,
                                                table_value_size as usize
                                            ));

                                            level_value_positions.insert(value_hash, level_value_position);

                                            let current_level_value_position = level_value_position;

                                            level_value_position += 8 + table_value_size;

                                            current_level_value_position

                                        },
                                    };

                                    
                                    table_file.seek(SeekFrom::Start(table_key_position))?;

                                    let table_key_size_buffer = [0u8;8];
                                    table_file.read_exact(&mut table_key_position_buffer)?;
                                    let table_key_size = u64::from_be_bytes(table_key_size_buffer);

                                    value_data_offset += 8 + table_key_size;

                                    level_index.insert(
                                        key_bytes, 
                                        (value_hash, level_key_position, current_level_value_position)
                                    );

                                    key_data_locations.push((
                                        i,
                                        table_key_position + 8,
                                        table_key_size as usize
                                    ));

                                    level_key_position += 8 + table_key_size;

                                    // fix table file seek 

                                }

                            }
                        }
                        None => todo!(),
                    }
                }

                let current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                let next_level_path = format!("{}/levels/{}", &self.directory, level + 1);

                fs::create_dir_all(&next_level_path)?;

                let compacted_path_str = format!("{}/{}.neutron", &next_level_path, &current_time);

                let compacted_path = Path::new(&compacted_path_str);

                let mut level_file = OpenOptions::new().append(true).create(true).open(compacted_path)?;   
            
                let mut bloom_filter = BloomFilter::new(level_index.len());

                for (key_hash, _) in &level_index {
                    bloom_filter.insert(key_hash);
                }

                let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

                let key_count: u64 = level_index.len() as u64;

                let index_position = 32 + bloom_filter_bytes.len() as u64;

                let key_data_position = index_position + (key_count * 80);

                value_data_offset += key_data_position;

                level_file.write_all(&[1u8])?;
                
                level_file.write_all(&key_count.to_be_bytes())?;
                level_file.write_all(&index_position.to_be_bytes())?;
                level_file.write_all(&key_data_position.to_be_bytes())?;
                level_file.write_all(&bloom_filter_bytes)?;

                for (key_hash, (value_hash, key_position, value_position)) in level_index {
                    level_file.write_all(&key_hash)?;
                    level_file.write_all(&value_hash)?;
                    level_file.write_all(&(key_position + key_data_position).to_be_bytes())?;
                    level_file.write_all(&(value_position + value_data_offset).to_be_bytes())?;
                }

                for (table_file_index, key_data_position, key_data_size) in key_data_locations {
                    table_files[table_file_index].seek(SeekFrom::Start(key_data_position))?;
                    let mut key_buffer = vec![0u8; key_data_size];
                    table_files[table_file_index].read_exact(&mut key_buffer)?;
                    level_file.write_all(&(key_data_size as u64).to_be_bytes())?;
                    level_file.write_all(&key_buffer)?;
                }

                for (table_file_index, value_data_position, value_data_size) in value_data_locations {
                    table_files[table_file_index].seek(SeekFrom::Start(value_data_position))?;
                    let mut value_buffer = vec![0u8; value_data_size];
                    table_files[table_file_index].read_exact(&mut value_buffer)?;
                    level_file.write_all(&(value_data_size as u64).to_be_bytes())?;
                    level_file.write_all(&value_buffer)?;
                }

                for table in tables {

                    let table_path = format!(
                        "{}/levels/{}/{}",
                        &self.directory,
                        &table.level,
                        &table.name
                    );

                    fs::remove_file(table_path)?;

                }

                self.tables.retain(|x| x.level != level);

                let table = Table {
                    bloom_filter,
                    level: level + 1,
                    name: format!("{}.neutron", current_time),
                    key_count,
                    file_size: level_file.metadata()?.len(),
                    index_position,
                    key_data_position,
                };

                self.tables.push(table);

                self.tables.sort_by(|a, b| b.name.cmp(&a.name));

            }

        }

        Ok(())

    }

}    
