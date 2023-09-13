 mod store;
use std::{collections::{HashMap, BTreeMap, HashSet}, fs::File};
use fides::BloomFilter;
use std::marker::PhantomData;

const TABLE_HEADER_SIZE: u64 = 25;
const KEY_INDEX_SIZE: u64 = 88;

#[derive(Debug)]
pub struct Store<K,V> {
    directory: String,
    graves: HashSet<[u8;32]>,
    tables: Vec<Table>,
    logs_file: File,
    values: HashMap<[u8;32], ValueObject<V>>,
    cache_size: u64,
    cache_limit: u64,
    cache: BTreeMap<[u8;32], KeyObject>,
    phantom: PhantomData<K>,
}

#[derive(Debug)]
pub struct ValueObject<V> {
    value: V,
    value_size: usize,
    value_log_position: u64,
}

#[derive(Debug)]
pub struct KeyObject {
    value_hash: [u8;32],
    key_size: usize,
    key_log_position: u64,
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
