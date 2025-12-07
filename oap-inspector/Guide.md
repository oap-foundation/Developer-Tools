# OAP Inspector - User Guide

The **OAP Inspector** is the ultimate debugging tool for the Open Agent Protocol (OAP). It allows developers to intercept, inspect, decrypt, visualize, and even replay OAP traffic between agents.

## 🚀 Installation & Running

### Option 1: Docker (Recommended)
We recommend using Docker to ensure all system dependencies are met without polluting your host machine.

```bash
# Build the image
docker build -f "Developer Tools/oap-inspector/Dockerfile" -t oap-inspector .

# Run the container (Exposes UI on port 3000, Proxy on port 9000)
docker run -p 3000:3000 -p 9000:9000 oap-inspector
```
Access the UI at `http://localhost:3000`.

### Option 2: Local Development
**Prerequisites**: Rust (latest stable), Node.js 20+, `libwebkit2gtk` (Linux only).

```bash
cd "Developer Tools/oap-inspector"
npm install
npm run tauri dev
```

---

## 🛠 Features

### Phase 1: The Interceptor (Layer 0)
The Inspector starts a transparent HTTP proxy on port `9000`.
- **Configure Agents**: Point your agent's outbound HTTP calls to use `http://localhost:9000` as a proxy, or send requests directly to `http://localhost:9000/YOUR_TARGET_URL`.
- **View Logs**: All intercepted requests appear in the "Traffic List" on the left.
- **Inspect**: Click a log entry to view Headers, Body, and Status code.

### Phase 2: X-Ray Vision (Layers 1 & 2)
OAP traffic is encrypted (JWE). The Inspector can look inside if you provide the keys.
- **Key Injection**: In the bottom sidebar ("Active Ephemeral Keys"), paste the **X25519 Private Key** (Multibase or Hex) involved in the handshake.
- **Auto-Decryption**: When a message passes through that matches the injected key, the Inspector automatically decrypts it.
- **Handshake Visualizer**: Click the "Handshake" tab to see a sequence diagram of the OAEP handshake (Req -> Res -> Ack).

### Phase 3: Business Logic Inspector (Layer 3)
Analyze the actual OACP (Commerce) and OAPP (Payment) workflows.
- **Business Flow Tab**: This tab visualizes the conversation state machine based on `threadId`. It connects related messages (e.g., Offer -> Order -> Invoice) into a flow chart.
- **Schema Validation**: Decrypted JSON is automatically checked against OAP Schemas. Look for the "✅ Valid" or "❌ Failed" banner.
- **Credential Viewer**: If a message contains a W3C Verifiable Credential (e.g., in a Handshake), an "Identity Card" is rendered at the bottom of the Decrypted view.

### Phase 4: God Mode (Active Debugging)
Don't just watch—interact.
- **Request Replay**:
    1. Select a decrypted request.
    2. Go to "Req (Decrypted)" tab.
    3. Click **"EDIT & RESEND"**.
    4. Modify the JSON payload (e.g., change the price).
    5. Click **"RESEND (REPLAY)"**. The Inspector re-encrypts the packet using the correct session keys and fires it off.
- **Export/Import Sessions**:
    - Use the 📤 **Export** button in the sidebar to save your debug session (decrypted logs included) to a `.oaplog` JSON file.
    - Use 📥 **Import** to load a session from a colleague.

---

## 🔒 Security Note
- **Private Keys**: This tool holds private keys in memory to perform decryption. **Do not** use this with production keys on public networks.
- **Exported Logs**: Exported `.oaplog` files contain **decrypted** data. Treat them as sensitive artifacts.

## ❓ Troubleshooting
- **"Failed to decrypt"**: Ensure you injected the *Private Key* of the *Recipient* (for requests) or the *Sender*'s ephemeral key (if derived). Usually, the recipient's static or ephemeral private key is required.
- **Docker Build Fails**: Run the build command from the **root** `OAP-Github` directory, as the Dockerfile needs access to the `Reference Implementations` folder.
