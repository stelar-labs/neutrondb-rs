
use astro_notation::encode;
use crate::Store;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl Store {

    pub fn delete(&mut self, key: &str) {
            
        match self.graves.iter().find(|x| x == &key) {
            
            Some(_) => (),
            
            None => {

                self.graves.push(key.to_string());

                let logs_path_str: String = format!("{}/logs", &self.directory);

                let logs_path: &Path = Path::new(&logs_path_str); 

                let logs_put: String = format!("0x02 {}\n", encode::str(key));

                let mut logs_file = OpenOptions::new()
                    .read(true)
                    .append(true)
                    .create(true)
                    .open(logs_path)
                    .unwrap();

                write!(logs_file, "{}", &logs_put).unwrap();

            }

        }

    }
}