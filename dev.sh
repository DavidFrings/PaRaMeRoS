#!/bin/sh

if [ -f .env ]; then
  . ./.env
else
  echo ".env file not found"
  exit 1
fi

docker compose -f docker-compose.dev.yml down
docker compose -f docker-compose.dev.yml up -d --build
