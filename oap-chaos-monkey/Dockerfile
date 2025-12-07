FROM rust:1.83-slim-bookworm as builder

WORKDIR /app
RUN apt-get update && apt-get install -y pkg-config libssl-dev && rm -rf /var/lib/apt/lists/*
# Cache dependencies
COPY Cargo.toml .
# Create dummy main to build deps
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN cargo build --release
RUN rm -rf src

# Real build
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
# Need runtime dependencies usually (ca-certificates, etc)
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/oap-chaos-monkey .
COPY chaos.toml .

# Hardening
RUN useradd -m oap
USER oap

EXPOSE 8080

CMD ["./oap-chaos-monkey"]
