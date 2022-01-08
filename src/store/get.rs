use crate::list;
use crate::store::bloom_filter;
use crate::Store;
use std::error::Error;
use std::path::Path;

pub fn run(store: &Store, key: &str) -> Result<Option<String>, Box<dyn Error>> {

    let grave_query = store.graves
        .iter()
        .find(|&x| x == key);

    match grave_query {

        Some(_) => Ok(None),

        None => {

            let mut cache = store.cache.clone();
            
            cache.reverse();

            let cache_query = cache
                .iter()
                .find(|x| x.0 == key);

            match cache_query {

                Some(r) => Ok(Some(r.1.to_owned())),

                None => {

                    let store_path = format!("./ndb/{}", store.name);

                    let mut search_res: Option<String> = None;
                    
                    for list in store.lists.clone() {

                        if bloom_filter::lookup(&list.bloom_filter, &key) {

                            let list_path_str: String = format!("{}/level_{}/{}.neutron", &store_path, list.level, list.name);

                            let list_path: &Path = Path::new(&list_path_str);

                            let key_query = list::search::key(list_path, key)?;

                            match key_query {

                                Some(r) => search_res = Some(r),

                                None => ()

                            }

                        }

                    }

                    Ok(search_res)

                }
            }

        }

    }

}
