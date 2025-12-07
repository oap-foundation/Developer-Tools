# OAP CLI

The official Command Line Interface for the Open Agent Protocol (OAP).

## Requirements

- Rust 1.82+ (Required by modern cryptography dependencies)

## Installation

### Homebrew (macOS/Linux)
```bash
brew tap oap-foundation/tap
brew install oap-cli
```

### Docker
```bash
docker run -it oap-foundation/cli oap --help
```

### Cargo (Rust)
```bash
cargo install oap-cli
```

## Autocompletion

Generate completion script for your shell (bash, zsh, fish, powershell, elvish).

### Zsh
```bash
oap completions zsh > ~/.zfunc/_oap
```

### Bash
```bash
oap completions bash > /etc/bash_completion.d/oap
```

## Usage

### Identity Management

Generate a new DID (Ed25519):

```bash
oap did gen
```

For a complete workflow example (Identity -> Connect -> Send), see [WORKFLOW_EXAMPLE.md](WORKFLOW_EXAMPLE.md).

Resolve a DID (currently supports `did:key`):

```bash
oap did resolve did:key:z6MkhaXgBZDvotDkL5257faiztiGiC2QtKLGpbnnEGta2doK
```

## Configuration

Configuration is stored in `~/.config/oap/config.toml`.
Default relay: `http://localhost:3000`
