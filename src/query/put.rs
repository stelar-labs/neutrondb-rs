
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::query::{compaction, flush};
use crate::Store;

use stellar_notation::{
    byte_encode,
    value_encode
};

pub fn run(store: &mut Store, object: (String, String)) -> Result<&mut Store, Box<dyn Error>> {

    store.cache.push(object.clone());

    store.cache_buffer = [store.cache_buffer, byte_encode::object(&object.0, &object.1)].concat();

    let store_path = format!("./neutrondb/{}", store.name);

    let cache_path = format!("{}/cache.stellar", &store_path);

    fs::write(&cache_path, &store.cache_buffer)?;

    if store.cache_buffer.len() > 2097152 {

        flush::run(store.clone())?;

        fs::remove_file(&cache_path)?;

        store.cache.clear();

        compaction::run(store.clone())?;

        // reload grave and tables

    }

    let insert_key = object.0;

    let grave_query = store.grave.iter()
        .find(|&x| x == &insert_key);

    match grave_query {
        
        Some(_) => {
            
            store.grave.retain(|x| x != &insert_key);

            let grave_path = format!("{}/grave.stellar", &store_path);

            if store.grave.is_empty() {
                if Path::new(&grave_path).is_file() {
                    fs::remove_file(grave_path)?;

                }

            } else {

                let grave_group: Vec<(String, String)> = store.grave
                    .iter()
                    .map(|x| (x.to_string(), value_encode::u128(&0)))
                    .collect();

                let grave_buffer = byte_encode::group(grave_group);

                fs::write(&grave_path, &grave_buffer)?;
            
            }
        
        },

        None => ()

    }

    Ok(store)
    
}