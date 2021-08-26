
use std::error::Error;

use std::fs;

use crate::flush;
use crate::compaction;
use crate::bloom_filter;

use stellar_notation::StellarObject;
use stellar_notation::StellarValue;

#[derive(Clone, Debug)]
pub struct Table(pub String, pub u8, pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct Store {
    pub name: String,
    pub cache: Vec<StellarObject>,
    pub grave: Vec<String>,
    pub tables: Vec<Table>
}

impl Store {

    pub fn put(&mut self, object: StellarObject) -> Result<(), Box<dyn Error>> {

        let store_path = format!("./neutrondb/{}", self.name);

        self.cache.push(object.clone());

        let cache_path = format!("{}/cache.stellar", &store_path);

        let cache_serialization = self.cache.iter()
            .map(|x| stellar_notation::serialize(x.to_owned()))
            .collect();

        let cache_encoded = stellar_notation::encode(&cache_serialization);

        fs::write(&cache_path, &cache_encoded)?;

        if cache_encoded.len() > 2097152 {

            flush::perform(self.clone())?;

            fs::remove_file(&cache_path)?;

            self.cache.clear();

            compaction::perform(self.clone())?;

            // reload grave and tables

        }

        let insert_key = object.0;

        let grave_query = self.grave.iter()
            .find(|&x| x == &insert_key);

        match grave_query {
            
            Some(_) => {
                
                self.grave.retain(|x| x != &insert_key);

                let grave_objects: Vec<Vec<u8>> = self.grave.iter()
                    .map(|x| StellarObject(x.to_string(), StellarValue::IntegerType(0)))
                    .map(|x| stellar_notation::serialize(x))
                    .collect();

                let grave_bytes = stellar_notation::encode(&grave_objects);

                let grave_path = format!("{}/grave.stellar", &store_path);

                fs::write(&grave_path, &grave_bytes)?;
            
            },

            None => ()

        }

        Ok(())
        
    }

    pub fn get(&self, key: &str) -> Result<Option<StellarObject>, Box<dyn Error>> {

        let mut result: Option<StellarObject> = None;

        let cache_query = self.cache.iter()
            .find(|x| &x.0 == &key);

        match cache_query {

            Some(res) => result = Some(res.to_owned()),

            None => {

                let store_path = format!("./neutrondb/{}", self.name);
                
                for table in &self.tables {

                    if bloom_filter::lookup(&table.2, &key) {

                        let table_path = format!("{}/level_{}/{}.stellar", &store_path, table.1, table.0);

                        let table_bytes = fs::read(&table_path)?;

                        let table_query = stellar_notation::find(&table_bytes, &key);

                        match table_query {

                            Some(res) => {
                                result = Some(res);
                                break
                            },

                            None => ()

                        }
                    }

                }

            }
        }

        return Ok(result)

    }

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {

        self.cache.retain(|x| x.0 != key);

        self.grave.push(key.to_string());

        let grave_objects: Vec<Vec<u8>> = self.grave.iter()
            .map(|x| StellarObject(x.to_string(), StellarValue::IntegerType(0)))
            .map(|x| stellar_notation::serialize(x))
            .collect();

        let grave_bytes = stellar_notation::encode(&grave_objects);

        let grave_path = format!("./neutrondb/{}/grave.stellar", &self.name);

        fs::write(&grave_path, &grave_bytes)?;

        Ok(())
        
    }

}