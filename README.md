# NeutronDB Rust
Rust implementation of NeutronDB, a Log-Structured Merge-tree Key-Value Store using Stellar Notation to Serialize and Encode Data.

## Usage
```
neutrondb = "0.9.0"
stellar-notation = "0.9.0"
```

## Functions

### declarations
```
use neutrondb;
use stellar_notation::StellarObject;
use stellar_notation::StellarValue;
```
### store
```
let mut store = neutrondb::store("my_store").unwrap();
```

### put
```
let key: String = String::from("key_1");
let object: StellarObject = StellarObject(key, value);
store.put(object).unwrap();
```

### get
### delete

## Future Topics
- Read/Write Performance
- Compression
- Error Correction

## Contribution
Any interested party can contact me through email at itsmereystar@protonmail.com
