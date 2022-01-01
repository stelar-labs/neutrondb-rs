
use std::error::Error;
use std::fs;

use crate::list;
use crate::store::bloom_filter;
use crate::Store;

pub fn run(store: &Store, key: &str) -> Result<Option<String>, Box<dyn Error>> {

    let mut result: Option<String> = None;

    let grave_query = store.graves
        .iter()
        .find(|&x| x == key);

    match grave_query {

        Some(_) => (),

        None => {

            let mut cache = store.cache.clone();
            
            cache.reverse();

            let cache_query = cache
                .iter()
                .find(|x| x.0 == key);

            match cache_query {

                Some(res) => result = Some(res.1.to_owned()),

                None => {

                    let store_path = format!("./ndb/{}", store.name);

                    let mut lists = store.lists.clone();
                    
                    lists.reverse();
                    
                    for list in lists {

                        if bloom_filter::lookup(&list.bloom_filter, &key) {

                            let list_path = format!("{}/level_{}/{}.ndbl", &store_path, list.level, list.name);

                            let list_buffer = fs::read(&list_path)?;

                            let list = list::deserialize::list(&list_buffer)?;

                            let list_query = list.iter()
                                .find(|x| x.0 == key);

                            match list_query {

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