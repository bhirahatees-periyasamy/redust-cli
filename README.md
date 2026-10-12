# redust

A mini Redis written in Rust: an in-memory key-value store with `SET`, `GET` and `DELETE`.

## Usage

```rust
use redust::service::{RedustService, ServiceOperations};

let mut service = RedustService::new();
service.set("name".into(), "redust".into()); // "OK"
service.get("name");                         // Some("redust")
service.delete("name");                      // true
```

## Layout

- `service.rs`: public API (`set`, `get`, `delete`)
- `codec.rs`: converts values between `String` and bytes
- `database.rs`: the in-memory `HashMap` store

## Development

```
cargo run
cargo test
cargo fmt
cargo clippy --all-targets -- -D warnings
```
