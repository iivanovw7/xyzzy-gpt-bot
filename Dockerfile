FROM lukemathwalker/cargo-chef:latest-rust-1 AS chef

WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS rust_builder
COPY --from=planner /app/recipe.json recipe.json

RUN apt-get update && apt install -y openssl
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

FROM node:bookworm-slim as node_builder

WORKDIR /app

RUN npm install -g corepack@latest && corepack enable

COPY .env .
COPY .nvmrc .

RUN npm install pm2 -g

FROM debian:bookworm-slim AS runtime

WORKDIR /app

COPY --from=rust_builder /app/target/release/xyzzy-gpt-bot ./server
COPY --from=node_builder /app/.env .

RUN apt-get update -y && \
    apt-get install -y ca-certificates

EXPOSE 80
EXPOSE 443
EXPOSE 8080

ENTRYPOINT ["/app/server"]
