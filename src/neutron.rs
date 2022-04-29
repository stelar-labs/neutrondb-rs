mod create;
mod fetch;
mod search;
use std::path::Path;
use std::error::Error;
use std::collections::BTreeMap;

pub fn create(bloom: Vec<u8>, list: BTreeMap<String, String>) -> Vec<u8> {
    create::create(bloom, list)
}

pub fn fetch(buffer: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    fetch::fetch(buffer)
}

pub fn search(key: &str, path: &Path) -> Result<String, Box<dyn Error>> {
    search::search(key, path)
}
