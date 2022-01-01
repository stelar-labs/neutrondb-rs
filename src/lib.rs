
use std::error::Error;
use std::fmt;

mod list;
mod store;

#[derive(Debug)]
struct CustomError(String);

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "NeutronDB Error: {}", self.0)
    }
}

impl Error for CustomError {}

#[derive(Clone, Debug)]
struct List {
    name: String,
    level: u8,
    bloom_filter: Vec<u8>
}

#[derive(Clone, Debug)]
pub struct Store {
    name: String,
    cache: Vec<(String, String)>,
    graves: Vec<String>,
    lists: Vec<List>
}

impl Store {

    pub fn connect(name: &str) -> Result<Store, Box<dyn Error>> {
        store::init::run(name)
    }

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        store::put::run(self, key, value)
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        store::get::run(self, key)
    }

    pub fn get_all(&self) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
        store::get_all::run(self)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        store::delete::run(self, key)
    }

}
