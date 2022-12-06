mod store;
use std::collections::BTreeMap;
use fides::BloomFilter;

#[derive(Debug)]
pub struct Store<K,V> {
    pub cache: BTreeMap<K, V>,
    pub directory: String,
    pub graves: Vec<K>,
    pub tables: Vec<Table>
}

#[derive(Debug)]
pub struct Table {
    pub bloom_filter: BloomFilter,
    pub count: u64,
    pub level: u8,
    pub name: String,
    pub size: u64
}
