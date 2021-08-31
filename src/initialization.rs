
use std::error::Error;

use std::fs;
use std::path::Path;

use stellar_notation::{
    StellarObject,
    StellarValue,
    bytes::decode
};

use crate::Table;

pub fn store(name: &str) -> Result<(), Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", name);

    if Path::new(&store_path).is_dir() == false {
        fs::create_dir_all(&store_path)?;
    }

    Ok(())

}

pub fn cache(name: &str) -> Vec<StellarObject> {

    let store_path = format!("./neutrondb/{}", name);

    let cache_path = format!("{}/cache.stellar", &store_path);

    let mut cache: Vec<StellarObject> = Vec::new();

    if Path::new(&cache_path).is_file() {
        
        let cache_bytes = fs::read(&cache_path).unwrap();

        cache = decode::list(&cache_bytes);

    }

    return cache

}

pub fn grave(name: &str) -> Vec<String> {

    let store_path = format!("./neutrondb/{}", name);

    let grave_path = format!("{}/grave.stellar", &store_path);

    let mut grave: Vec<String> = Vec::new();

    if Path::new(&grave_path).is_file() {

        let grave_bytes = fs::read(&grave_path).unwrap();

        let grave_objects = decode::list(&grave_bytes);

        grave = grave_objects.iter()
            .map(|x| x.0.to_string())
            .collect();

    }

    return grave

}

pub fn tables(name: &str) ->  Vec<Table> {

    let store_path = format!("./neutrondb/{}", name);

    let table_locations_path = format!("{}/table_locations.stellar", &store_path);

    let mut tables: Vec<Table> =  Vec::new();

    if Path::new(&table_locations_path).is_file() {

        let bloom_filters_path = format!("{}/bloom_filters.stellar", &store_path);

        let bloom_filters = fs::read(&bloom_filters_path).unwrap();

        let bloom_filters_objects = decode::list(&bloom_filters);
        
        let table_locations = fs::read(&table_locations_path).unwrap();

        let table_locations_objects = decode::list(&table_locations);

        table_locations_objects.iter()
            .for_each(|x| {

                let table_name: String = x.0.to_string();

                let bloom_filter_query = bloom_filters_objects.iter()
                    .find(|y| x.0 == y.0);

                match bloom_filter_query {
                    
                    Some(res) => {

                        let mut level: u8 = 0;

                        match x.1 {
                            StellarValue::UInt8(r) => level = r as u8,
                            _ => ()
                        }

                        let mut bloom_filter: Vec<u8> = Vec::new();

                        match &res.1 {
                            StellarValue::Bytes(r) => bloom_filter = r.clone(),
                            _ => ()
                        }

                        let table = Table(table_name, level, bloom_filter);

                        tables.push(table);
                        
                    },

                    None => ()
                }

            })

    }

    return tables

}