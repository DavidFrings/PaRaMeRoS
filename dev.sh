#!/bin/sh

if [ -f .env ]; then
  . ./.env
else
  echo ".env file not found"
  exit 1
fi

cd ./api
docker compose -f "../docker-compose.dev.yml" down
docker compose -f "../docker-compose.dev.yml" up -d --build 'db'
sleep 3
diesel migration run --database-url postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${POSTGRES_PORT}/${POSTGRES_DB}
docker compose -f "../docker-compose.dev.yml" down
docker compose -f "../docker-compose.dev.yml" up -d --build
