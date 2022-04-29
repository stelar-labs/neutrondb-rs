use astro_format::string;
use crate::Store;
use std::error::Error;
use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

impl Store {

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {

        self.cache.insert(key.to_string(), value.to_string());

        let logs_put: String = format!(
            "put {} {}\n",
            string::encode::bytes(&key.to_string().into_bytes()),
            string::encode::bytes(&value.to_string().into_bytes())
        );

        let logs_path_str = format!("{}/logs", &self.directory);

        let logs_path = Path::new(&logs_path_str);

        let mut logs_file = OpenOptions::new()
            .append(true)
            .create(true)
            .open(logs_path)?;

        write!(logs_file, "{}", &logs_put)?;

        if logs_file.metadata()?.len() > 3200000 {

            self.flush()?;
        
            fs::remove_file(logs_path)?;

            self.cache.clear();

            self.compaction()?;

        }

        self.graves.retain(|x| x != key);

        Ok(())

    }
}