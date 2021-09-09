
use std::error::Error;
use std::fs;

use crate::Store;

use stellar_notation::{
    byte_encode,
    value_encode
};

pub fn run(store: &mut Store, key: &str) -> Result<(), Box<dyn Error>> {

    let grave_query = store.grave.iter()
        .find(|x| x == &key);

    match grave_query {
        
        Some(_) => (),
        
        None => {

            let store_path = format!("./neutrondb/{}", store.name);

            store.cache.retain(|x| x.0 != key);

            if store.cache.is_empty() {
                let cache_path = format!("{}/cache.stellar", &store_path);
                fs::remove_file(&cache_path)?;

            }

            store.grave.push(key.to_string());

            let grave_path = format!("{}/grave.stellar", &store_path);

            let grave_group: Vec<(String, String)> = store.grave.iter()
                .map(|x| (x.to_string(), value_encode::u128(&0)))
                .collect();

            let grave_buffer = byte_encode::group(grave_group);

            fs::write(&grave_path, &grave_buffer)?;

        }

    }

    Ok(())
    
}