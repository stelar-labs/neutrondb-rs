use crate::Store;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl<K,V> Store<K,V> {

    pub fn delete(&mut self, key: &K) -> Result<(), Box<dyn Error>>
    
        where 
        
            K: std::cmp::PartialEq + std::clone::Clone + Into<Vec<u8>>
            
            {
            
                match self.graves.iter().find(|x| x == &key) {
                    
                    Some(_) => Ok(()),
                    
                    None => {

                        let k_bytes: Vec<u8> = key.clone().into();
                        
                        self.graves.push(key.clone());

                        let logs_path_str = format!("{}/logs", &self.directory);
                        
                        let logs_path = Path::new(&logs_path_str); 

                        let logs_append: String = format!("del {}\n", hex::encode(&k_bytes));

                        let mut logs_file = OpenOptions::new()
                            .append(true)
                            .create(true)
                            .open(logs_path)?;

                        write!(logs_file, "{}", &logs_append)?;

                        Ok(())

                    }

                }

            }

}
