# VeloPay - Build & Run Guide

## Prerequisites

- Rust 1.70+ with wasm32-unknown-unknown target
- protobuf-compiler
- OpenSSL (Windows: set `OPENSSL_VENDORED=1`)

```powershell
# Install Rust targets and components
rustup target add wasm32-unknown-unknown
rustup component add rust-src
```

## Build

```powershell
cd velo-chain

# Windows: Set environment variable
$env:OPENSSL_VENDORED = "1"

# 1. Check code compiles (fast, no binary)
cargo check

# 2. Run linter (optional)
cargo clippy

# 3. Build release binary
cargo build --release

# Binary created at: target\release\velo-node.exe
```

**Build time**: 15-45 minutes (first build), 2-5 minutes (incremental)

## Run

### Quick Start (Development)

Temporary chain with Alice as validator:

```powershell
.\quick-start.bat
```

### Persistent Single Validator

```powershell
.\run-alice.bat
```

### Two-Validator Network

Terminal 1:
```powershell
.\run-alice.bat
```

Terminal 2:
```powershell
.\run-bob.bat
```

### Custom Chain Spec

```powershell
# Generate chain spec (one time)
.\generate-spec.bat

# Run with custom spec
.\start-local.bat
```

## Test

```powershell
# Run unit tests
cargo test

# Test RPC endpoints (node must be running)
.\test-rpc.bat

# Python test script (node must be running)
python test-chain.py
```

## Verify

```powershell
# Check the build succeeded
dir target\release\velo-node.exe

# Should show ~68 MB binary file
```

## RPC Endpoints

When node is running:
- **HTTP RPC**: http://localhost:9944
- **WebSocket**: ws://localhost:9944

Test with curl:
```powershell
curl -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_chain\"}" http://localhost:9944
```

## Common Issues

### Build Errors

**Network timeout downloading crates:**
```powershell
cargo fetch  # Pre-download dependencies
cargo build --release
```

**Out of memory:**
```powershell
cargo build --release -j 2  # Use fewer parallel jobs
```

### Runtime Errors

**GenesisBuilder error:**
Make sure you've pulled the latest code with the GenesisBuilder API fix.

**NetworkKeyNotFound:**
Scripts auto-generate keys with `--unsafe-force-node-key-generation` flag.

## Next Steps

See **TESTING.md** for detailed testing guide and **WINDOWS-BUILD-GUIDE.md** for troubleshooting.
