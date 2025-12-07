# OAP Chaos Monkey - User Guide

The **OAP Chaos Monkey** is a proxy designed to test the resilience of Open Agent Protocol (OAP) applications. By sitting between your Agent and the Relay, it can simulate network failures, data corruption, and security attacks.

## Getting Started

### Prerequisites
- **Docker**: The recommended way to run the tool.
- **Rust**: If you prefer building from source.

### Quick Start
1.  **Build the Image**:
    ```bash
    docker build -t oap-chaos .
    ```
2.  **Run with a Preset**:
    ```bash
    # Simulate a bad connection (High Latency, 10% Packet Loss)
    docker run -p 8080:8080 oap-chaos --mode subway
    ```
3.  **Configure your Agent**:
    Point your OAP SDK or Agent to `http://localhost:8080`.

## Chaos Modes (Presets)

The tool supports `presets` via the `--mode` flag, which override the configuration:

| Mode | Description | Effects |
| :--- | :--- | :--- |
| `subway` | "Bad WiFi" simulation | **Latency**: Jitter (500-3000ms)<br>**Packet Loss**: 10% |
| `malicious-relay` | Hostile Network | **sabotage**: 20% Corruption<br>**Security**: Replay Attacks enabled |
| `ddos` | Extreme Stress | **Packet Loss**: 90% |

## Detailed Configuration (`chaos.toml`)

For fine-grained control, edit `chaos.toml`:

### `[server]`
- `port`: Port to listen on (default: `8080`).
- `target_url`: Where to forward valid requests (e.g., `https://relay.oap.dev`).

### `[chaos]` (Network Layer)
- `latency_mode`:
    - `"Fixed"`: Constant delay (`latency_fixed_ms`).
    - `"Jitter"`: Random delay between `min_ms` and `max_ms`.
- `failure_rate`: Chance (0.0-1.0) to return an HTTP Error (500/503) immediately.

### `[sabotage]` (Data Layer)
- `mode`:
    - `"PacketLoss"`: Drops the request (returns 200 OK to sender).
    - `"Corrupt"`: Flips a random byte in the body.
    - `"Truncate"`: Cuts body in half.
- `drop_rate`: Probability of sabotage.
- `target_shard_indices`: Array of integers (e.g., `[3, 4]`). If set, only affects shards with these indices (smart targeting).

### `[security]` (Attack Layer)
- `replay_enabled`: If true, resends requests after `replay_delay_ms`.
- `mitm_downgrade`: Attempts to downgrade `cipher_suite` in Handshakes.
- `exhaustion_flood`: **Passive/Active flood**. Be careful enabling this!

## Metrics & Reporting
When the server shuts down (e.g., via `Ctrl+C` or `docker stop`), it prints a summary of all events to the logs:

```text
📊 --- OAP CHAOS MONKEY REPORT ---
Total Requests:      1240
Successful Forwards: 1100
Dropped (Loss):      120
Corrupted/Sabotaged: 15
Replayed (Security): 5
---------------------------------
```
