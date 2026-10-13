# Using redust

redust is an in-memory key-value store. Nothing is saved to disk, so everything is gone when you quit.

## TCP server

```
cargo run
```

This starts a server on `127.0.0.1:1234` and prints `Starting Redust Server on 127.0.0.1:1234`. Talk to it with any TCP client, such as `nc`:

```
$ echo "SET name redust" | nc 127.0.0.1 1234
OK
$ echo "GET name" | nc 127.0.0.1 1234
redust
$ echo "DELETE name" | nc 127.0.0.1 1234
OK
$ echo "GET name" | nc 127.0.0.1 1234
None
```

The store is shared across connections, so a key set on one connection can be read on the next.

### Commands

| Command            | What it does                         | Reply                          |
| ------------------ | ------------------------------------ | ------------------------------ |
| `SET <key> <value>` | store a value, overwrites if it exists | `OK`                          |
| `GET <key>`        | read a value                         | the value, or `None` if missing |
| `DELETE <key>`     | remove a key                         | `OK`, or `None` if it wasn't there |

Every reply ends with a newline.

### Things to know

- One command per connection: the server reads a single message (up to 1024 bytes), replies, and moves on to the next connection. Connect again for the next command.
- Connections are handled one at a time, so a client that connects and sends nothing blocks the others.
- The address is fixed at `127.0.0.1:1234` in `main.rs`. If the port is taken, startup fails with an error.
- Command names don't care about case (`set`, `SET`), keys do (`Name` and `name` are different keys).
- Values are a single word. `SET greeting hello world` stores `hello`.
- Bad input gives `Invalid command`. `SET` without a value gives `SET requires a key and value`.

## Interactive prompt

`InteractiveMode` in `command.rs` reads commands from a prompt. `main` currently starts the TCP server instead, so the prompt isn't started by `cargo run` yet; call it from Rust with `InteractiveMode::new().read_cmd()`.

```
redust> SET name redust
OK
redust> GET name
redust
redust> DELETE name
OK
redust> GET name
None
```

It accepts the same commands as the server, plus:

| Command         | What it does     |
| --------------- | ---------------- |
| `EXIT` / `QUIT` | leave the prompt |

Ctrl-C and Ctrl-D also quit.

- Blank lines are ignored.
- Arrow keys work for history inside a session.

## From Rust code

```rust
use redust::service::{RedustService, ServiceOperations};

let mut service = RedustService::new();

service.set("name".into(), "redust".into()); // "OK"
service.get("name");                         // "redust"
service.delete("name");                      // true

// or send a command string, same as the prompt
service.execute("SET name redust".into());   // "OK"
service.execute("GET name".into());          // "redust"
```

## Running the tests

```
cargo test
```
