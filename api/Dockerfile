# Build stage
FROM rust:1.87 AS builder
WORKDIR /usr/src/api
RUN apt-get update && apt-get install -y libpq-dev build-essential
COPY . .
RUN cargo install --path .

# Production stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y libssl3 ca-certificates libpq-dev && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/parameros-api /usr/local/bin/parameros-api
COPY --from=builder /usr/src/api/migrations /usr/local/bin/migrations
WORKDIR /usr/local/bin
CMD ["parameros-api"]