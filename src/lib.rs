
use std::error::Error;
use std::fs;

mod bloom_filter;
mod compaction;
mod initialization;
mod flush;

use stellar_notation::{
    byte_encode,
    byte_decode,
    value_encode
};

use std::path::Path;

#[derive(Clone, Debug)]
pub struct Table(pub String, pub u8, pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct Store {
    pub name: String,
    pub cache: Vec<(String, String)>,
    pub grave: Vec<String>,
    pub tables: Vec<Table>
}

impl Store {

    pub fn put(&mut self, object: (String, String)) -> Result<(), Box<dyn Error>> {

        self.cache.push(object.clone());

        let store_path = format!("./neutrondb/{}", self.name);

        let cache_path = format!("{}/cache.stellar", &store_path);

        let cache_serielized = byte_encode::group(self.cache.clone());

        fs::write(&cache_path, &cache_serielized)?;

        if cache_serielized.len() > 2097152 {

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

                let grave_path = format!("{}/grave.stellar", &store_path);

                if self.grave.is_empty() {

                    if Path::new(&grave_path).is_file() {

                        fs::remove_file(grave_path)?;

                    }

                } else {

                    let grave_group: Vec<(String, String)> = self.grave.iter()
                        .map(|x| (x.to_string(), value_encode::u128(&0)))
                        .collect();

                    let grave_group_bytes = byte_encode::group(grave_group);

                    fs::write(&grave_path, &grave_group_bytes)?;
                
                }
            
            },

            None => ()

        }

        Ok(())
        
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {

        let mut result: Option<String> = None;

        let grave_query = self.grave.iter()
            .find(|x| x == &key);

        match grave_query {

            Some(_) => (),

            None => {

                let mut cache = self.cache.clone();
                
                cache.reverse();

                let cache_query = cache.iter()
                    .find(|x| &x.0 == &key);

                match cache_query {

                    Some(res) => result = Some(res.1.to_owned()),

                    None => {

                        let store_path = format!("./neutrondb/{}", self.name);
                        
                        for table in &self.tables {

                            if bloom_filter::lookup(&table.2, &key) {

                                let table_path = format!("{}/level_{}/{}.stellar", &store_path, table.1, table.0);

                                let table_serialized = fs::read(&table_path)?;

                                let table_deserialized = byte_decode::group(&table_serialized);

                                let table_query = table_deserialized.iter()
                                    .find(|x| x.0 == key);

                                match table_query {

                                    Some(res) => {
                                        result = Some(res.1.to_owned());
                                        break
                                    },

                                    None => ()

                                }
                            }

                        }

                    }
                }

            }

        }

        return Ok(result)

    }

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {

        let grave_query = self.grave.iter()
            .find(|x| x == &key);

        match grave_query {
            
            Some(_) => (),
            
            None => {

                let store_path = format!("./neutrondb/{}", self.name);

                self.cache.retain(|x| x.0 != key);

                if self.cache.is_empty() {

                    let cache_path = format!("{}/cache.stellar", &store_path);
                    fs::remove_file(&cache_path)?;

                }

                self.grave.push(key.to_string());

                let grave_path = format!("{}/grave.stellar", &store_path);

                let grave_group: Vec<(String, String)> = self.grave.iter()
                    .map(|x| (x.to_string(), value_encode::u128(&0)))
                    .collect();

                let grave_group_bytes = byte_encode::group(grave_group);

                fs::write(&grave_path, &grave_group_bytes)?;

            }

        }

        Ok(())
        
    }

    pub fn get_all(&self) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {

        let store_path = format!("./neutrondb/{}", self.name);

        let mut group = self.cache.clone();
        
        group.reverse();

        for table in &self.tables {

            let table_path = format!("{}/level_{}/{}.stellar", &store_path, table.1, table.0);

            let table_bytes = fs::read(&table_path)?;

            let table_objects = byte_decode::group(&table_bytes);

            group = [group, table_objects].concat();

        }

        if group.is_empty() {
            
            Ok(None)
        
        } else {

            group.sort_by_key(|x| x.0.to_string());

            group.dedup_by_key(|x| x.0.to_string());
            
            Ok(Some(group))

        }

    }

}

pub fn store(name: &str) -> Result<Store, Box<dyn Error>> {
    
    initialization::store(&name)?;

    let cache = initialization::cache(&name);

    let grave = initialization::grave(&name);

    let tables = initialization::tables(&name);

    let store = Store {
        name: String::from(name),
        cache: cache,
        grave: grave,
        tables: tables
    };

    return Ok(store)
    
}
