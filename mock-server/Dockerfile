FROM rust:1-slim AS builder
WORKDIR /build
COPY mock-server/Cargo.toml mock-server/Cargo.toml
COPY mock-server/src mock-server/src
WORKDIR /build/mock-server
RUN cargo build --release

FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*
COPY --from=builder /build/mock-server/target/release/mock-server /usr/local/bin/mock-server
RUN mkdir -p /data/reports
EXPOSE 8080
HEALTHCHECK --interval=10s --timeout=3s --start-period=5s \
    CMD curl -f http://localhost:8080/health || exit 1
ENTRYPOINT ["mock-server"]
