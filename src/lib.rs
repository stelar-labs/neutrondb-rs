mod bloom;
mod neutron;
mod store;
use std::collections::BTreeMap;

#[derive(Debug)]
pub struct Store {
    cache: BTreeMap<String, String>,
    cache_size: u64,
    directory_location: String,
    graves: Vec<String>,
    tables: Vec<Table>
}

#[derive(Debug)]
pub struct Table {
    pub bloom: bloom::Bloom,
    pub level: u8,
    pub name: String,
    pub size: u64
}
