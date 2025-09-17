### Lockfile stage
FROM cgr.dev/chainguard/rust AS lockfile
WORKDIR /work
COPY ./Cargo.toml ./
COPY ./.build/src ./src
RUN cargo generate-lockfile


### Build stage
FROM cgr.dev/chainguard/rust AS builder
WORKDIR /work
COPY ./Cargo.toml ./
COPY --from=lockfile /work/Cargo.lock ./
# Cache dependencies (Save time on rebuilds)
COPY ./.build/src ./src
RUN cargo build
ARG CACHE_BUST=1
# Docker starts here on rebuilds
COPY ./src ./src
RUN cargo clean --package parameros-api
RUN cargo build

### Production stage
FROM cgr.dev/chainguard/wolfi-base
RUN apk add --no-cache libpq
COPY --from=builder /work/target/debug/parameros-api /usr/local/bin/api
RUN mkdir -p /api/uploads
RUN chown -R 65532:65532 /api/uploads
CMD ["api"]
