# NeutronDB

NeutronDB is a log-structured merge-tree key-value store for any implemented data type.

## Authors

- Roy R. O. Okello: [Email](mailto:royokello@protonmail.com) & [GitHub](https://github.com/royokello)

## Features

## Usage

### Module.rs

```text
use neutrondb::Store;
```

### New
`new: directory -> Store`

```text
let mut accounts_store: Store<Hash, Account> = Store::new("./ndb")?;
```
### Put
`put: &key, &value`

```text
accounts_store.put(&Hash, &Account)?;
```

### Get
`get: &key -> value`

```text

let account = accounts_store.get(&Hash)?;

```
### Delete
`delete: &key`

```text
accounts_store.delete(&Hash)?;
```

### Cache Size

- Increase cache size
- Minimum cache size is 1MB / 1_000_000

```
accounts_store.cache_size(1_000_000_000)
```

## Future

- 🚀 batching requests for performance
- 📥 store.put_many(&[(K,V)])
- 📤 store.get_many(&[K]) -> Vec<(K,V)>
- 🦾 store.iter(lambda) -> Vec<_>
- 🧠 store.fold(accumulator, lambda) -> accumulator
- 🔍 store.any(lambda) -> V
- 📸 snapshots

## License

MIT License

Copyright Stelar Labs

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

## Disclaimer

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
