# NeutronDB

NeutronDB is a log-structured merge-tree key-value store for any implemented data type.

## Authors

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## About

### Neutron Table Binary Format

- Version (1 Byte)
- Key Count (8 Bytes)
- Index Position (8 Bytes)
- Keys Position (8 Bytes)
- Bloom Filter (Dynamic Size)
- Index Data (Dynamic Size)
- Keys Data (Dynamic Size)
- Values Data (Dynamic Size)

## Features

- Key-Value Store with Put, Get & Delete
- Supports Rust Native Types
- Supports any Type that implements IntoBytes & TryFromBytes
- Supports 64bit Data Locations within Tables, Max ~18 Exabyte Files

## Usage

### Installation

<!-- - From [Crates](https://crates.io/) by running `cargo add neutrondb` -->
- Using Crates by adding `neutrondb = "6.0.0"` to `Cargo.toml` under `[dependencies]`

### New

- `new: directory_path -> store/error`
- Example: `let mut accounts_store: Store<Hash, Account> = Store::new("./ndb")?;`

### Put

- `put: &key, &value -> ()/error`
- Example: `accounts_store.put(&Hash, &Account)?;`

### Get

- `get: &key -> value/error`
- Example: `let account = accounts_store.get(&Hash)?;`

### Delete

- `delete: &key -> ()/error`
- Example: `accounts_store.delete(&Hash)?;`

### Cache Limit

- `cache_limit: u64`
- sets cache limit
- default & minimum cache limit is 1MB
- Example: `accounts_store.cache_limit(1_000_000_000)`

## Future

- 🚀 batching requests for performance
- 📥 store.puts(&[(K,V)])
- 📤 store.gets(&[K]) -> BTreeMap<K,V>
- 🗺️ store.map: (Fn: (K,V) -> T) -> Vec<T>
- 🧠 store.fold: acc, Fn: (K,V) -> acc -> acc
- 🔍 store.any: Fn: (K,V) -> bool -> V
- 🗄 store.all: Fn: (K,V) -> bool -> BTreeMap<K,V>
- 📸 snapshots

## Scaling

- Supports 64bit Data Positions
- Use ZFS

## Memory

NeuronDB requires memory:
- General: Graves(Key Hashes of deleted items)
- Cache: Key Hashes & Values
- Flushing & Compaction: New Table Index & Data Locaions

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
