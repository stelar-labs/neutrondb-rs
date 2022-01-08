
use astro_notation::decode;

use crate::Metadata;
use crate::Store;

use std::error::Error;
use std::fs::File;
use std::fs::OpenOptions;
use std::fs;
use std::io::BufReader;
use std::io::BufRead;
use std::path::Path;
use std::str;

pub fn run(name: &str) -> Result<Store, Box<dyn Error>> {

    let store_path = format!("./ndb/{}", name);

    fs::create_dir_all(&store_path)?;

    let logs_path_str: String = format!("{}/logs", &store_path);

    let logs_path: &Path = Path::new(&logs_path_str);

    let logs_file: File = OpenOptions::new().read(true).append(true).create(true).open(logs_path)?;

    let mut store = Store {
        name: String::from(name),
        logs_file: logs_file,
        cache: vec![],
        cache_size: 0,
        graves: vec![],
        lists: vec![]
    };

    let graves_path = format!("{}/graves", &store_path);

    if Path::new(&graves_path).is_file() {
        
        let graves_file = File::open(graves_path)?;

        let graves_buffer = BufReader::new(graves_file);

        for line in graves_buffer.lines() {

            let line = line?;

            store.graves.push(decode::as_str(&line)?)

        }

    }

    if logs_path.is_file() {
        
        let cache_file = File::open(logs_path).unwrap();

        let cache_buffer = BufReader::new(cache_file);

        for line in cache_buffer.lines() {

            let line = line?;
            
            let split_line: Vec<&str> = line.split(' ').collect();

            let log_type: u8 = decode::as_u8(split_line[0])?;

            match log_type {

                1 => {

                    let key_buffer: Vec<u8> = decode::as_bytes(split_line[1])?;

                    store.cache_size += key_buffer.len() as u64;

                    let key: String = str::from_utf8(&key_buffer)?.to_string();

                    let value_buffer: Vec<u8> = decode::as_bytes(split_line[2])?;

                    store.cache_size += value_buffer.len() as u64;

                    let value: String = str::from_utf8(&value_buffer)?.to_string();

                    store.cache.push((key.to_owned(), value));

                    let grave_query = store.graves
                        .iter()
                        .find(|&x| x == &key);

                    match grave_query {
                        
                        Some(_) => store.graves.retain(|x| x != &key),

                        None => ()

                    }

                },

                2 => {

                    let grave: String = decode::as_str(split_line[1])?;

                    store.graves.push(grave)

                },

                _ => ()

            }

        }

    }

    let meta_path = format!("{}/meta", &store_path);
    
    if Path::new(&meta_path).is_file() {

        let meta_file = File::open(meta_path).unwrap();

        let meta_buffer = BufReader::new(meta_file);

        for line in meta_buffer.lines() {

            let line = line?;

            let values: Vec<&str> = line.split(' ').collect();

            let name: String = decode::as_str(values[0])?;

            let level: u8 = decode::as_u8(values[1])?;

            let size: u64 = decode::as_u64(values[2])?;

            let bloom_filter: Vec<u8> = decode::as_bytes(values[3])?;

            let metadata: Metadata = Metadata {
                name: name,
                level: level,
                size: size,
                bloom_filter: bloom_filter
            };

            store.lists.push(metadata)

        }

    }

    Ok(store)
    
}
