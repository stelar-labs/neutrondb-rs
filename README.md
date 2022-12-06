# NeutronDB

NeutronDB is a log-structured merge-tree key-value store for any implemented data type.

## Usage

### Cargo.toml

```text
[dependencies]
neutrondb = "5.0.0"
```

### Module.rs

```text
use neutrondb::Store;
```

## About

### Files

`Neutron Logs`

```text

    +-------------------+
    |  Log Type & Data  |
    |  ...              |
    +-------------------+

```

`Neutron Table`

```text

    +-----------+
    |  Version  |
    +-----------+

    +-----------------+
    |  Keys & Values  |
    |  ...            |
    +-----------------+

    +----------------+
    |  Bloom Filter  |
    +----------------+

```

`Neutron Graves`

```text

    +--------+
    |  Keys  |
    |  ...   |
    +--------+

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

## Future

- 🚀 batching requests for performance
- 📥 store.put_many(&[(K,V)])
- 📤 store.get_many(&[K]) -> Vec<(K,V)>
- 🦾 store.iter(lambda) -> Vec<_>
- 🧠 store.fold(accumulator, lambda) -> accumulator
- 🔍 store.any(lambda) -> V
- 🐘 store.memory(size)

2022-12-07
