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

In your module:

```text
use neutrondb::Store;
```

In .gitignore

```text
/neutrondb/
```

## Features

### Tables

- Logs: New entries, recent updates and deletes
- Data: Entries in key order
- Graves: Deleted entries

`Neutron Data Table`

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

### API

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

2022-07-12
