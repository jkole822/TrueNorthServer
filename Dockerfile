# === Builder stage ===
FROM rustlang/rust:nightly-slim AS builder

WORKDIR /app
COPY . .

# Install dependencies required for building with OpenSSL + Postgres
RUN apt-get update && \
    apt-get install -y pkg-config libssl-dev libpq-dev && \
    cargo install sqlx-cli --no-default-features --features postgres && \
    rm -rf /var/lib/apt/lists/*

# Build the release binary
RUN cargo build --release

# === Runtime stage ===
# Uses Debian Bookworm to include OpenSSL 3 (libssl.so.3)
FROM debian:bookworm-slim

WORKDIR /app

# Install runtime dependencies (OpenSSL 3 for Rust + Postgres)
RUN apt-get update && \
    apt-get install -y libssl3 libpq5 ca-certificates && \
    rm -rf /var/lib/apt/lists/*

# Copy compiled binary and sqlx CLI
COPY --from=builder /app/target/release/true_north_server .
COPY --from=builder /usr/local/cargo/bin/sqlx /usr/local/bin/sqlx

# Copy migration files
COPY --from=builder /app/migrations ./migrations

# Add script to run migrations + start app
COPY entrypoint.sh ./entrypoint.sh
RUN chmod +x ./entrypoint.sh
ENV RUST_LOG=info
ENTRYPOINT ["./entrypoint.sh"]
