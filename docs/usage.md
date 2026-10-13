# Using redust

redust is an in-memory key-value store. Nothing is saved to disk, so everything is gone when you quit.

## Interactive prompt

```
cargo run
```

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

### Commands

| Command            | What it does                         | Reply                          |
| ------------------ | ------------------------------------ | ------------------------------ |
| `SET <key> <value>` | store a value, overwrites if it exists | `OK`                          |
| `GET <key>`        | read a value                         | the value, or `None` if missing |
| `DELETE <key>`     | remove a key                         | `OK`, or `None` if it wasn't there |
| `EXIT` / `QUIT`    | leave the prompt                     |                                |

Ctrl-C and Ctrl-D also quit.

### Things to know

- Command names don't care about case (`set`, `SET`), keys do (`Name` and `name` are different keys).
- Values are a single word. `SET greeting hello world` stores `hello`.
- Blank lines are ignored.
- Bad input gives `Invalid command`. `SET` without a value gives `SET requires a key and value`.
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
