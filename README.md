
# NeutronDB Rust

Rust Implementation of NeutronDB, LSM-Tree Key-Value Store.

## Usage

```

neutrondb = "0.9.17"

```

```

use neutrondb::Store;

```

## Functions

### New

```

let mut accs = Store::new("accs")?;

```

### Put

```

accs.put("user1", "balance1")?;

```

### Get

```

let balance1: String = accs.get("user1")?;

```

### Get All

```

let accounts: Vec<(String, String)> = accs.get_all()?;

```

### Delete

```

accs.delete("user1")?;

```

## Future Topics
- Increase Bloom Filter Size
- Use Sorted Lists on Merged Lists
- Read/Write Performance through multi-threading and batching
- Data Compression
- Error Detection
- Error Correction

## Contribution
Any interested party can contact me on twitter @itsmereystar or email itsmereystar@protonmail.com

2021-09-11
