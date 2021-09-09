
use std::error::Error;

mod init;
mod query;

#[derive(Clone, Debug)]
pub struct Table(pub String, pub u8, pub Vec<u8>);

#[derive(Clone, Debug)]
pub struct Store {
    pub name: String,
    pub cache: Vec<(String, String)>,
    pub cache_buffer: Vec<u8>,
    pub grave: Vec<String>,
    pub tables: Vec<Table>
}

impl Store {

    pub fn put(&mut self, object: (String, String)) -> Result<(), Box<dyn Error>> {
        query::put::run(self, object)?;
        Ok(())
        
    }

    pub fn get(&self, key: &str) -> Result<Option<String>, Box<dyn Error>> {
        let result = query::get::run(self, key)?;
        Ok(result)

    }

    pub fn delete(&mut self, key: &str) -> Result<(), Box<dyn Error>> {
        query::delete::run(self, key)?;
        Ok(())
        
    }

    pub fn get_all(&self) -> Result<Option<Vec<(String, String)>>, Box<dyn Error>> {
        let result = query::get_all::run(self)?;
        Ok(result)
        
    }

}

pub fn store(name: &str) -> Result<Store, Box<dyn Error>> {
    let result: Store = init::run(name)?;
    Ok(result)

}
