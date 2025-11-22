# VeloPay Node Management Scripts

**Production-ready scripts for managing VeloPay Substrate blockchain validators on Windows.**

## Project Overview

VeloPay is a professional blockchain-based payment system with a fiat-backed stablecoin (VCS) built on Substrate. This repository contains the complete set of Windows batch scripts for:

- Generating persistent Ed25519 node keys
- Starting and managing validator nodes (Alice and Bob)
- Monitoring network status
- Inspecting node configuration
- Resetting testnet data

These scripts solve the problem of ephemeral node identities in development, providing stable Peer IDs necessary for production deployments.

## Prerequisites

### Required Software

| Software | Version | Purpose | Installation |
|----------|---------|---------|--------------|
| **Windows** | 10/11 | Operating system | N/A |
| **velo-node.exe** | Latest | Compiled Substrate node | `cargo build --release` |
| **PowerShell** | 5.1+ | Script execution (built-in) | Pre-installed on Windows |

### Optional Tools

| Tool | Purpose | Installation |
|------|---------|--------------|
| **curl** | Network status checks | https://curl.se/windows/ |
| **Git** | Version control | https://git-scm.com/download/win |

### Build the Node First

```cmd
cd velo-chain
rustup target add wasm32-unknown-unknown
set OPENSSL_VENDORED=1
cargo build --release
```

This creates `target\release\velo-node.exe` which is required by all scripts.

## Quick Start

**Get running in 3 commands:**

```cmd
# 1. Generate node keys (one-time setup)
setup-node-keys-v2.bat

# 2. Start Alice in one terminal
run-alice-final.bat

# 3. Start Bob in another terminal (copy Alice's peer ID when prompted)
run-bob-final.bat
```

**Done!** You now have a 2-validator testnet running with persistent identities.

## Available Scripts

| Script | Purpose | When to Use |
|--------|---------|-------------|
| **setup-node-keys-v2.bat** | Generate persistent node keys | First-time setup or when regenerating identities |
| **run-alice-final.bat** | Start Alice validator | Every time you want to start the primary validator |
| **run-bob-final.bat** | Start Bob validator | Every time you want to start the secondary validator |
| **inspect-node-keys.bat** | View node key information | Debugging or verification |
| **reset-testnet.bat** | Delete blockchain data | Starting fresh or testing upgrades |
| **check-network-status.bat** | Monitor both validators | Checking if nodes are connected |

## First-Time Setup

### Step 1: Build the Node

```cmd
cd velo-chain
set OPENSSL_VENDORED=1
cargo build --release
```

**Wait time:** 15-45 minutes (first build only)

### Step 2: Generate Node Keys

```cmd
setup-node-keys-v2.bat
```

**What it does:**
- Creates `node-keys/` directory
- Generates `alice-node-key` (64-char hex)
- Generates `bob-node-key` (64-char hex)
- Displays security warnings

**Expected output:**
```
[SUCCESS] Node keys generated!

Files created:
  - node-keys\alice-node-key (64 hex characters)
  - node-keys\bob-node-key (64 hex characters)

SECURITY WARNING
[!] Keep these files SECURE and BACKED UP
```

### Step 3: Start Alice

```cmd
run-alice-final.bat
```

**Watch for:**
```
🏷  Local node identity is: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

**Copy that `12D3Koo...` string!** This is Alice's Peer ID.

**Leave this terminal open** - Alice must keep running.

### Step 4: Start Bob (in a NEW terminal)

```cmd
run-bob-final.bat
```

**You'll be prompted:**
```
Enter Alice's Peer ID: 12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
```

Paste Alice's Peer ID and press Enter.

The script saves it to `alice-peer-id.txt` for future runs.

### Step 5: Verify Connection

In Bob's output, look for:
```
💤 Idle (1 peers), best: #0 (0x1234…5678), finalized #0 (0x1234…5678)
```

**1 peers** means success! Bob is connected to Alice.

## Daily Operations

### Starting the Network

**Every time you want to run your testnet:**

```cmd
# Terminal 1
run-alice-final.bat

# Terminal 2 (new window)
run-bob-final.bat
```

Bob will automatically use the saved peer ID from `alice-peer-id.txt`.

### Stopping the Network

Press `Ctrl+C` in each terminal window.

### Checking Status

```cmd
check-network-status.bat
```

**Sample output:**
```
========================================
Quick Status Summary
========================================

[+] Alice is RUNNING
[+] Bob is RUNNING

