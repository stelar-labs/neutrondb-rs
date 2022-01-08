
use astro_notation::encode;
use crate::store::{ compaction, flush };
use crate::Store;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn run(store: &mut Store, key: &str, value: &str) -> Result<(), Box<dyn Error>> {

    store.cache.push((key.to_string(), value.to_string()));

    let key_buffer: Vec<u8> = key.to_string().into_bytes();

    store.cache_size += key_buffer.len() as u64;

    let value_buffer: Vec<u8> = value.to_string().into_bytes();

    store.cache_size += value_buffer.len() as u64;

    store.cache_size += 14;

    let logs_put: String = format!("0x01 {} {}\n", encode::bytes(&key_buffer), encode::bytes(&value_buffer));

    write!(store.logs_file, "{}", &logs_put)?;

    if store.cache_size > 2097152 {

        flush::run(store)?;

        let store_path: String = format!("./ndb/{}", store.name);

        let logs_path_str: String = format!("{}/logs", &store_path);

        let logs_path: &Path = Path::new(&logs_path_str);

        fs::remove_file(&logs_path)?;

        store.logs_file = OpenOptions::new().read(true).append(true).create(true).open(logs_path)?;

        store.cache.clear();
        
        store.cache_size = 0;

        compaction::run(store)?;

    }

    let grave_query = store.graves
        .iter()
        .find(|&x| x == key);

    match grave_query {
        
        Some(_) => store.graves.retain(|x| x != key),

        None => ()

    }

    Ok(())
    
}