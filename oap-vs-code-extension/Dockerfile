FROM node:20-bookworm-slim

WORKDIR /app

# Install dependencies for Rust and Node builds
RUN apt-get update && apt-get install -y curl build-essential ca-certificates && rm -rf /var/lib/apt/lists/*

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

# Install wasm-pack
RUN curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh

COPY package*.json ./
RUN npm install

COPY . .

# Build WASM
WORKDIR /app/src/wasm
RUN wasm-pack build --target nodejs --out-dir ../../out/wasm

WORKDIR /app

RUN npm run compile

# Phase 5: Hardening - Run as non-root
RUN useradd -m vscode
USER vscode

CMD ["echo", "Tests ready to run. Use 'npm test' inside if configured for headless."]
