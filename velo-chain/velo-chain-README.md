# Velo Chain - Substrate Blockchain Node

The blockchain layer of VeloPay - a custom Substrate-based chain with specialized pallets for payment operations.

---

## 🏗️ Architecture

### Pallets

- **VeloPay Pallet**: Core payment logic with mint/burn operations
  - Coin transfers
  - Mint requests and approvals
  - Burn requests and processing
  - Balance management

### Runtime Configuration

- **Consensus**: Aura (block production) + GRANDPA (finality)
- **Block Time**: 6 seconds
- **Coin**: VCS (VeloCash)
- **Decimal Places**: 12

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- Substrate dependencies

### Build

```bash
cargo build --release
```

### Run Development Node

```bash
# Single node (Alice)
./target/release/velo-chain --dev --tmp

# Or use batch script
start-local.bat  # Windows
./start-local.sh # Linux/Mac
```

### Run Multi-Node Network

**Terminal 1 (Alice):**
```bash
./target/release/velo-chain \
  --base-path /tmp/alice \
  --chain local \
  --alice \
  --port 30333 \
  --rpc-port 9944 \
  --node-key 0000000000000000000000000000000000000000000000000000000000000001 \
  --validator
```

**Terminal 2 (Bob):**
```bash
./target/release/velo-chain \
  --base-path /tmp/bob \
  --chain local \
  --bob \
  --port 30334 \
  --rpc-port 9945 \
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp \
  --validator
```

---

## 🧪 Testing

```bash
# Run all tests
cargo test

# Run specific pallet tests
cargo test -p pallet-velopay
```

---

## 📝 Chain Specification

Located in `node/src/chain_spec.rs`:
- Genesis configuration
- Initial balances
- Validator set
- Runtime parameters

---

## 🔧 Configuration

### Genesis Accounts (Development)

- **Alice**: `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`
- **Bob**: `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`

### Ports

- **RPC**: 9944 (Alice), 9945 (Bob)
- **WS**: Same as RPC
- **P2P**: 30333 (Alice), 30334 (Bob)

---

## 🔌 Integration

### Connect via Polkadot.js

```javascript
import { ApiPromise, WsProvider } from '@polkadot/api';

const wsProvider = new WsProvider('ws://127.0.0.1:9944');
const api = await ApiPromise.create({ provider: wsProvider });
```

### Connect via Subxt (Rust)

```rust
use subxt::{OnlineClient, PolkadotConfig};

let api = OnlineClient::<PolkadotConfig>::new().await?;
```

---

## 📚 VeloPay Pallet API

### Extrinsics

- `transfer(dest, amount)` - Transfer coins
- `request_mint(amount, bank_reference)` - Request coin minting
- `approve_mint(request_id)` - Approve mint request (authority only)
- `request_burn(amount, bank_account)` - Request coin burning
- `approve_burn(request_id)` - Approve burn request (authority only)

### Storage

- `Balances` - Account balances
- `MintRequests` - Pending mint requests
- `BurnRequests` - Pending burn requests
- `TotalSupply` - Current coin supply

### Events

- `Transfer(from, to, amount)`
- `MintRequested(account, amount)`
- `MintApproved(request_id)`
- `BurnRequested(account, amount)`
- `BurnApproved(request_id)`

---

## 🛠️ Development

### Add New Pallet

1. Create pallet in `pallets/`
2. Add to `runtime/Cargo.toml`
3. Configure in `runtime/src/lib.rs`
4. Build and test

### Modify Existing Pallet

1. Edit pallet code in `pallets/velopay/src/lib.rs`
2. Update tests
3. Rebuild: `cargo build --release`

---

## 📖 Resources

- [Substrate Documentation](https://docs.substrate.io)
- [Polkadot Wiki](https://wiki.polkadot.network)
- [Rust Book](https://doc.rust-lang.org/book/)

---

**Part of VeloPay Project**
