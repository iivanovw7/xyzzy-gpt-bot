# ================================
# STAGE 0: Base Rust Setup
# ================================
FROM rust:slim-bookworm AS rust_base

RUN apt-get update -y \
    && apt-get install -y \
        pkg-config \
        libssl-dev \
        openssl \
        perl \
        build-essential \
        libsqlite3-dev \
    && rm -rf /var/lib/apt/lists/*

RUN cargo install cargo-chef --locked

# ================================
# STAGE 1: Chef Planner
# ================================
FROM rust_base AS chef
WORKDIR /app
COPY bot .
RUN cargo chef prepare --recipe-path recipe.json

# ================================
# STAGE 2A: Rust Builder
# ================================
FROM rust_base AS rust_builder
WORKDIR /

COPY --from=chef /app/recipe.json /bot/recipe.json
RUN cargo chef cook --release --recipe-path /bot/recipe.json

COPY bot /bot
RUN cd /bot && cargo build --release

# ================================
# STAGE 2B: Frontend Builder
# ================================

FROM node:24-slim AS frontend_builder
WORKDIR /app/web

RUN npm install -g pnpm
COPY web/package.json web/pnpm-lock.yaml ./

RUN pnpm install --frozen-lockfile
COPY web/ .

RUN pnpm run ng build --configuration production --output-path=dist/web/browser

# ================================
# STAGE 3: Final Runtime Image
# ================================
FROM debian:bookworm-slim AS runtime
WORKDIR /bot

RUN apt-get update -y \
    && apt-get install -y \
        libssl3 \
        libsqlite3-0 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /db

COPY --from=rust_builder /bot/target/release/bot ./server
COPY --from=frontend_builder /app/web/dist/web/browser ./web-dist

COPY bot/.env .
COPY bot/config.toml .

EXPOSE 80 443 8080 5005

USER root

ENTRYPOINT ["sh", "-c", "mkdir -p /db && ./server"]
