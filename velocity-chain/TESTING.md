# VelocityPay Blockchain Testing Guide

## Quick Start

### Option 1: Development Mode (Simplest)
Run a temporary single-node chain for quick testing:
```bash
run-dev.bat
```
- Auto-starts with Alice as validator
- Temporary storage (deleted on exit)
- RPC at http://localhost:9944
- Best for quick testing and development

### Option 2: Persistent Single Node
Run a persistent single-validator chain:
```bash
run-alice.bat
```
- Data stored in `./chain-data/alice`
- RPC at http://localhost:9944
- Chain persists between restarts

### Option 3: Two-Node Network
Run a local network with 2 validators:

Terminal 1:
```bash
run-alice.bat
```

Terminal 2:
```bash
run-bob.bat
```
- Alice RPC: http://localhost:9944
- Bob RPC: http://localhost:9945
- Simulates a real network environment

## Testing Your Chain

### Direct RPC Testing (Recommended)

Use the provided test scripts to interact with your chain directly:

```bash
# Test all RPC endpoints
test-rpc.bat

# Or use Python
python test-chain.py
```

### Manual RPC Calls

Check chain info:
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' http://localhost:9944
```

Get latest block:
```bash
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getBlock"}' http://localhost:9944
```

## Test Accounts

Your development chain comes with pre-funded test accounts:

- **Alice**: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
- **Bob**: 5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty
- **Charlie**: 5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y
- **Dave**: 5DAAnrj7VHTznn2AWBemMuyBwZWs6FNFjdyVXUeYum3PTXFy
- **Eve**: 5HGjWAeFDfFCWPsjFQdVV2Msvz2XtMktvgocEZcCj68kUMaw
- **Ferdie**: 5CiPPseXPECbkjWCa6MnjNokrgYjMqmKndv2rSnekmSK2DjL

All accounts start with 1,000,000 tokens.

### Optional: Using Polkadot.js Apps (Generic UI)

**Note**: Polkadot.js Apps is just a generic Substrate blockchain explorer - VelocityPay is NOT part of Polkadot. This is purely an optional development tool.

If you want a visual interface for testing:
1. Open https://polkadot.js.org/apps
2. Click top-left → "Development" → "Local Node"
3. Connect to `ws://127.0.0.1:9944`

**For production**: Build your own custom frontend (see CUSTOM-FRONTEND.md)

## Testing Custom Pallets

### VelocityPay Pallet
Your stablecoin functionality. Test:
- Minting stablecoins
- Transferring stablecoins
- Burning stablecoins
- Checking balances

### KYC Pallet
Know Your Customer verification. Test:
- Adding KYC records
- Verifying accounts
- Querying KYC status

### Compliance Pallet
Transaction compliance checking. Test:
- Setting compliance rules
- Checking transaction compliance
- Monitoring compliance events

## Useful Commands

### Check Node Status
```bash
# Get node health
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_health"}' http://localhost:9944

# Get node version
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_version"}' http://localhost:9944

# Get peers (for multi-node setup)
curl -H "Content-Type: application/json" -d '{"id":1, "jsonrpc":"2.0", "method": "system_peers"}' http://localhost:9944
```

### View Logs
The node will output logs to the console. Look for:
- ✅ Block production: `Prepared block for proposing`
- ✅ Block finalization: `Finalized block`
- ✅ Peer connections: `Discovered new external address`
- ❌ Errors: Watch for `ERROR` messages

### Clean Chain Data
To start fresh (removes all blocks and state):
```bash
target\release\velocity-node purge-chain --dev
```

Or manually delete:
```bash
rmdir /s /q chain-data
```

## Common Issues

### Port Already in Use
If you get "Address already in use":
1. Check if another node is running: `netstat -ano | findstr :9944`
2. Kill the process or use different ports with `--rpc-port` and `--port`

### Node Not Producing Blocks
- Check if you're running in `--validator` mode
- Verify keystore has the right keys (`--alice`, `--bob`, etc.)
- Check logs for errors

### Can't Connect with Polkadot.js
- Ensure `--rpc-external` flag is set
- Check firewall isn't blocking port 9944
- Verify the node is running: `curl http://localhost:9944`

### Chain Stuck
- Try restarting with fresh data: `purge-chain`
- Check system resources (RAM, disk space)
- Review logs for errors

## Next Steps

1. **Start the chain** with one of the run scripts
2. **Connect via Polkadot.js Apps** at https://polkadot.js.org/apps
3. **Test transfers** between Alice and Bob
4. **Interact with custom pallets** through the Extrinsics tab
5. **Monitor events** in the Network → Explorer tab
6. **Check storage** in the Developer → Chain State tab

## Production Deployment

When ready for production:
1. Generate new chain spec: `velocity-node build-spec --chain dev > custom-spec.json`
2. Edit `custom-spec.json` with your validators and config
3. Convert to raw: `velocity-node build-spec --chain custom-spec.json --raw > custom-spec-raw.json`
4. Run nodes with: `--chain custom-spec-raw.json`
5. Remove `--dev`, `--tmp`, `--alice` flags
6. Use proper validator keys
7. Configure proper bootnodes and network settings

## Resources

- Substrate Documentation: https://docs.substrate.io
- Polkadot.js Apps: https://polkadot.js.org/apps
- Polkadot.js API Docs: https://polkadot.js.org/docs/api

---

**Note**: The trie-db v0.29.1 warning during build is a future-compatibility notice and does not affect current functionality. This will be resolved when Polkadot SDK updates their dependencies.
