use crate::{neutron, Store};

impl Store {

    pub fn get(&self, key: &str) -> Option<String> {
            
        match self.graves.iter().find(|&x| x == key) {

            Some(_) => None,

            None => {
                
                match self.cache.get(key) {

                    Some(r) => Some(r.clone()),

                    None => {

                        let mut res: Option<String> = None;
                        
                        for table in &self.tables {

                            match table.bloom.search(key) {
                                
                                true => {

                                    let table_path = format!(
                                        "{}/levels/{}/{}",
                                        &self.directory_location,
                                        table.level,
                                        table.name
                                    );
                                
                                    match neutron::get(key, &table_path) {
                                        
                                        Ok(r) => {
                                            res = Some(r);
                                            break
                                        },
                                        
                                        Err(_) => ()
                                    
                                    }

                                },

                                false => ()

                            }

                        }

                        res
                    }
                }
            }
        }
    }
}
