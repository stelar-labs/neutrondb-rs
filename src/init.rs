
use std::error::Error;
use std::fs;
use std::path::Path;

use crate::Store;
use crate::Table;

use stellar_notation::{
    byte_decode,
    value_decode
};

pub fn run(name: &str) -> Result<Store, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", name);

    let mut store = Store {
        name: String::from(name),
        cache: vec![vec![]],
        cache_buffer: vec![],
        grave: vec![],
        tables: vec![]
    };

    if Path::new(&store_path).is_dir() == false {
        fs::create_dir_all(&store_path)?;
        Ok(store)
    
    } else {

        let cache_path = format!("{}/cache.stellar", &store_path);

        if Path::new(&cache_path).is_file() {
            store.cache_buffer = fs::read(&cache_path)?;
            store.cache = byte_decode::group(&store.cache_buffer)?;

        }

        let grave_path = format!("{}/grave.stellar", &store_path);

        if Path::new(&grave_path).is_file() {
            let grave_bytes = fs::read(&grave_path)?;
            let grave_group: Vec<(String, String)> = byte_decode::group(&grave_bytes)?;

            store.grave = grave_group
                .iter()
                .map(|x| x.0.to_string())
                .collect();

        }

        let tables_path = format!("{}/tables.stellar", &store_path);

        if Path::new(&tables_path).is_file() {

            let blooms_path = format!("{}/blooms.stellar", &store_path);

            let blooms_buffer = fs::read(&blooms_path)?;

            let blooms_group = byte_decode::group(&blooms_buffer)?;

            let tables_buffer = fs::read(&tables_path)?;

            let tables_group = byte_decode::group(&tables_buffer)?;

            tables_group
                .iter()
                .for_each(|x| {

                    let table_name: String = x.0.to_string();

                    let bloom_query = blooms_group
                        .iter()
                        .find(|y| x.0 == y.0);

                    match bloom_query {
                        
                        Some(res) => {

                            let level: u128 = value_decode::as_u128(&x.1).unwrap();

                            let bloom: Vec<u8> = value_decode::as_bytes(&res.1).unwrap();

                            let table = Table(table_name, level as u8, bloom);

                            store.tables.push(table);
                            
                        },

                        None => ()
                    }

                })

        }

        Ok(store)
    
    }
    
}