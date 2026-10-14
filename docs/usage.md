# Using redust

redust is an in-memory key-value store. Nothing is saved to disk, so everything is gone when you quit.

## TCP server

```
cargo run --bin redust-server
```

This starts a server on `127.0.0.1:6380` and prints `Starting Redust Server on 127.0.0.1:6380`. Talk to it with any TCP client, such as `nc`:

```
$ echo "SET name redust" | nc 127.0.0.1 6380
OK
$ echo "GET name" | nc 127.0.0.1 6380
redust
$ echo "DELETE name" | nc 127.0.0.1 6380
OK
$ echo "GET name" | nc 127.0.0.1 6380
None
```

The store is shared across connections, so a key set on one connection can be read on the next.

### Server options

```
cargo run --bin redust-server -- --host 0.0.0.0 --port 7000
redust-server --detach
redust-server --help
```

| Flag                | Default     | Meaning                                       |
| ------------------- | ----------- | --------------------------------------------- |
| `-H`, `--host <ip>` | `127.0.0.1` | address to bind to                            |
| `-p`, `--port <n>`  | `6380`      | port to listen on                             |
| `-d`, `--detach`    | off         | run in the background and return immediately  |
| `-V`, `--version`   |             | print the version                             |

With `--detach` the server relaunches itself via `setsid` (Unix only), prints `Redust server process launched (PID: <pid>). Logs: redust-server.log`, and exits. Output goes to `redust-server.log` in the current directory (appended). To stop it, `kill <pid>`.

The plain `redust` binary (`cargo run --bin redust`) is the older entry point and still listens on the fixed address `127.0.0.1:1234`.

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
- If the port is taken, startup fails with `Redust server error: ...` and exit code 1.
- Command names don't care about case (`set`, `SET`), keys do (`Name` and `name` are different keys).
- Values are a single word. `SET greeting hello world` stores `hello`.
- Bad input gives `Invalid command`. `SET` without a value gives `SET requires a key and value`.

## Interactive prompt

`InteractiveMode` in `command.rs` reads commands from a prompt. Neither binary starts it yet, so call it from Rust with `InteractiveMode::new().read_cmd()`.

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
