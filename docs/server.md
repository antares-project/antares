# harmon-server

*all commands must be executed in the ./server folder*

## Developing

```bash
cargo run
```

## Building

```bash
cargo build --release
```

#### Sqlx-cli
used to manage migrations
```rs
cargo install sqlx-cli --no-default-features --features rustls,sqlite
```

## migrations
create new migration
```sh
sqlx migrate add -r <name>
```

migrations are applied automatically when the project is run, however, if you want to apply them manually, it would be like this

```sh
sqlx migrate run
cargo sqlx prepare
```

### generate Ed25519 key
```sh
openssl genpkey -algorithm Ed25519 -outform DER | tail -c 32 | xxd -p -c 32
```