#!/bin/bash

echo "Stopping and removing old container..."
docker compose down --remove-orphans || true

echo "Building new image and running service..."
docker compose up --build -d
