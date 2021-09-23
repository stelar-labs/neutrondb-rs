
# NeutronDB Rust

NeutronDB is a Log-structured merge-tree key-value store library written in Rust.

## Usage

In Cargo.toml
```

[dependencies]
neutrondb = "0.9.18"

```

In your rust file
```

use neutrondb::Store;

```

In .gitignore
```

/neutrondb/

```

## Features
- Keys and Values are UTF-8 strings of any length.
- Data is stored unsorted in linked lists.
- There are 5 functions new, put, get, get_all, delete.

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
Pull requests, bug reports and any kind of suggestion are welcome.

2021-09-23
