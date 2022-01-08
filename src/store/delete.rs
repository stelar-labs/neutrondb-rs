
use astro_notation::encode;
use crate::Store;
use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn run(store: &mut Store, key: &str) -> Result<(), Box<dyn Error>> {

    let grave_query = store.graves
        .iter()
        .find(|x| x == &key);

    match grave_query {
        
        Some(_) => (),
        
        None => {

            store.graves.push(key.to_string());

            let store_path = format!("./ndb/{}", store.name);

            let logs_path_str: String = format!("{}/logs", &store_path);

            let logs_path: &Path = Path::new(&logs_path_str); 

            let logs_put: String = format!("0x02 {}\n", encode::str(key));

            let mut logs_file = OpenOptions::new()
                .read(true)
                .append(true)
                .create(true)
                .open(logs_path)?;

            write!(logs_file, "{}", &logs_put)?;

        }

    }

    Ok(())
    
}