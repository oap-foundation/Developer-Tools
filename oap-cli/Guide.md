# OAP CLI User Guide

The **Open Agent Protocol (OAP) CLI** is the official command-line tool for interacting with the OAP ecosystem. It allows developers to generate identities, inspect JWE messages, managing keys, and test relay connectivity.

## 🚀 Installation

### Option 1: Docker (Recommended)
The easiest way to use the CLI without setting up a Rust environment.

```bash
# From the root of the OAP-Github repository
docker build -t oap-cli -f "Developer Tools/oap-cli/Dockerfile" .

# Run the CLI
docker run --rm -it oap-cli --help
```

### Option 2: Build from Source
Requires Rust 1.83+ and `libssl-dev`.

```bash
cd "Developer Tools/oap-cli"
cargo install --path .
```

## 📖 Usage Guide

### 1. Identity Management (`did`)

Generate a new [did:key](https://w3c-ccg.github.io/did-method-key/) identity.

```bash
# Generate plain text output
oap did gen

# Generate JSON output (useful for piping)
oap did gen --format json --alias my-agent
```

**Output Example:**
```json
{
  "did": "did:key:z6Mk...",
  "secret_key": "abc123...",
  "public_key": "xyz789...",
  "alias": "my-agent"
}
```

> [!WARNING]
> **Security Notice**: The `secret_key` allows control over the identity. Store it securely (e.g., in a password manager or environment variable).

### 2. Network Testing (`relay`)

Check if an OAP Relay is reachable and measuring latency.

```bash
oap relay ping --url http://localhost:3000
```

### 3. Message Inspection (`msg`)

debug JWE (JSON Web Encryption) envelopes without needing the recipient's private key (for headers) or with the key (for payload).

**Decode Headers (No Key Required):**
```bash
oap msg decode --jwe "eyBk..."
```

**Decrypt Payload:**
```bash
oap msg decrypt --jwe "eyBk..." --key "SECRET_KEY_HEX"
```

### 4. Connection Simulation (`connect`)

Simulate an OAEP handshake with another agent (requires a running relay/agent).

```bash
oap connect did:key:z6MkPeer...
```

## ⚙️ Configuration

The CLI loads configuration from `~/.config/oap/config.toml`.

```toml
# Default Relay URL
default_relay = "http://localhost:3000"
```

## 🛡️ Security & Hardening

- **Memory Safety**: Built with Rust for memory safety.
- **Key Validations**: The CLI validates key lengths and generated DIDs.
- **TLS**: Uses `rustls` for secure transport by default.

## 🛠️ Troubleshooting

**"OpenSSL not found" during build:**
Install `pkg-config` and `libssl-dev` (Ubuntu/Debian) or `openssl` (macOS via Homebrew).

**"Connection Refused" when pinging localhost from Docker:**
If running inside Docker, `localhost` refers to the container. Use `host.docker.internal` (macOS/Windows) or `--network host` (Linux) to access services on the host machine.
