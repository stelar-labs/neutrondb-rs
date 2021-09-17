
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::linked_list;
use crate::List;
use crate::Store;

use stellar_notation::{ decode };

pub fn run(name: &str) -> Result<Store, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", name);

    let mut store = Store {
        name: String::from(name),
        cache: vec![],
        cache_buffer: vec![],
        graves: vec![],
        lists: vec![]
    };

    if Path::new(&store_path).is_dir() == false {
        fs::create_dir_all(&store_path)?;
        Ok(store)
    
    } else {

        let cache_path = format!("{}/cache.ndbl", &store_path);

        if Path::new(&cache_path).is_file() {
            store.cache_buffer = fs::read(&cache_path)?;
            store.cache = linked_list::deserialize::list(&store.cache_buffer)?;

        }

        let graves_path = format!("{}/graves.ndbl", &store_path);

        if Path::new(&graves_path).is_file() {
            let graves_buffer: Vec<u8> = fs::read(&graves_path)?;
            let grave_objects: Vec<(String, String)> = linked_list::deserialize::list(&graves_buffer)?;

            store.graves = grave_objects
                .iter()
                .map(|x| x.0.to_string())
                .collect();

        }

        let lists_path = format!("{}/lists.ndbl", &store_path);
        
        if Path::new(&lists_path).is_file() {

            let lists_buffer: Vec<u8> = fs::read(&lists_path)?;

            let lists: Vec<(String, String)> = linked_list::deserialize::list(&lists_buffer)?;

            store.lists = lists
                .iter()
                .map(|x| {

                    let values: Vec<String> = decode::as_list(&x.1).unwrap();

                    println!(" * level: {:?}", decode::as_u8(&values[0]).unwrap());

                    List {
                        name: x.0.to_string(),
                        level: decode::as_u8(&values[0]).unwrap(),
                        bloom_filter: decode::as_bytes(&values[1]).unwrap()
                    }

                })
                .collect();

        }

        Ok(store)
    
    }
    
}