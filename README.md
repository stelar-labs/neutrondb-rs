# NeutronDB

NeutronDB is a Log-structured merge-tree key-value store for Rust.

## Disclaimer

NeutronDB is still in development and is unstable until version 3.

## Usage

In your `Cargo.toml`:

```text
[dependencies]
neutrondb = "2.3.0"
```

In your Rust file:

```text
use neutrondb::Store;
```

In .gitignore

```text
/neutrondb/
```

## Features

- Keys and Values are UTF-8 strings of any length.
- There are five functions connect, put, get, get_all, and delete.
- Data is stored in the Neutron table format.
- The Neutron table file has the index length, bloom filter length, index, bloom filter and values.

## API

`New`

```text
let mut accs = Store::new("accs")?;
```

`Put`

```text
accs.put("user1", "balance1")?;
```

`Get`

```text
let bal = accs.get("user_1")?;
```

`Get All`

```text
let accounts = accs.get_all()?;
```

`Delete`

```text
accs.delete("user1")?;
```

## Development

- Read/Write Performance through multi-threading and batching
- Find function
- Memory efficiency to enable large databases

## Contribution

Pull requests, bug reports and any kind of suggestion are welcome.

2022-04-29
