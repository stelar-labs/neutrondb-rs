
# NeutronDB Rust

Rust implementation of NeutronDB, LSM-tree Key-Value Store.

## Usage

```

neutrondb = "0.9.13"

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

let key: String = String::from("user_1");

let value: String = String::from("bal_1");

accs.put((key, value))?;

```

### get

```

let value: String = accs.get("user_1")?;

```

### get all

```

let values: Vec<(String, String)> = accs.get_all()?;

```

### delete

```

accs.delete("user_1")?;

```

## Future Topics
- Flush & Compact in accordance with level capacity
- Increase Bloom Filter size
- Read/Write Performance
- Compression
- Error Correction

## Contribution
Any interested party can contact me through twitter @itsmereystar or email at itsmereystar@protonmail.com
