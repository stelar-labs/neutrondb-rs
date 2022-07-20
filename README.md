# NeutronDB

NeutronDB is a log-structured merge-tree key-value store for UTF-8 strings of arbitrary length.

## Disclaimer

still in development

## Usage

In your `Cargo.toml`:

```text
[dependencies]
neutrondb = "3.0.0"
```

In your `module.rs`:

```text
use neutrondb::Store;
```

In your `.gitignore`:

```text
/neutrondb/
```

## Features

### Files

- Logs: Recent puts and deletes
- Table: Non recent key-values
- Graves: Deleted keys

`Neutron Logs`

```text

    + - - - - - - - - - - - - - - - - - - - - - +
    |                                           |
    |   + - - - - - - - +   + - - - - - - - +   |
    |   |   Log 1 Type  |   |   Log 1 Data  |   |
    |   + - - - - - - - +   + - - - - - - - +   |
    |                                           |
    |                    ...                    |   
    + - - - - - - - - - - - - - - - - - - - - - +

```

Type: Data

- put: key, value
- delete: key

`Neutron Table`

```text

    + - - - - - - - - - - - - - +
    |                           |
    |   + - - - - - - - - - +   |
    |   |   Bloom Filter    |   |
    |   + - - - - - - - - - +   |
    |                           |
    |   + - - - - - - - - - +   |
    |   |       Keys        |   |
    |   + - - - - - - - - - +   |
    |                           |
    |   + - - - - - - - - - +   |
    |   |       Index       |   |
    |   + - - - - - - - - - +   |
    |                           |
    |   + - - - - - - - - - +   |
    |   |       Values      |   |
    |   + - - - - - - - - - +   |
    |                           |
    + - - - - - - - - - - - - - +

```

`Neutron Graves`

```text

    + - - - - - - - - - +
    |                   |
    |   + - - - - - +   |
    |   |   Key 1   |   |
    |   + - - - - - +   |
    |                   |
    |       ....        |
    + - - - - - - - - - +

```

### API

`New`

```text
let mut accounts_store = Store::new("accounts")?;
```

`Put`

```text
accounts_store.put("address_1", "balance_1")?;
```

`Get`

```
let balance_1 = accounts_store.get("address_1")?;
```

`Gets`
```
let addresses: Vec<&str>;

let accounts = accounts_store.gets(&addresses)?;
```

`Get All`

```text
let accounts = accounts_store.get_all()?;
```

`Match`
```
let prefix: &str;

let accounts = accounts_store.match(prefix)?;
```

`Range`
```
let first_address: &str;

let last_address: &str;

let accounts = accounts_store.range(first_address, last_address)?;
```

`Each`
```
let function: fn(&str,&str);

accounts_store.each(function)?;
```

`Map`
```
let function: fn(&str,&str) -> T;

let map = accounts_store.map(function)?;
```

`Fold`
```
let function: fn(T,&str,&str) -> T;

let accumulation = accounts_store.fold(function)?;
```

`Delete`

```text
accounts_store.delete("address_1")?;
```

2022-07-21
