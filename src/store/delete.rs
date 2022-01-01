
use std::error::Error;
use std::fs;

use crate::list;
use crate::Store;

use stellar_notation::{ encode };

pub fn run(store: &mut Store, key: &str) -> Result<(), Box<dyn Error>> {

    let grave_query = store.graves
        .iter()
        .find(|x| x == &key);

    match grave_query {
        
        Some(_) => (),
        
        None => {

            let store_path = format!("./ndb/{}", store.name);

            store.cache.retain(|x| x.0 != key);

            if store.cache.is_empty() {

                let cache_path = format!("{}/cache.ndbl", &store_path);
                
                fs::remove_file(&cache_path)?;

            }

            store.graves.push(key.to_string());

            let graves_path = format!("{}/graves.ndbl", &store_path);

            let grave_list: Vec<(String, String)> = store.graves
                .iter()
                .map(|x| (x.to_string(), encode::u8(&0)))
                .collect();

            let graves_buffer = list::serialize::list(&grave_list);

            fs::write(&graves_path, &graves_buffer)?;

        }

    }

    Ok(())
    
}