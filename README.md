# xyzzy-gpt-bot

Telegram bot for `chat-gpt` and `budgeting`.

Contents:

- [Requirements](#requirements)
- [Installation](#installation)
- [Scripts](#scripts)
- [Dockerfile](#dockerfile)

## Requirements

- cargo 1.77.2 (e52e36006 2024-03-26)
- Rustc 1.77.2 (25ef9e3d8 2024-04-09)

## Installation

```bash
sudo pacman -S rustup
sudo pacman -S sqlite

rustup default stable

cargo add justfile
cargo add sqlx
```

## Scripts

```bash
just bot
```

## Dockerfile

- Install docker

```bash
sudo pacman -Syu
sudo pacman -S docker docker-compose
sudo systemctl start docker.service
sudo systemctl enable docker.service
sudo usermod -aG docker $USER

docker version
docker-compose --version
```

- Commands

```bash
# clear cache
docker system prune -a
docker image prune

# Update docker image after changes
chmod +x recompose.sh
./recompose.sh
```
