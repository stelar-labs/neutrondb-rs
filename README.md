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

2022-07-13
