
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::Store;
use crate::Table;

use stellar_notation::{ decoding };

pub fn run(name: &str) -> Result<Store, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", name);

    let mut store = Store {
        name: String::from(name),
        cache: vec![],
        cache_buffer: vec![],
        graves: vec![],
        tables: vec![]
    };

    if Path::new(&store_path).is_dir() == false {
        fs::create_dir_all(&store_path)?;
        Ok(store)
    
    } else {

        let cache_path = format!("{}/cache.stellar", &store_path);

        if Path::new(&cache_path).is_file() {
            store.cache_buffer = fs::read(&cache_path)?;
            store.cache = decoding::group(&store.cache_buffer)?;

        }

        let grave_path = format!("{}/graves.stellar", &store_path);

        if Path::new(&grave_path).is_file() {
            let grave_bytes = fs::read(&grave_path)?;
            let grave_group: Vec<(String, String)> = decoding::group(&grave_bytes)?;

            store.graves = grave_group
                .iter()
                .map(|x| x.0.to_string())
                .collect();

        }

        let tables_path = format!("{}/tables.stellar", &store_path);
        
        if Path::new(&tables_path).is_file() {

            let tables_buffer = fs::read(&tables_path)?;

            let tables_group = decoding::group(&tables_buffer)?;

            store.tables = tables_group
                .iter()
                .map(|x| {

                    let value_buffer = decoding::as_bytes(&x.1).unwrap();

                    let value_group = decoding::group(&value_buffer).unwrap();

                    Table {
                        name: x.0.to_string(),
                        level: decoding::as_u128(&value_group[1].1).unwrap() as u8,
                        bloom_filter: decoding::as_bytes(&value_group[0].1).unwrap()
                    }

                })
                .collect();

        }

        println!(" * tables: {}", store.tables.len());

        Ok(store)
    
    }
    
}