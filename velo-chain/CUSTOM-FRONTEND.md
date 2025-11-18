# Building a Custom Frontend for VelocityPay

## What is VelocityPay?

VelocityPay is a **custom independent blockchain** with:
- **Consensus**: Proof of Authority (PoA) using Aura + GRANDPA
- **Custom Pallets**: VelocityPay (stablecoin), KYC, Compliance
- **Architecture**: Substrate-based but completely independent
- **Not connected to Polkadot** - this is your own sovereign chain

## Why NOT Polkadot.js Apps?

Polkadot.js Apps is just a **generic development tool** for Substrate chains. For production, you need:
- Your own branded interface
- Custom business logic and workflows
- Integration with your payment systems
- Mobile apps, web apps, APIs

## Architecture Options

### Option 1: REST API Gateway (Recommended for Production)

Build a backend service that wraps the blockchain:

```
[Your Frontend] → [REST API] → [VelocityPay Node]
   (React/Vue)      (Node.js)      (Rust/Substrate)
```

**Benefits**:
- Simpler frontend development
- Can add business logic layer
- Better security (hide blockchain complexity)
- Cache frequently used data

**Tech Stack**:
- Backend: Node.js, Python FastAPI, or Go
- Use substrate-api libraries
- Expose REST/GraphQL endpoints

### Option 2: Direct WebSocket Connection

Frontend connects directly to node via WebSocket:

```
[Your Frontend] ←WebSocket→ [VelocityPay Node]
   (JavaScript)                  (ws://localhost:9944)
```

**Benefits**:
- Real-time updates
- Direct blockchain interaction
- Lower latency

**Tech Stack**:
- Use @polkadot/api JavaScript library
- Works in browser or Node.js
- WebSocket connection to your node

### Option 3: Hybrid Approach

Combine both for best of both worlds:

```
[Frontend] → [REST API for reads] → [Cache/DB]
           → [WebSocket for writes] → [Node]
```

## Quick Start: JavaScript Frontend

### 1. Install Dependencies

```bash
npm init -y
npm install @polkadot/api
```

### 2. Connect to Your Chain

```javascript
// connect.js
const { ApiPromise, WsProvider } = require('@polkadot/api');

async function main() {
  // Connect to local VelocityPay node
  const provider = new WsProvider('ws://127.0.0.1:9944');
  const api = await ApiPromise.create({ provider });

  // Get chain info
  const chain = await api.rpc.system.chain();
  const lastHeader = await api.rpc.chain.getHeader();

  console.log(`Connected to: ${chain}`);
  console.log(`Latest block: ${lastHeader.number}`);

  // Query VelocityPay pallet
  // const balance = await api.query.velocitypay.balances(accountId);

  // Call VelocityPay functions
  // const tx = api.tx.velocitypay.transfer(recipient, amount);
}

main().catch(console.error);
```

### 3. Build Your UI

```javascript
// Example: React component for VelocityPay
import { useState, useEffect } from 'react';
import { ApiPromise, WsProvider } from '@polkadot/api';

function VelocityPayApp() {
  const [api, setApi] = useState(null);
  const [blockNumber, setBlockNumber] = useState(0);

  useEffect(() => {
    async function connect() {
      const provider = new WsProvider('ws://127.0.0.1:9944');
      const api = await ApiPromise.create({ provider });
      setApi(api);

      // Subscribe to new blocks
      await api.rpc.chain.subscribeNewHeads((header) => {
        setBlockNumber(header.number.toNumber());
      });
    }
    connect();
  }, []);

  return (
    <div>
      <h1>VelocityPay Dashboard</h1>
      <p>Current Block: {blockNumber}</p>
      {/* Your custom UI here */}
    </div>
  );
}
```

## Direct RPC Testing (No Libraries)

You can test your chain with just curl or any HTTP client:

### Get Chain Info
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "system_chain"}' \
  http://localhost:9944
```

### Get Latest Block
```bash
curl -H "Content-Type: application/json" \
  -d '{"id":1, "jsonrpc":"2.0", "method": "chain_getHeader"}' \
  http://localhost:9944
```

### Subscribe to New Blocks (WebSocket)
```javascript
const ws = new WebSocket('ws://localhost:9944');

ws.onopen = () => {
  ws.send(JSON.stringify({
    id: 1,
    jsonrpc: '2.0',
    method: 'chain_subscribeNewHeads',
    params: []
  }));
};

ws.onmessage = (event) => {
  const data = JSON.parse(event.data);
  console.log('New block:', data);
};
```

## Available RPC Methods

Your VelocityPay node exposes these RPC endpoints:

### System Methods
- `system_chain` - Get chain name
- `system_version` - Get node version
- `system_health` - Get node health status
- `system_peers` - Get connected peers
- `system_properties` - Get chain properties

### Chain Methods
- `chain_getHeader` - Get latest block header
- `chain_getBlock` - Get block by hash
- `chain_subscribeNewHeads` - Subscribe to new blocks
- `chain_getFinalizedHead` - Get finalized block

### State Methods
- `state_getStorage` - Query storage
- `state_getMetadata` - Get runtime metadata
- `state_call` - Call runtime API

### Author Methods (Transactions)
- `author_submitExtrinsic` - Submit transaction
- `author_pendingExtrinsics` - Get pending transactions

### Custom Pallet Methods
Your VelocityPay, KYC, and Compliance pallets are accessible via:
- `state_call` with pallet name and method
- Submit extrinsics via `author_submitExtrinsic`

## Production Deployment

For a production VelocityPay system:

1. **Multiple Validator Nodes** (PoA authorities)
2. **RPC Nodes** (for public access, non-validating)
3. **Custom API Gateway** (your backend service)
4. **Web/Mobile Apps** (your branded frontend)
5. **Database** (for indexing and caching blockchain data)

```
[Users] → [Web/Mobile App] → [API Gateway] → [RPC Nodes]
                                               ↓
                                          [Validator Nodes]
```

## Testing Without Any UI

Just use the scripts I created:

```bash
# Start your chain
run-dev.bat

# Test via RPC
test-rpc.bat

# Or use Python
python test-chain.py
```

## Summary

**Key Points**:
1. VelocityPay is YOUR custom blockchain (not Polkadot)
2. PoA consensus = you control the validators
3. Polkadot.js Apps is optional (just for testing)
4. Build your own frontend with your branding
5. Connect via WebSocket (ws://localhost:9944) or HTTP RPC
6. Use any programming language (JavaScript, Python, Go, Rust)

**You don't need Polkadot anything** - just connect to your node's RPC endpoint and build whatever interface you want!
