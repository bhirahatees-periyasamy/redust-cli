# redust

A mini Redis written in Rust: an in-memory key-value store with `SET`, `GET` and `DELETE`.

## Usage

```rust
use redust::service::{RedustService, ServiceOperations};

let mut service = RedustService::new();
service.set("name".into(), "redust".into()); // "OK"
service.get("name");                         // "redust" ("None" if missing)
service.delete("name");                      // true
```

`service.execute("SET name redust".into())` runs a command string (`SET`, `GET`, `DELETE`). Command names are case-insensitive, keys aren't.

`cargo run` starts a prompt where you can type those commands directly. `exit` or Ctrl-D quits. More in [docs/usage.md](docs/usage.md).

## Layout

- `service.rs`: public API (`set`, `get`, `delete`, `execute`)
- `command.rs`: the `Commands` enum and the interactive prompt
- `codec.rs`: converts values between `String` and bytes
- `database.rs`: the in-memory `HashMap` store

## Development

```
cargo run
cargo test
cargo fmt
cargo clippy --all-targets -- -D warnings
```
