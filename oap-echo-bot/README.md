# OAP Echo Bot (PHP Edition)

A reference implementation of an OAP Agent in PHP, designed to act as an "Echo Bot" for testing and demonstration purposes.

## Features

- **Identity**: Manages `did:web` identity with Ed25519 keys.
- **Transport**: Connects to OAP Relays via HTTP (OATP).
- **Security**: Implements JWE encryption/decryption (X25519/ChaCha20-Poly1305).
- **Handshake**: Responds to OAEP Connection Requests.
- **Echo**: Mirrors received text messages back to the sender.

## Requirements

- PHP 8.2+
- `ext-sodium`
- `ext-json`
- `ext-zip` (for Composer)
- Composer

## Quick Start

1.  **Install Dependencies**:
    ```bash
    composer install
    ```

2.  **Generate Identity**:
    ```bash
    php scripts/generate_did.php
    ```

3.  **Run Bot**:
    ```bash
    php scripts/run_bot.php
    ```

## Docker

Build and run using Docker:

```bash
docker build -t oap-echo-bot -f Dockerfile ../..
docker run -it oap-echo-bot
```

For detailed instructions, see [GUIDE.md](GUIDE.md).
