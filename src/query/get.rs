
use std::error::Error;
use std::fs;

use crate::query::bloom_filter;
use crate::Store;

use stellar_notation::{ decoding };

pub fn run(store: &Store, key: &str) -> Result<Option<String>, Box<dyn Error>> {

    let mut result: Option<String> = None;

    let grave_query = store.graves.iter()
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

                    let mut reversed_tables = store.tables.clone();
                    
                    reversed_tables.reverse();
                    
                    for table in reversed_tables {

                        if bloom_filter::lookup(&table.bloom_filter, &key) {

                            let table_path = format!("{}/level_{}/{}.stellar", &store_path, table.level, table.name);

                            let table_buffer = fs::read(&table_path)?;

                            let table_group = decoding::group(&table_buffer)?;

                            let table_query = table_group.iter()
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