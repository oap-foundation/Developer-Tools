# OAP VS Code Extension

**Open Agent Protocol (OAP) Developer Tools for VS Code.**

Supercharge your OAP development with intelligent schema validation, snippets, WASM-powered key management, and integrated local environment control.

![OAP Icon](icon.png)

## Features

### 1. Intelligent Editing (Phase 1)
- **JSON Validation**: Automatic red-squiggles and validation for OACP (`Offer`, `Proposal`) and OAPP (`PaymentRequest`) files.
- **Snippets**: Type `oacp-offer`, `oapp-payment`, and more to instantly generate protocol-compliant JSON structures.

### 2. OAP Intelligence (Phase 2)
- **DID Resolution**: Hover over any `did:key:...` string to see its resolution status, key type, and creation date. Powered by OAP Core Rust (WASM).
- **Identity Generation**: Run `OAP: Generate New Identity` to insert a fresh DID and key pair directly into your code.

### 3. Integrated Workflow (Phase 3)
- **LocalNet Controller**: Start and stop your local OAP Network (Docker Compose) directly from the Status Bar.
- **Interaction Simulation**: Right-click any JSON file and select "Send to LocalNet Relay" to test your agents.
- **Project Scaffolding**: Create new Node.js or Python agent projects with a single command: `OAP: Create New Agent Project`.

## Requirements
- **Docker**: Required for LocalNet control.
- **OAP LocalNet**: A local clone of the `oap-localnet` repository (configure path in settings).

## Extension Settings

This extension contributes the following settings:

* `oap.localNetPath`: Path to your local `oap-localnet` folder (default: `../oap-localnet`).
* `oap.relayUrl`: URL of the OAP Relay for simulation (default: `http://localhost:8000`).

## Known Issues
- WASM module loading may fail if system dependencies for the Node binding are missing (mostly relevant in custom dev container setups).
- LocalNet control assumes `docker` is in your system PATH.

## Release Notes

### 1.0.0
- Initial release with Schemas, Snippets, WASM integration, and LocalNet control.

## Documentation
For a deep dive into all features, see the [User Guide](Guide.md).

---
**Enjoy building with OAP!**
