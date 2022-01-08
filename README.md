
# NeutronDB Rust

NeutronDB is a Log-structured merge-tree key-value store library written in Rust.

## Usage

In your `Cargo.toml`:

```

[dependencies]
neutrondb = "2.0.0"

```

In your Rust file:

```

use neutrondb::Store;

```

In .gitignore

```

/ndb/

```

## Features
- Keys and Values are UTF-8 strings of any length.
- Data is stored in linked lists.
- There are five functions connect, put, get, get_all, and delete.

## API

`Connect`

```

let mut accs = Store::connect("accs")?;

```

`Put`

```

accs.put("user1", "balance1")?;

```

`Get`

```

let balance1: String = accs.get("user1")?;

```

`Get All`

```

let accounts: Vec<(String, String)> = accs.get_all()?;

```

`Delete`

```

accs.delete("user1")?;

```

## Future
- Increase Bloom Filter Size
- Read/Write Performance through multi-threading and batching
- Data Compression
- Error Detection
- Error Correction

## Contribution
Pull requests, bug reports and any kind of suggestion are welcome.

2022-01-08
