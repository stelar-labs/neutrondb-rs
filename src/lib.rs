
use std::fs;
use std::path::Path;
use std::error::Error;
use std::time::{SystemTime, UNIX_EPOCH};

use stellar_notation::StellarObject;

#[derive(Debug)]
pub struct Store {
    name: String,
    cache: Vec<StellarObject>,
    cache_buffer: Vec<Vec<u8>>,
    pub cache_size: u64,
    grave: Vec<StellarObject>,
    grave_buffer: Vec<Vec<u8>>,
    manifest: Vec<u8>
}

impl Store {

    pub fn insert(&mut self, object: StellarObject) -> Result<(), Box<dyn Error>> {

        let store_path = format!("./neutrondb/{}", self.name);

        let serialized_object = stellar_notation::serialize(object.clone());

        let object_size = serialized_object.len() as u64;

        self.cache_buffer.push(serialized_object);

        self.cache_size += object_size;

        let cache_path = format!("{}/cache.stellar", &store_path);

        let encoded_cache = stellar_notation::encode(&self.cache_buffer);

        fs::write(&cache_path, &encoded_cache)?;

        self.cache.push(object.clone());

        if self.cache_size > 2097152 {

            let mut current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

            let new_l1_path = format!("{}/l1/{}.stellar", &store_path, &current_time);

            fs::write(&new_l1_path, &encoded_cache)?;

            fs::remove_file(&cache_path)?;

            self.cache.clear();
            self.cache_buffer.clear();
            self.cache_size = 0;

            for level in 1..=4 {

                let dir_path = format!("{}/l{}", &store_path, &level);

                if Path::new(&dir_path).is_dir() {

                    let mut dir_files = Vec::new();

                    for file in fs::read_dir(&dir_path)? {
                        let file = file?;
                        let file_path = file.path();
                        if file_path.is_file() {
                            dir_files.push(file_path)
                        }
                    }

                    if dir_files.len() == 5 {
                        
                        dir_files.sort();
                        dir_files.reverse();

                        let encoded_files: Vec<Vec<u8>> = dir_files.iter()
                            .map(|x| fs::read(x).unwrap())
                            .collect();

                        let decoded_files: Vec<Vec<Vec<u8>>> = encoded_files.iter()
                            .map(|x| stellar_notation::decode(x))
                            .collect();

                        let new_objects_uncoded = decoded_files.concat();
                        
                        let new_objects_deserialized: Vec<StellarObject> = new_objects_uncoded.iter()
                            .map(|x| stellar_notation::deserialize(x).unwrap())
                            .collect();

                        let mut compaction_keys: Vec<String> = new_objects_deserialized.iter()
                            .map(|x| x.0.to_owned())
                            .collect();

                        compaction_keys.sort();
                        compaction_keys.dedup();

                        let mut compaction_objects = Vec::new();
    
                        for x in compaction_keys {
                            let query = new_objects_deserialized.iter().find(|&y| y.0 == x);
                            match query {
                                Some(r) => compaction_objects.push(r),
                                None => ()
                            }
                        }

                        let compaction_serialization: Vec<Vec<u8>> = compaction_objects.iter()
                            .map(|&x| stellar_notation::serialize(x.clone()))
                            .collect();

                        let compaction_encoding = stellar_notation::encode(&compaction_serialization);

                        current_time = SystemTime::now().duration_since(UNIX_EPOCH)?.as_millis();

                        let next_level_path = format!("{}/l{}", &store_path, &level + 1);

                        fs::create_dir_all(&next_level_path)?;

                        let compaction_path = format!("{}/{}.stellar", &next_level_path, &current_time);

                        fs::write(&compaction_path, &compaction_encoding)?;
                        
                    }

                }

            }

        }

        let insert_key = object.0;

        let grave_query = self.grave.iter()
            .find(|x| x.0 == insert_key);

        match grave_query {
            Some(_) => self.grave.retain(|x| x.0 != insert_key),
            None => ()
        }

        Ok(())
        
    }

    // pub fn get(&self, key: &str) -> Vec<u8> {

    //     // query cache
    //         // if found return
    //         // else query files
    //             // if found return
    //             // else not found 

    //     return vec![]

    // }

    // pub fn delete(&self, key: &str) {

    //     // stellar remove from self.cache

    //     // add key to grave
        
    // }

}


pub fn store(name: &str) -> Result<Store, Box<dyn Error>> {

    let store_path = format!("./neutrondb/{}", name);

    fs::create_dir_all(&store_path)?;

    let cache_path = format!("{}/cache.stellar", &store_path);

    let mut cache: Vec<StellarObject> = Vec::new();
    let mut cache_buffer: Vec<Vec<u8>> = Vec::new();
    let mut cache_size: u64 = 0;

    if Path::new(&cache_path).is_file() {
        
        let encoded_cache = fs::read(&cache_path)?;
        
        cache_buffer = stellar_notation::decode(&encoded_cache);

        cache_size = cache_buffer.iter()
            .map(|x| x.len() as u64)
            .sum();

        cache = cache_buffer.iter()
            .map(|x| stellar_notation::deserialize(x).unwrap())
            .collect();

    }

    let grave_path = format!(".neutrondb/{}/grave.stellar", name);
    
    let mut grave: Vec<StellarObject> = Vec::new();
    let mut grave_buffer: Vec<Vec<u8>> = Vec::new();

    if Path::new(&grave_path).is_file() {

        let encoded_grave = fs::read(&grave_path)?;

        grave_buffer = stellar_notation::decode(&encoded_grave);

        grave = grave_buffer.iter().map(|x| stellar_notation::deserialize(x).unwrap()).collect();

    }

    let manifest_path = format!(".neutrondb/manifest.stellar");
    let mut manifest: Vec<u8> =  Vec::new();

    if Path::new(&manifest_path).is_file() {
        manifest = fs::read(&manifest_path)?;
    }

    let store = Store {
        name: String::from(name),
        cache: cache,
        cache_buffer: cache_buffer,
        cache_size: cache_size,
        grave: grave,
        grave_buffer: grave_buffer,
        manifest: manifest
    };

    return Ok(store)
    
}
