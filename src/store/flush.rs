use fides::BloomFilter;
use crate::{Store, Table};
use std::error::Error;
use std::fs::{self, File};
use std::fs::OpenOptions;
use std::io::{Write, BufReader, BufRead};
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl<K,V> Store<K,V> {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>>
    
        where
        
            K: Clone + Into<Vec<u8>>,
            
            V: Clone + Into<Vec<u8>>
        
                {

                    let current_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_millis();

                    let logs_path_str = format!("{}/logs", &self.directory);

                    let logs_path = Path::new(&logs_path_str);

                    let mut logs_file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(logs_path)?;

                    write!(
                        logs_file,
                        "flush {} table creation started",
                        current_time
                    )?;

                    let level_path = format!(
                        "{}/levels/1",
                        self.directory
                    );
                    
                    fs::create_dir_all(&level_path)?;

                    let table_path_str = format!(
                        "{}/{}.neutron",
                        &level_path,
                        &current_time
                    );
                    
                    let table_path = Path::new(&table_path_str);

                    let mut table_file = OpenOptions::new()
                        .append(true)
                        .create(true)
                        .open(table_path)?;

                    write!(
                        table_file,
                        "neutrondb table version 1.0\n\n"
                    )?;
                    
                    let mut bloom_filter = BloomFilter::new(self.cache.len());

                    for (key, value) in &self.cache {
                        
                        let k_bytes: Vec<u8> = key.clone().into();

                        bloom_filter.insert(&k_bytes);

                        let v_bytes: Vec<u8> = value.clone().into();
                        
                        write!(
                            table_file,
                            "{} {}\n",
                            hex::encode(&k_bytes),
                            hex::encode(&v_bytes)
                        )?;

                    }

                    let bloom_filter_bytes: Vec<u8> = (&bloom_filter).into();

                    write!(
                        table_file,
                        "\n{}\n",
                        hex::encode(&bloom_filter_bytes)
                    )?;

                    write!(
                        logs_file,
                        "flush {} table creation finished",
                        current_time
                    )?;

                    let table = Table {
                        bloom_filter,
                        count: self.cache.len() as u64,
                        level: 1_u8,
                        name: format!("{}.neutron", current_time),
                        size: table_file.metadata()?.len()
                    };

                    self.tables.push(table);

                    self.tables.sort_by_key(|k| k.name.clone());

                    self.tables.reverse();

                    let graves_path = format!(
                        "{}/graves",
                        &self.directory
                    );

                    let graves_location = Path::new(&graves_path);

                    if graves_location.is_file() {

                        fs::remove_file(graves_location)?;

                        let graves_input = self.graves
                            .iter()
                            .fold(
                                String::new(),
                                |mut acc, x| {

                                    let g_bytes: Vec<u8> = x.clone().into();

                                    acc = format!("{}{}\n", acc, hex::encode(g_bytes));

                                    acc
                                    
                                }
                            );

                        fs::write(graves_location, graves_input)?;

                    };

                    write!(
                        logs_file,
                        "flush {} graves updated",
                        current_time
                    )?;

                    Ok(())

                }

}
