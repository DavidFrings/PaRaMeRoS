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
COPY ./src ./src
RUN cargo clean --package parameros-api
RUN cargo build --release

### Production stage
FROM cgr.dev/chainguard/wolfi-base
LABEL org.opencontainers.image.source=https://github.com/DavidFrings/PaRaMeRoS
LABEL org.opencontainers.image.description="This is the API service for PaRaMeRoS"
LABEL org.opencontainers.image.licenses=MPL-2.0
RUN apk add --no-cache libpq
COPY --from=builder /work/target/release/parameros-api /usr/local/bin/api
RUN mkdir -p /api/uploads
RUN chown -R 65532:65532 /api/uploads
CMD ["api"]
