# NeutronDB

NeutronDB is a log-structured merge-tree key-value store for any implemented data type.

## Usage

### Cargo.toml

```text
[dependencies]
neutrondb = "4.0.0"
```

### Module.rs

```text
use neutrondb::Store;
```

## About

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

## API

`new: directory -> Store`

```text
let mut accounts_store: Store<Hash, Account> = Store::new("./ndb")?;
```

`put: &key, &value`

```text
accounts_store.put(&Hash, &Account)?;
```

`get: &key -> value`

```text

let account = accounts_store.get(&Hash)?;

```

`delete: &key`

```text
accounts_store.delete(&Hash)?;
```

2022-10-18
