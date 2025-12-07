# OAP Echo Bot Guide

This guide provides detailed instructions on setting up, configuring, and running the OAP Echo Bot.

## Architecture

The Echo Bot is built using PHP 8.2 and leverages the OAP Language Bindings for PHP:
- **oaep-php**: Handles cryptography (Ed25519, X25519), DIDs, and JWEs.
- **oatp-php**: Handles transport (Relay Client, Blind Inbox).

It operates as a long-running CLI daemon that polls an OAP Relay for new messages.

## Installation

### Local Setup

1.  **Prerequisites**: Ensure PHP 8.2+ and Composer are installed.
2.  **Clone Repository**: Clone the OAP repository.
3.  **Navigate to Bot Directory**:
    ```bash
    cd "Developer Tools/oap-echo-bot"
    ```
4.  **Install Dependencies**:
    ```bash
    composer install
    ```
    *Note: This will link the local `Language Bindings` packages.*

### Docker Setup

The bot includes a `Dockerfile` based on `php:8.2-cli-alpine`.

1.  **Build Image**:
    From the root of the repository (to include bindings):
    ```bash
    docker build -t oap-echo-bot -f "Developer Tools/oap-echo-bot/Dockerfile" .
    ```

## Configuration

### Identity Generation

Before running, the bot needs an identity.

1.  Run the generation script:
    ```bash
    php scripts/generate_did.php
    ```
2.  This creates:
    - `data/keys.json`: Contains your Secret Key and Public Key. **Keep this safe!**
    - `data/did.json`: Your DID Document. Host this at `https://echo.oap.foundation/.well-known/did.json` (or your domain) for `did:web` resolution.

### Bot Configuration

Edit `scripts/run_bot.php` to configure:
- `relay_url`: The URL of the OAP Relay (default: `http://localhost:3000`).
- `poll_interval`: Seconds between inbox checks.

## Running the Bot

### Manual Execution

```bash
php scripts/run_bot.php
```

### Via Docker

```bash
docker run -it \
  -v $(pwd)/data:/app/oap-echo-bot/data \
  oap-echo-bot
```
*Mount the `data` directory to persist keys.*

## Usage

1.  Start the bot.
2.  From another OAP Agent (e.g., Rust CLI), send a `ConnectionRequest` to the bot's DID (`did:web:echo.oap.foundation`).
3.  The bot will respond with a signed `ConnectionResponse`.
4.  Send a text message.
5.  The bot will reply with "You said: [your message]".
