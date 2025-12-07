# OAP LocalNet

![OAP LocalNet](https://img.shields.io/badge/OAP-LocalNet-blue?style=for-the-badge) ![Docker](https://img.shields.io/badge/Docker-Enabled-blue?style=for-the-badge&logo=docker)

**OAP LocalNet** is a complete, offline-capable simulation of the **Open Agent Protocol (OAP)** ecosystem. It provides a deterministic "sandbox" environment for developing, testing, and debugging OAP applications without connecting to the public Mainnet.

## ✨ Features

*   **Mock Relays (Layer 1)**: Three interconnected OATP relays supporting configurable **Chaos Engineering** (latency, failure rates).
*   **Mock Discovery (Layer 0)**: A PSI (Private Set Intersection) server with a "God Mode" for injecting test contacts.
*   **Mock Ledger (Layer 2)**: An OAPP (Open Agent Payment Protocol) ledger with an unlimited **Faucet** for funding test identities.
*   **Introspection Dashboard**: A web-based UI to visualize network status, view real-time logs, and control chaos settings.
*   **Zero Config**: Automatically generates test identities ("Alice", "Bob", "Mallory") and handles DNS resolution via Docker aliases.

---

## 🚀 Quick Start

### Prerequisites
*   Docker & Docker Compose installed.
*   `curl` (optional, for manual testing).

### 1. Start LocalNet
Use the provided CLI wrapper to launch the stack:

```bash
cd "Developer Tools/oap-localnet"
./oap-localnet start
```

This will build the Docker images and start 6 containers (3 Relays, Discovery, Ledger, Nginx, Dashboard).

### 2. Access the Dashboard
Open **[http://localhost:3000](http://localhost:3000)** in your browser.
You will see the status of all services and a live log stream.

### 3. Get Test Identities
LocalNet automatically seeds 3 developer identities on startup. To see them:

```bash
./oap-localnet logs | grep "Identity"
# Or run a reset to see them generated afresh:
./oap-localnet reset
```

---

## 🏗 Architecture

| Service | Internal URL | Host Access | Description |
| :--- | :--- | :--- | :--- |
| **Relay 1-3** | `http://relayX:8080` | `https://localhost:8443` | OATP Transport nodes. Simulates the mesh network. |
| **Discovery** | `http://discovery:8081` | `http://localhost:8081` | Layer 0 Identity resolution (Phone/Email -> DID). |
| **Ledger** | `http://ledger:8082` | `http://localhost:8082` | Layer 2 Payments. Includes `/faucet`. |
| **Dashboard** | `http://dashboard:3000` | `http://localhost:3000` | Web UI for logs and chaos control. |
| **Nginx** | - | `https://localhost:8443` | SSL Termination. Routes traffic to relays. |

---

## 🎮 Manual Test Guide

### 1. Verify Connectivity
Ensure all services are "online" in the Dashboard.
Run a health check:
```bash
curl -k https://localhost:8443/health
# {"status":"ok","uptime":"forever",...}
```

### 2. Fund an Identity (Faucet)
Grab a DID from the seeder logs (e.g., `did:key:abc...`) and fund it:
```bash
curl http://localhost:8082/faucet/did:key:abc...
# {"status":"funded", "new_balance": 2000000000}
```

### 3. Simulate Network Chaos
Test if your app handles bad network conditions:
1.  Go to the **Dashboard**.
2.  Find **Relay 1**.
3.  Set **Failure Rate** to `0.5` (50% errors) and **Latency** to `500` (ms).
4.  Click **Apply**.
5.  Send requests to `https://localhost:8443` (which routes to relays) and observe 500 errors or delays.

---

## 🛠 CLI Reference

The `./oap-localnet` script wraps Docker Compose commands for convenience:

*   `start`: Builds and starts the network in background mode.
*   `stop`: Stops all running containers.
*   `reset`: **Wipes all data**, rebuilds, and restarts. Generates new identites.
*   `logs`: Follows the logs of all services (`Ctrl+C` to exit).
*   `status`: Shows running containers and ports.
*   `dashboard`: Opens the web interface.

---

## 📦 Transition to Production

When your application is robust on LocalNet, switch to Mainnet by updating your configuration:

1.  **Change URLs**: Point your app from `localhost` to the official OAP Foundation endpoints (e.g., `https://relay.oap.foundation`).
2.  **Switch DID Method**: `did:key` is great for dev, but consider `did:web` (domain-bound) or `did:oap` (ledger-bound) for production identity persistence.
3.  **Real Assets**: The LocalNet ledger uses "Play Money". You will need to onboard with a real OAP Gateway to transact value.

---

## ⚠️ Troubleshooting

**"Port already in use"**
LocalNet uses ports `3000`, `8080`, `8081`, `8082`, `8443`. Ensure these are free.

**"Certificates not found"**
The start script generates them automatically. If missing, run:
```bash
cd nginx && ./generate_certs.sh
```

**"Connection Refused"**
Wait 5-10 seconds after `start`. The containers need a moment to initialize.
