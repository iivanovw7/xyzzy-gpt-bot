FROM rust:slim-bookworm AS rust_base

RUN cargo install cargo-chef --locked

# === STAGE 1: Chef Planner ===
FROM rust_base AS chef

WORKDIR /app

FROM chef AS planner

COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# === STAGE 2: Rust Builder ===
FROM chef AS rust_builder
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update -y \
    && apt-get install -y openssl perl build-essential \
    && rm -rf /var/lib/apt/lists/*

RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# === STAGE 3: Node Builder ===
FROM node:bookworm-slim as node_builder

WORKDIR /app

RUN npm install -g corepack@latest --force && corepack enable

COPY .env .
COPY .nvmrc .

RUN npm install pm2 -g

# === STAGE 4: Final Runtime Image ===
FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=rust_builder /app/target/release/xyzzy-gpt-bot ./server
COPY --from=node_builder /app/.env .

RUN mkdir -p /app/db

ENV DATABASE_URL=/app/data/data.db

RUN apt-get update -y \
    && apt-get install -y ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/*

EXPOSE 80 443 8080

ENTRYPOINT ["/app/server"]
