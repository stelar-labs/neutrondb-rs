mod create;
mod get_all;
mod get;
use std::error::Error;
use std::collections::BTreeMap;

pub fn create(bloom: Vec<u8>, list: BTreeMap<String, String>) -> Vec<u8> {
    create::create(bloom, list)
}

pub fn get_all(buffer: &Vec<u8>) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    get_all::get_all(buffer)
}

pub fn get(key: &str, path: &str) -> Result<String, Box<dyn Error>> {
    get::get(key, path)
}

// match

// iterate
