
use std::error::Error;
use std::fs;
use std::path::Path;

use stellar_notation::{ encode };

use crate::list;
use crate::store::{ compaction, flush };
use crate::Store;

pub fn run(store: &mut Store, key: &str, value: &str) -> Result<(), Box<dyn Error>> {

    store.cache.push((key.to_string(), value.to_string()));

    let store_path = format!("./ndb/{}", store.name);

    let cache_path = format!("{}/cache.ndbl", &store_path);

    println!(" * cache: {:?}", store.cache);

    let serialized_cache: Vec<u8> = list::serialize::list(&store.cache);

    println!(" * serialized_cache: {:?}", serialized_cache);

    fs::write(&cache_path, &serialized_cache)?;

    if serialized_cache.len() > 2097152 {

        flush::run(store)?;

        fs::remove_file(&cache_path)?;

        store.cache.clear();

        compaction::run(store)?;

    }

    let grave_query = store.graves.iter()
        .find(|&x| x == key);

    match grave_query {
        
        Some(_) => {
            
            store.graves.retain(|x| x != key);

            let graves_path = format!("{}/graves.ndbl", &store_path);

            if store.graves.is_empty() {

                if Path::new(&graves_path).is_file() {

                    fs::remove_file(graves_path)?;
                    
                }

            } else {

                let grave_list: Vec<(String, String)> = store.graves
                    .iter()
                    .map(|x| (x.to_string(), encode::u8(&0)))
                    .collect();

                let graves_buffer = list::serialize::list(&grave_list);

                fs::write(&graves_path, &graves_buffer)?;
            
            }
        
        },

        None => ()

    }

    Ok(())
    
}