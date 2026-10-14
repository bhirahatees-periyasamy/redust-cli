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

`redust-server` starts a TCP server (default `127.0.0.1:6380`). Send it one command per connection:

```
cargo run --bin redust-server                # or: cargo run --bin redust-server -- -H 0.0.0.0 -p 7000
cargo run --bin redust-server -- --detach    # run in the background, logs go to redust-server.log

echo "SET name redust" | nc 127.0.0.1 6380   # OK
echo "GET name" | nc 127.0.0.1 6380          # redust
```

| Flag                | Default     | Meaning                                  |
| ------------------- | ----------- | ---------------------------------------- |
| `-H`, `--host <ip>` | `127.0.0.1` | address to bind to                       |
| `-p`, `--port <n>`  | `6380`      | port to listen on                        |
| `-d`, `--detach`    | off         | run in the background (Unix, uses `setsid`) |

There is also an interactive prompt (`InteractiveMode` in `command.rs`) where you type the same commands directly. `exit` or Ctrl-D quits. The plain `redust` binary (`src/main.rs`) still starts a server on `127.0.0.1:1234`; the prompt isn't wired into either binary yet.

More in [docs/usage.md](docs/usage.md).

## Layout

- `service.rs`: public API (`set`, `get`, `delete`, `execute`)
- `server.rs`: the TCP server (`RedustServer`)
- `bin/redust-server.rs`: the `redust-server` binary (flags, detach mode)
- `command.rs`: the `Commands` enum and the interactive prompt (`InteractiveMode`)
- `codec.rs`: converts values between `String` and bytes
- `database.rs`: the in-memory `HashMap` store

## Development

```
cargo run --bin redust-server
cargo test
cargo fmt
cargo clippy --all-targets -- -D warnings
```
