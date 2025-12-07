# OAP VS Code Extension - User Guide

This guide provides detailed instructions on how to use the Open Agent Protocol (OAP) features in VS Code.

## 1. Installation
The extension is currently distributed as a `.vsix` file or via source.
- **From Source**: 
  1. Clone the repo.
  2. Run `npm install` and `npm run package`.
  3. Install the generated VSIX.

## 2. Editing OAP Files (Phase 1)
The extension automatically detects OAP JSON files.
- **Validation**: Ensure your JSON has a valid `@context` (e.g., `https://w3id.org/oacp/v1`). You will see red errors if required fields are missing.
- **Snippets**: Start typing `oacp` or `oapp` to see templates.
  - `oacp-offer`: Creates a standard Offer object.
  - `oapp-payment`: Creates a Payment Request.

## 3. Intelligence Features (Phase 2)
### DID Resolution
Hover over any string starting with `did:key:...`.
A tooltip will appear showing resolution details (Method, Status, Creation Date).
*Note*: This requires the WASM module to be loaded.

### Identity Generation
Need a new DID?
1. Open the Command Palette (`Ctrl+Shift+P`).
2. Type `OAP: Generate New Identity`.
3. A new DID will be inserted at your cursor.

## 4. LocalNet Workflow (Phase 3)
### Controlling the Network
1. Set the path to your LocalNet repo in settings: `oap.localNetPath`.
2. Click the **OAP LocalNet** item in the Status Bar (bottom right).
   - 🔴 **Offline**: Click to Start (`docker compose up -d`).
   - 🟢 **Online**: Click to Stop (`docker compose down`).
3. Check the **Output** tab (select "OAP LocalNet" in the dropdown) for logs.

### Simulating Interactions
1. Right-click any JSON file in the explorer.
2. Select **OAP: Send to LocalNet Relay**.
3. The file content is sent content to the configured Relay URL (`oap.relayUrl`).

## 5. Troubleshooting
- **WASM Error**: If you see "WASM module not loaded", check if `iconv` or other binary deps are missing on your OS. The extension uses a pre-compiled WASM that binds to Node.js.
- **Docker Error**: Ensure `docker` is in your system PATH and the Docker daemon is running.
