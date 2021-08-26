
use std::error::Error;

use std::fs;
use std::path::Path;

use stellar_notation::StellarObject;
use stellar_notation::StellarValue;

use crate::store::Table;

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

        let cache_objects = stellar_notation::decode(&cache_bytes);

        cache = cache_objects.iter()
            .map(|x| stellar_notation::deserialize(x).unwrap())
            .collect();

    }

    return cache

}

pub fn grave(name: &str) -> Vec<String> {

    let store_path = format!("./neutrondb/{}", name);

    let grave_path = format!("{}/grave.stellar", &store_path);

    let mut grave: Vec<String> = Vec::new();

    if Path::new(&grave_path).is_file() {

        let grave_bytes = fs::read(&grave_path).unwrap();

        let grave_objects = stellar_notation::decode(&grave_bytes);

        grave = grave_objects.iter()
            .map(|x| stellar_notation::deserialize(x).unwrap().0)
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

        let bloom_filters_bytes = fs::read(&bloom_filters_path).unwrap();
        
        let table_locations_bytes = fs::read(&table_locations_path).unwrap();

        let table_locations_objects = stellar_notation::decode(&table_locations_bytes);

        table_locations_objects.iter()
            .map(|x| stellar_notation::deserialize(x).unwrap())
            .for_each(|x| {

                match stellar_notation::find(&bloom_filters_bytes, &x.0) {
                    
                    Some(res) => {

                        let mut level: u8 = 0;

                        match x.1 {
                            StellarValue::IntegerType(r) => level = r as u8,
                            _ => ()
                        }

                        let mut bloom_filter: Vec<u8> = Vec::new();

                        match res.1 {
                            StellarValue::ByteType(r) => bloom_filter = r,
                            _ => ()
                        }

                        let table = Table(x.0, level, bloom_filter);
                        tables.push(table);
                        
                    },

                    None => ()
                }

            })

    }

    return tables

}