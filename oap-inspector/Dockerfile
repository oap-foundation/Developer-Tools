# Start with a modern Rust image
FROM rust:latest AS builder

# Install build dependencies for Tauri and Node.js
# - curl, git for setup
# - libwebkit2gtk-4.0-dev, build-essential, etc. for Tauri (linux)
# - Node.js for frontend build
RUN apt-get update && apt-get install -y \
    curl \
    wget \
    gnupg \
    libwebkit2gtk-4.1-dev \
    build-essential \
    libssl-dev \
    libgtk-3-dev \
    libayatana-appindicator3-dev \
    librsvg2-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Install Node.js 20 (as root)
RUN curl -fsSL https://deb.nodesource.com/setup_20.x | bash - \
    && apt-get install -y nodejs

# Create a non-root user
RUN useradd -m -s /bin/bash tauri

WORKDIR /app

# Copy project files and dependency
# We assume build context is the OAP-Github root
COPY ["Developer Tools/oap-inspector", "./Developer Tools/oap-inspector/"]
COPY ["Reference Implementations/oap-core-rs", "./Reference Implementations/oap-core-rs/"]

# Fix permissions
RUN chown -R tauri:tauri /app
USER tauri

WORKDIR "/app/Developer Tools/oap-inspector"

# Install Frontend Dependencies
RUN npm install

# Run Frontend Build
RUN npm run build

# Run Rust Check and Test
# We don't need to build the full bundle (which might require more display server setup)
# but we can verify the backend compiles and tests pass.
WORKDIR "/app/Developer Tools/oap-inspector/src-tauri"
ENV CARGO_TARGET_DIR=/tmp/target
RUN cargo check
RUN cargo test

# If we wanted to build the app (headless warning: tauri build usually expects display or fails without mock)
# RUN cargo build --release
