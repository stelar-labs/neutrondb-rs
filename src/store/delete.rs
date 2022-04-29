use astro_format::string;
use crate::Store;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl Store {

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
            
        match self.graves.iter().find(|x| x == &key) {
            
            Some(_) => Ok(()),
            
            None => {

                self.graves.push(key.to_string());

                let logs_path_str: String = format!("{}/logs", &self.directory);

                let logs_path: &Path = Path::new(&logs_path_str); 

                let logs_append: String = format!("delete {}\n", string::encode::bytes(&key.to_string().into_bytes()));

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