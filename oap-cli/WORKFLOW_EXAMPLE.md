# OAP CLI Workflow Example: Testing a Web Shop Agent

This guide demonstrates how to use the OAP CLI to test a Web Shop Agent, covering identity creation, connection testing, and message sending.

## 1. Create Identity

First, generate a new identity for your test client and save it with an alias.

```bash
oap did gen --alias my-test-agent
```

**Output:**
```text
Generated new Identity:
Alias: my-test-agent
DID: did:key:z6Mk...
Public Key: ...
Secret Key: ...
WARNING: Save the Secret Key securely!
Saved to ~/.oap/keystore/my-test-agent.json
```

## 2. Test Handshake

Initiate a handshake with the target agent (e.g., running locally on port 8080) using your created identity.

```bash
oap connect did:web:localhost:8080 --identity my-test-agent --verbose
```

**Output:**
```text
Initiating handshake with did:web:localhost:8080...
Using identity alias: my-test-agent
Client DID: did:key:z6Mk...
Sending ConnectionRequest... OK
Received ConnectionResponse... Signature VALID
Handshake Successful!
Connected to did:web:localhost:8080
```

## 3. Send Message

Send a message to the agent via the OAP Relay network.

```bash
oap send "Hello Shop" --recipient did:web:localhost:8080
```

**Output:**
```text
Sent via Relay [wss://relay.local]. ID: uuid-123
```

## Next Steps

- Use `oap listen` to receive the response.
- Use `oap msg decode` to inspect the encrypted traffic if you have access to the relay logs.
