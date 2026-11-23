FROM rust:1.100.0-trixie AS builder

ARG CARGO_LEPTOS_VERSION
RUN rustup target add wasm32-unknown-unknown
COPY scripts/install-cargo-leptos.sh /tmp/install-cargo-leptos.sh
RUN sh /tmp/install-cargo-leptos.sh "${CARGO_LEPTOS_VERSION}"

WORKDIR /workspace
COPY Cargo.toml Cargo.lock ./
COPY crates/igsn-web/Cargo.toml crates/igsn-web/Cargo.toml
RUN cargo fetch --locked
COPY crates/igsn-web/src crates/igsn-web/src
COPY crates/igsn-web/style crates/igsn-web/style
RUN cargo leptos build --release --cargo-locked

FROM debian:trixie-slim AS runtime
RUN apt-get update \
    && apt-get install -y --no-install-recommends curl \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /workspace/target/release/igsn-web ./igsn-web
COPY --from=builder /workspace/target/site/ ./site/
ENV LEPTOS_ENV=PROD \
    LEPTOS_SITE_ADDR=0.0.0.0:3000 \
    LEPTOS_SITE_ROOT=/app/site \
    LEPTOS_OUTPUT_NAME=grover
USER 65534:65534
EXPOSE 3000
HEALTHCHECK --interval=15s --timeout=3s --start-period=5s --retries=3 \
    CMD ["curl", "--fail", "--silent", "--output", "/dev/null", "http://127.0.0.1:3000/"]
ENTRYPOINT ["/app/igsn-web"]
