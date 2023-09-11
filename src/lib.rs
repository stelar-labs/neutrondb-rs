 mod store;
use std::{collections::HashMap, fs::File};
use fides::BloomFilter;

#[derive(Debug)]
pub struct Store<K,V> {
    pub cache: HashMap<K, CacheObject>,
    pub directory: String,
    pub graves: Vec<[u8;32]>,
    pub tables: Vec<Table>,
    pub logs_file: File,
    pub values: HashMap<[u8;32], ValueObject<V>>,
    pub cache_size: u64,
}

#[derive(Debug)]
pub struct ValueObject<V> {
    value: V,
    value_size: usize,
    log_position: u64,
}

#[derive(Debug)]
pub struct CacheObject {
    key_hash: [u8;32],
    value_hash: [u8;32],
    key_size: usize,
    log_position: u64,
}

#[derive(Debug)]
pub struct Table {
    pub bloom_filter: BloomFilter,
    pub key_count: u64,
    pub level: u8,
    pub name: String,
    pub file_size: u64,
    pub index_position: u64,
    pub key_data_position: u64,
}
