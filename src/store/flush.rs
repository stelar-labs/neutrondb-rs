use astro_format::string;
use crate::bloom::Bloom;
use crate::{neutron, Store, Table};
use opis::Int;
use std::error::Error;
use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

impl Store {

    pub fn flush(&mut self) -> Result<(), Box<dyn Error>> {

        let level_1_path = format!(
            "{}/levels/1",
            self.directory_location
        );
        
        fs::create_dir_all(&level_1_path)?;

        let current_time = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_millis();

        let table_path = format!(
            "{}/{}",
            &level_1_path,
            &current_time
        );
        
        let table_location = Path::new(&table_path);

        let bloom = self.cache
            .iter()
            .fold(
                Bloom::new(self.cache.len()),
                |acc, x|
                { acc.insert(&x.0) }
            );

        let table_buffer = neutron::create(
            Int { magnitude: bloom.bits.clone(), sign: false }.to_bytes(),
            self.cache.clone()
        );

        fs::write(table_location, &table_buffer)?;

        let table = Table {
            bloom: bloom,
            level: 1_u8,
            name: format!("{}", current_time),
            size: table_location.metadata()?.len()
        };

        self.tables.push(table);

        let graves_path = format!(
            "{}/graves",
            &self.directory_location
        );

        let graves_location = Path::new(&graves_path);

        if graves_location.is_file() {

            fs::remove_file(graves_location)?;

            let graves_input = self.graves
                .iter()
                .fold(
                    String::new(),
                    |acc, x|
                    format!(
                        "{}{}\n",
                        acc,
                        string::encode::bytes(x.as_bytes())
                    )
                );

            fs::write(graves_location, graves_input)?;

        };

        Ok(())

    }

}
