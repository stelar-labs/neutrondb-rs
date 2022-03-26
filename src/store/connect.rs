
use astro_notation::decode;
use crate::{Store, Table};
use std::fs::File;
use std::fs;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str;

impl Store {

    pub fn connect(name: &str) -> Store {

        let directory = format!("./neutrondb/{}", name);

        let path = Path::new(&directory);

        fs::create_dir_all(&path).unwrap();

        let mut store: Store = Store {
            directory: directory.clone(),
            cache: Vec::new(),
            graves: Vec::new(),
            meta: Vec::new()
        };
        
        if Path::new(&format!("{}/graves", &directory)).is_file() {
            for line in BufReader::new(File::open(format!("{}/graves", &directory)).unwrap()).lines() {
                store.graves.push(decode::as_str(&line.unwrap()))
            }
        }

        if Path::new(&format!("{}/meta", &directory)).is_file() {
            
            for line in BufReader::new(File::open(format!("{}/meta", &directory)).unwrap()).lines() {

                let line = line.unwrap();

                let split: Vec<&str> = line.split(' ').collect();
                
                let table: Table = Table {
                    name: decode::as_str(split[0]),
                    level: decode::as_u8(split[1]),
                    bloom_filter: decode::as_bytes(split[2])
                };
    
                store.meta.push(table)
    
            }
    
        }

        if Path::new(&format!("{}/logs", &directory)).is_file() {
            
            for line in BufReader::new(File::open(format!("{}/logs", &directory)).unwrap()).lines() {

                let line = line.unwrap();

                let split: Vec<&str> = line.split(' ').collect();
    
                let log_type: u8 = decode::as_u8(split[0]);
    
                match log_type {
                    
                    1 => {
                        
                        let key: String = str::from_utf8(&decode::as_bytes(split[1])).unwrap().to_string();
                        let value: String = str::from_utf8(&decode::as_bytes(split[2])).unwrap().to_string();
    
                        store.cache.push((key.clone(), value));
                        
                        match store.graves.iter().find(|&x| x == &key) {
                            Some(_) => store.graves.retain(|x| x != &key),
                            None => ()
                        }
    
                    },

                    2 => store.graves.push(decode::as_str(split[1])),
                    
                    _ => ()
    
                }
            }
        }

        store

    }
}