Network Status: FULLY OPERATIONAL
```

### Resetting Data (Keep Keys)

```cmd
reset-testnet.bat
```

**Use this when:**
- Testing runtime upgrades
- Blockchain data gets corrupted
- Want to start with a fresh chain (but keep same Peer IDs)

**What it deletes:**
- `chain-data\alice\` - Alice's blockchain database
- `chain-data\bob\` - Bob's blockchain database
- `alice-peer-id.txt` - Saved peer ID (will re-prompt)

**What it preserves:**
- `node-keys\` - Your persistent node keys (same Peer IDs!)

### Inspecting Keys

```cmd
inspect-node-keys.bat
```

**Shows:**
- Node key file locations
- Hex content of each key
- File sizes and modification dates
- Instructions for viewing Peer IDs

## File Structure

```
velo-chain/
│
├── 📁 node-keys/                    ⚠️ CRITICAL - DO NOT COMMIT!
│   ├── alice-node-key              # Alice's Ed25519 private key (64-char hex)
│   └── bob-node-key                # Bob's Ed25519 private key (64-char hex)
│
├── 📁 chain-data/                   ⚠️ DO NOT COMMIT!
│   ├── alice/                      # Alice's blockchain database
│   │   ├── chains/
│   │   │   └── local_testnet/
│   │   │       ├── db/             # RocksDB blockchain state
│   │   │       └── keystore/       # Session keys
│   │   └── network/
│   │       └── secret_ed25519      # Network identity (derived from node-key)
│   │
│   └── bob/                        # Bob's blockchain database
│       └── (same structure as alice/)
│
├── alice-peer-id.txt                # Saved Alice peer ID (convenience)
│
├── 🔧 setup-node-keys-v2.bat        # Generate persistent node keys
├── 🚀 run-alice-final.bat           # Start Alice validator
├── 🚀 run-bob-final.bat             # Start Bob validator
├── 🔍 inspect-node-keys.bat         # View node key info
├── 🗑️  reset-testnet.bat            # Reset blockchain data
├── 📊 check-network-status.bat      # Monitor network status
│
├── 📖 NODE-MANAGEMENT-README.md     # This file
├── 📖 WORKING-SETUP-GUIDE.md        # Detailed technical guide
│
└── 📁 target/release/
    └── velo-node.exe               # Compiled Substrate node binary
```

### Git Ignore Configuration

**Add to `.gitignore`:**
```gitignore
# Node keys (NEVER commit these!)
node-keys/
alice-peer-id.txt

# Blockchain data
chain-data/

# Build artifacts
target/
```

## Understanding Peer IDs

### What is a Peer ID?

A **Peer ID** is a unique identifier for a node in the libp2p network (Substrate's networking layer).

**Format:** `12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp`

**Derivation:**
1. Start with node key (32-byte Ed25519 private key)
2. Derive public key from private key
3. Hash with multihash format
4. Encode with base58 → Peer ID

### Why Peer IDs Matter

**Without persistent node keys:**
```
Session 1: Alice is 12D3KooABC...
Session 2: Alice is 12D3KooDEF...  ❌ Different!
Session 3: Alice is 12D3KooGHI...  ❌ Different!
```
- Bootnodes can't find validators
- Network topology constantly changes
- Not suitable for production

**With persistent node keys (our solution):**
```
Session 1: Alice is 12D3KooABC...
Session 2: Alice is 12D3KooABC...  ✅ Same!
Session 3: Alice is 12D3KooABC...  ✅ Same!
```
- Stable network identity
- Reliable bootnode connections
- Production-ready

### How Our Scripts Help

| Problem | Our Solution |
|---------|-------------|
| Random peer IDs | Persistent node keys in files |
| Manual bootnode setup | Automated peer ID saving |
| Lost peer ID between sessions | `alice-peer-id.txt` storage |
| Complex key generation | One-click `setup-node-keys-v2.bat` |

## Security Best Practices

### Protecting Node Keys

**Node keys are like private keys - treat them as secrets!**

#### Development
```cmd
# OK for local testing
node-keys\alice-node-key
node-keys\bob-node-key
```

#### Production
```cmd
# Move to secure location
C:\ProgramData\VeloPay\keys\validator-01-key
C:\ProgramData\VeloPay\keys\validator-02-key

# Set restrictive permissions
icacls C:\ProgramData\VeloPay\keys /grant:r "SYSTEM:(OI)(CI)F" /inheritance:r
```

### What to Backup

| Item | Backup? | Frequency | Why |
|------|---------|-----------|-----|
| **node-keys/** | ✅ YES | After generation | Lost keys = lost identity |
| **chain-data/** | ✅ YES | Daily | Blockchain state |
| **alice-peer-id.txt** | ⚠️ Optional | N/A | Can regenerate from node key |
| **Scripts (.bat)** | ✅ YES | After changes | Recovery procedures |

### What NOT to Commit

**NEVER commit to version control:**
- ❌ `node-keys/` directory
- ❌ `alice-peer-id.txt`
- ❌ `chain-data/` directory
- ❌ Any file containing private keys

**Safe to commit:**
- ✅ Scripts (.bat files)
- ✅ Documentation (.md files)
- ✅ Chain specification (chain-spec-raw.json)
- ✅ Source code

## Common Issues and Solutions

### Issue: "Alice's node key not found"

**Error:**
```
[ERROR] Alice's node key not found!
Please run setup-node-keys-v2.bat first to generate node keys.
```

**Solution:**
```cmd
setup-node-keys-v2.bat
```

### Issue: Bob shows "0 peers"

**Symptoms:**
```
💤 Idle (0 peers), best: #0 (0x1234…5678)
```

**Checklist:**
1. ✅ Is Alice running? Check Alice's terminal
2. ✅ Is the peer ID correct? Compare with Alice's logs
3. ✅ Try re-entering peer ID:
   ```cmd
   del alice-peer-id.txt
   run-bob-final.bat
   ```
4. ✅ Check firewall (ports 30333, 30334)

### Issue: "Cannot connect" errors

**Error:**
```
Failed to create client: Cannot connect to ws://127.0.0.1:9944
```

**Solution:**
- Node isn't running - start it first
- Wrong port - Alice uses 9944, Bob uses 9945
- Firewall blocking - allow velo-node.exe

### Issue: Want different Peer IDs

**Scenario:** You want to regenerate Alice with a new identity.

**Solution:**
```cmd
# Delete old keys
rmdir /s /q node-keys

