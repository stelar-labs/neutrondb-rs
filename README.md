
# NeutronDB Rust

Rust implementation of NeutronDB, LSM-tree Key-Value Store.

## Usage

```

neutrondb = "0.9.10"

```

```

use neutrondb::store;

```

## Functions

### store

```

let mut accs = store("accs")?;

```

### put

```

let k: String = String::from("user_1");

let v: String = String::from("bal_1");

accs.put((k, v))?;

```

### get

```

let v: String = accs.get("user_1")?;

```

### delete

```

accs.delete("user_1")?;

```

## Intermediate Topics
- Increase write performane with cache byte buffer
- Flush & Compact in accordance with level capacity
- Increase Bloom Filter size

## Future Topics
- Read/Write Performance
- Compression
- Error Correction

## Contribution
Any interested party can contact me through email at itsmereystar@protonmail.com
