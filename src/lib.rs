
pub struct Store {
    pub name: String,
    pub cache: Vec<Vec<u8>>
}

impl Store {

    fn insert(&mut self, key_value: Vec<u8>) {
        
        // stellar push to self.cache
        
        // if self.cache is larger than 2MB
            // create level 1 file
            // delete log
            // clear cache
    }

    fn get(&self, key: &str) -> Vec<u8> {

        // query cache
            // if found return
            // else query files
                // if found return
                // else not found 

        return vec![]

    }

    fn delete(&self, key: &str) {

        // stellar remove from self.cache

        // add key to grave
        
    }

}


pub fn store(name: &str) -> Store {

    // initiate cache
    let cache = vec![vec![]];

    // load logs into cache

    let store = Store {
        name: String::from(name),
        cache: cache
    };

    return store
    
}
