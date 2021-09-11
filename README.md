
# NeutronDB Rust

Rust Implementation of NeutronDB, LSM-Tree String Key-Value Store.

## Usage

```

neutrondb = "0.9.15"

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
- Read/Write Performance
- Data Compression
- Error Correction

## Contribution
Any interested party can contact me on twitter @itsmereystar or email itsmereystar@protonmail.com

2021-09-11
