
use std::error::Error;
use std::fs;

use crate::query::bloom_filter;
use crate::Store;

use stellar_notation::{
    byte_decode
};

pub fn run(store: &Store, key: &str) -> Result<Option<String>, Box<dyn Error>> {

    let mut result: Option<String> = None;

    let grave_query = store.grave.iter()
        .find(|x| x == &key);

    match grave_query {

        Some(_) => (),

        None => {

            let mut cache = store.cache.clone();
            
            cache.reverse();

            let cache_query = cache.iter()
                .find(|x| &x.0 == &key);

            match cache_query {

                Some(res) => result = Some(res.1.to_owned()),

                None => {

                    let store_path = format!("./neutrondb/{}", store.name);
                    
                    for table in &store.tables {

                        if bloom_filter::lookup(&table.2, &key) {

                            let sorted_path = format!("{}/level_{}/{}.stellar", &store_path, table.1, table.0);

                            let sorted_buffer = fs::read(&sorted_path)?;

                            let sorted_group = byte_decode::group(&sorted_buffer)?;

                            let sorted_query = sorted_group.iter()
                                .find(|x| x.0 == key);

                            match sorted_query {

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