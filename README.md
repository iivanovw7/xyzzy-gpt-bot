# xyzzy-gpt-bot

Telegram bot for `chat-gpt`.

Contents:

-   [Crates](#crates)
-   [Requirements](#requirements)
-   [Installation](#installation)
-   [Scripts](#scripts)
-   [Dockerfile](#dockerfile)

## Crates

-   [teloxide](https://github.com/teloxide/teloxide)
-   [asyncopenapi](https://github.com/64bit/async-openai)
-   [tokio](https://docs.rs/tokio/latest/tokio)

## Requirements

-   cargo 1.77.2 (e52e36006 2024-03-26)
-   Rustc 1.77.2 (25ef9e3d8 2024-04-09)

## Installation

`.env` file example

```bash
TOKEN=XXX
OPEN_API_KEY=XXXX
USER_ID=99999
MODEL=gpt-4-turbo
```

-   Install rust

```bash
pacman -S rustup
rustup default stable
```

## Scripts

```bash
cargo run
cargo run --release
cargo build
cargo build --release
```

## Dockerfile

-   Install docker

```bash
sudo pacman -Syu
sudo pacman -S docker
sudo systemctl start docker.service
sudo systemctl enable docker.service
sudo usermod -aG docker $USER

sudo docker version

sudo curl -L "https://github.com/docker/compose/releases/download/1.29.2/docker-compose-$(uname -s)-$(uname -m)" -o /usr/local/bin/docker-compose
sudo chmod +x /usr/local/bin/docker-compose

docker-compose --version
# docker-compose version 1.29.2, build 5becea4c

```

-   Build and run container

```bash
docker build -t image-name -f Dockerfile .
docker run -d -p 8080:8080 --name container-name image-name

# clear cache
docker system prune -a
docker image prune

# Update docker image after changes
chmod +x recompose.sh
./recompose
```
