#!/bin/bash

if [ -f .env ]; then
    set -a
    source .env
    set +a
fi

sudo docker compose down
rm -r ./migrations

curl -L https://raw.githubusercontent.com/DavidFrings/PaRaMeRoS/refs/heads/main/docker-compose.yml -o docker-compose.yml
curl -L https://github.com/DavidFrings/PaRaMeRoS/archive/refs/heads/main.zip -o main.zip

mkdir -p tmp_unzip
unzip -q main.zip -d tmp_unzip
mkdir -p migrations
mv tmp_unzip/PaRaMeRoS-main/api/migrations/* migrations/
rm -rf tmp_unzip main.zip

sudo docker compose up -d

export DATABASE_URL=postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@127.0.0.1:${POSTGRES_PORT}/${POSTGRES_DB}
diesel migration run
