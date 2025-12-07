# OAP Chaos Monkey 🐒

A resilience testing tool for the Open Agent Protocol (OAP). It sits between your Agent and the Relay, injecting failure scenarios to verify your application's robustness.

## Features
- **Latency**: Fixed delay or Random Jitter ("Bad WiFi").
- **Packet Loss**: "Shard Eater" drops packets intelligently.
- **Corruption**: "Bit Flipper" modifies payloads to test crypto integrity.
- **Security**: Replay Attacks, Man-in-the-Middle Downgrades, Storage Exhaustion.

## Usage

### Prerequisites
- Docker (recommended) or Rust toolchain.

### Running with Presets
The tool comes with built-in scenarios:

```bash
# Bad Connection (High Latency, 10% Loss)
cargo run -- --mode subway

# Security Test (Corruption, Replay, Downgrade)
cargo run -- --mode malicious-relay

# Stress Test (90% Loss)
cargo run -- --mode ddos
```

### Configuration
Edit `chaos.toml` for fine-grained control over all parameters.

### Docker
```bash
docker build -t oap-chaos .
docker run -p 8080:8080 oap-chaos
```

Now configure your OAP Agent to use `http://localhost:8080` as its Relay URL.

## Documentation
For detailed configuration and modes, see the [User Guide](Guide.md).

## Metrics
When you stop the tool (Ctrl+C), it prints a report of all intercepted events:
```
📊 --- OAP CHAOS MONKEY REPORT ---
Total Requests:      150
Successful Forwards: 120
Dropped (Loss):      20
Corrupted/Sabotaged: 10
Replayed (Security): 5
---------------------------------
```
