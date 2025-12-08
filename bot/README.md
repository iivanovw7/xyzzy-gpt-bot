# bot

teloxide telegram bot

- [Crates](#crates)
- [Installation](#installation)
- [Scripts](#scripts)

## Crates

- [teloxide](https://github.com/teloxide/teloxide)
- [asyncopenapi](https://github.com/64bit/async-openai)
- [tokio](https://docs.rs/tokio/latest/tokio)

## Installation

`.env` file example

```bash
TOKEN=XXX
OPEN_API_KEY=XXXX
USER_ID=99999
MODEL=gpt-4-turbo
DATABASE_URL=sqlite:data.db
WEB_APP_URL=https://WEB_APP_URL
```

### Setup `db` locally

```bash
rm -rf db
mkdir -p db

export DATABASE_URL=sqlite:///$(pwd)/db/data.db

sqlx database create
sqlx migrate run

cargo sqlx migrate run
```

## Scripts

```bash

cargo check
cargo run
cargo run --release
cargo build
cargo build --release
```