# Generate new ones
setup-node-keys-v2.bat

# Start fresh
run-alice-final.bat
```

### Issue: Disk space running low

**Symptoms:** `chain-data/` directory growing large.

**Solution:**
```cmd
# Stop nodes first (Ctrl+C in both terminals)

# Reset blockchain data but keep keys
reset-testnet.bat

# Restart
run-alice-final.bat
run-bob-final.bat
```

### Issue: "Timeout waiting for notification"

**Symptoms:** Bob can't connect even with correct peer ID.

**Solutions:**
1. Wait longer (first connection can take 30-60 seconds)
2. Check Alice is fully started (look for "Idle" in logs)
3. Restart both nodes

## Production Deployment

### Changes Needed for Production

| Setting | Development | Production |
|---------|-------------|------------|
| **Chain spec** | `--chain local` | `--chain /path/to/chain-spec-raw.json` |
| **RPC methods** | `--rpc-methods Unsafe` | `--rpc-methods Safe` |
| **RPC access** | `--rpc-external` | Remove (use nginx proxy) |
| **CORS** | `--rpc-cors all` | `--rpc-cors https://velopay.io` |
| **Validator identity** | `--alice` / `--bob` | `--name "Validator-01"` |
| **Bootnode** | `localhost` | Public IP or domain |
| **Data path** | `.\chain-data\alice` | `/var/lib/velopay/validator-01` |

### Production Checklist

#### Security
- [ ] Move node keys to secure location (e.g., HSM)
- [ ] Set restrictive file permissions
- [ ] Remove `--rpc-methods Unsafe`
- [ ] Configure firewall rules
- [ ] Use HTTPS for RPC (nginx reverse proxy)
- [ ] Enable CORS only for your domain

#### Monitoring
- [ ] Set up Prometheus metrics
- [ ] Configure Grafana dashboards
- [ ] Set up alert manager
- [ ] Monitor disk usage
- [ ] Track peer count

#### Operations
- [ ] Document runbooks
- [ ] Create backup scripts
- [ ] Test disaster recovery
- [ ] Plan upgrade procedures
- [ ] Set up logging aggregation

#### Network
- [ ] Use production chain spec
- [ ] Configure proper bootnodes
- [ ] Set up sentry nodes
- [ ] Plan validator rotation
- [ ] Test failover

### Example Production Script

```batch
@echo off
REM Production Validator Script

C:\VeloPay\velo-node.exe ^
  --base-path C:\VeloPay\data\validator-01 ^
  --chain C:\VeloPay\config\chain-spec-raw.json ^
  --name "VeloPay-Validator-01" ^
  --node-key-file C:\ProgramData\VeloPay\keys\validator-01-key ^
  --validator ^
  --port 30333 ^
  --rpc-port 9944 ^
  --rpc-cors https://velopay.io ^
  --rpc-methods Safe ^
  --prometheus-port 9615 ^
  --bootnodes /dns/bootnode.velopay.io/tcp/30333/p2p/12D3Koo... ^
  --telemetry-url "wss://telemetry.polkadot.io/submit/ 0"
```

## Contributing

### Improving Scripts

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Test thoroughly on Windows
5. Submit a pull request

### Testing Checklist

- [ ] Runs on clean Windows 10 installation
- [ ] Runs on Windows 11
- [ ] Works with/without curl installed
- [ ] Handles missing files gracefully
- [ ] Clear error messages
- [ ] Preserves data when appropriate

### Code Style

- Use `REM` for comments
- Add blank lines for readability
- Include `echo.` for spacing in output
- Always check error levels (`%ERRORLEVEL%`)
- Provide helpful error messages

## License

Apache-2.0

## Additional Resources

- **Substrate Documentation:** https://docs.substrate.io/
- **Polkadot Wiki:** https://wiki.polkadot.network/
- **VeloPay Main README:** ../README.md
- **Technical Setup Guide:** WORKING-SETUP-GUIDE.md

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review WORKING-SETUP-GUIDE.md for technical details
3. Open an issue on GitHub
4. Contact the VeloPay development team

---

**Made with ❤️ for the VeloPay project**

**Version:** 2.0
**Last Updated:** 2025
**Platform:** Windows 10/11
