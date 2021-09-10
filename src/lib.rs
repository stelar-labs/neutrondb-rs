
use std::error::Error;

mod init;
mod query;

#[derive(Clone, Debug)]
struct Table {
    name: String,
    level: u8,
    bloom_filter: Vec<u8>
}

#[derive(Clone, Debug)]
pub struct Store {
    pub name: String,
    cache: Vec<(String, String)>,
    cache_buffer: Vec<u8>,
    graves: Vec<String>,
    tables: Vec<Table>
}

impl Store {

    pub fn new(name: &str) -> Result<Store, Box<dyn Error>> {
        init::run(name)
    }

    pub fn put(&mut self, key: &str, value: &str) -> Result<(), Box<dyn Error>> {
        query::put::run(self, key, value)
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        query::get::run(self, key)
    }

    pub fn get_all(&self) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
        query::get_all::run(self)
    }

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        query::delete::run(self, key)
    }

}
