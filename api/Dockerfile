### Build stage
FROM cgr.dev/chainguard/rust:latest-dev AS builder
# cmake + clang build the vendored libjxl (jpegxl-rs)
USER root
RUN apk add --no-cache cmake clang
USER nonroot
WORKDIR /work
COPY ./Cargo.toml ./Cargo.lock ./
COPY ./src ./src
COPY ./migrations ./migrations
RUN cargo clean --package parameros-api
RUN cargo build --release --locked

### Production stage
FROM cgr.dev/chainguard/wolfi-base
LABEL org.opencontainers.image.source=https://github.com/DavidFrings/PaRaMeRoS
LABEL org.opencontainers.image.description="This is the API service for PaRaMeRoS"
# Source is MPL-2.0; the binary statically links jpegxl-rs/jpegxl-sys (GPL-3.0-or-later)
LABEL org.opencontainers.image.licenses="MPL-2.0 AND GPL-3.0-or-later"
RUN apk add --no-cache libpq libstdc++
COPY --from=builder /work/target/release/parameros-api /usr/local/bin/api
RUN mkdir -p /api/uploads
RUN chown -R 65532:65532 /api/uploads
USER 65532:65532
CMD ["api"]
