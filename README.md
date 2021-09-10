
# NeutronDB Rust

Rust Implementation of NeutronDB, A String Key-Value Store.

## Usage

```

neutrondb = "0.9.14"

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

accs.put("user1", "balance1")?;

```

### get

```

let balance1: String = accs.get("user1")?;

```

### get all

```

let accounts: Vec<(String, String)> = accs.get_all()?;

```

### delete

```

accs.delete("user1")?;

```

## Future Topics
- Increase Bloom Filter size
- Read/Write Performance
- Compression
- Error Correction

## Contribution
Any interested party can contact me on twitter @itsmereystar or email itsmereystar@protonmail.com

2021-09-10
