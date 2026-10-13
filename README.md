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

`cargo run` starts a TCP server on `127.0.0.1:1234`. Send it one command per connection:

```
echo "SET name redust" | nc 127.0.0.1 1234   # OK
echo "GET name" | nc 127.0.0.1 1234          # redust
```

There is also an interactive prompt (`InteractiveMode` in `command.rs`) where you type the same commands directly. `exit` or Ctrl-D quits. `main` currently starts the server, so the prompt isn't wired into `cargo run` yet.

More in [docs/usage.md](docs/usage.md).

## Layout

- `service.rs`: public API (`set`, `get`, `delete`, `execute`)
- `server.rs`: the TCP server (`RedustServer`)
- `command.rs`: the `Commands` enum and the interactive prompt (`InteractiveMode`)
- `codec.rs`: converts values between `String` and bytes
- `database.rs`: the in-memory `HashMap` store

## Development

```
cargo run
cargo test
cargo fmt
cargo clippy --all-targets -- -D warnings
```
