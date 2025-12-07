# Build Stage
FROM rust:1.83-bookworm AS builder

WORKDIR /usr/src/app

# Install build dependencies
RUN apt-get update && apt-get install -y libssl-dev pkg-config

# Copy sources (Context must be repo root)
COPY ["Reference Implementations/oap-core-rs", "Reference Implementations/oap-core-rs/"]
COPY ["Developer Tools/oap-cli", "Developer Tools/oap-cli/"]

# Build
WORKDIR "/usr/src/app/Developer Tools/oap-cli"
RUN cargo build --release

# Runtime Stage
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    libssl3 \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Copy binary from builder
COPY --from=builder ["/usr/src/app/Developer Tools/oap-cli/target/release/oap-cli", "/usr/local/bin/oap"]

# Entrypoint
ENTRYPOINT ["oap"]
