#!/bin/bash
imageName=xyzzy-gpt-bot
containerName=xyzzy-gpt-bot

docker build -t $imageName -f Dockerfile .

echo Delete old container...
docker rm -f $containerName

echo Run new container...
docker-compose up -d
