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
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

# ================================
# STAGE 2: Rust Builder
# ================================
FROM rust_base AS rust_builder
WORKDIR /app

COPY --from=chef /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

COPY . .
RUN cargo build --release

# ================================
# STAGE 3: Final Runtime Image
# ================================
FROM debian:bookworm-slim AS runtime
WORKDIR /app

RUN apt-get update -y \
    && apt-get install -y \
        libssl3 \
        libsqlite3-0 \
        ca-certificates \
    && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /db 

COPY --from=rust_builder /app/target/release/xyzzy-gpt-bot ./server
COPY .env .

EXPOSE 80 443 8080

USER root

ENTRYPOINT ["sh", "-c", "mkdir -p /db && chmod -R 777 /db && /app/server"]
