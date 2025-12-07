#!/bin/bash

echo "👋 Stopping containers..."
docker compose -p xyzzy-gpt-bot down --remove-orphans || true

echo "⬇️ Pulling latest base images..."
docker compose pull

echo "🔨 Rebuilding & starting..."
docker compose -p xyzzy-gpt-bot up --build -d

echo "🧹 Cleaning old images..."
docker image prune -f

echo "📦 Running containers:"
docker compose ps
