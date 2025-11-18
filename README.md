# Velo Pay

Professional blockchain-based payment system with fiat-backed stablecoin (VCS).

## System Architecture

### 1. Velo Chain (Substrate Blockchain)
Custom blockchain built on Substrate framework with PoA consensus.

**Location**: `velo-chain/`

**Components Implemented**:
- **VeloPay Pallet** (`pallets/velopay/`) - Core stablecoin logic
  - Mint/burn mechanisms with authority control
  - Request-based minting (requires approval)
  - Reserved burn system (locks tokens before burning)
  - Transaction fee configuration (basis points)
  - Emergency pause/unpause functionality
  - Complete audit logging
  - Total supply tracking

- **KYC Pallet** (`pallets/kyc/`) - Identity verification
  - KYC submission with document hashing
  - Verification workflow (pending/verified/rejected)
  - KYC verifier role management
  - Integration with VeloPay pallet

- **Compliance Pallet** (`pallets/compliance/`) - AML/regulatory
  - Account flagging system
  - Suspicious activity alerts
  - Transaction limits and monitoring
  - Compliance officer role
  - Alert resolution workflow

- **Runtime** (`runtime/`) - Blockchain runtime configuration
  - Integrated all custom pallets
  - Configured Aura (PoA) consensus
  - GRANDPA finality gadget
  - Multi-signature support
  - Utility batch operations
  - Transaction payment system

- **Node** (`node/`) - Blockchain node implementation
  - Chain specification (dev/local/production)
  - Service layer with full node capabilities
  - RPC server for external interactions
  - CLI with standard Substrate commands

### 2. API Gateway (Actix-web + Subxt)
RESTful API backend connecting frontend to blockchain.

**Location**: `velopay-api/`

**Components Implemented**:
- Configuration management (`src/config.rs`)
- Data models:
  - User model with authentication
  - Transaction tracking
  - Mint request workflow
  - Burn request workflow
  - KYC submission data
- Chain client setup using Subxt
- Basic server with health check endpoints

**Components To Be Implemented**:
- Services layer:
  - `src/services/wallet.rs` - Wallet operations, balance queries
  - `src/services/payment.rs` - Payment processing, fee estimation
  - `src/services/mint.rs` - Mint request creation and approval
  - `src/services/burn.rs` - Burn request creation and approval
  - `src/services/kyc.rs` - KYC submission and verification
  - `src/services/analytics.rs` - Statistics and reporting
  - `src/services/auth.rs` - JWT authentication

- Routes layer:
  - `src/routes/wallet.rs` - Wallet endpoints
  - `src/routes/payment.rs` - Payment endpoints
  - `src/routes/mint.rs` - Mint endpoints
  - `src/routes/burn.rs` - Burn endpoints
  - `src/routes/kyc.rs` - KYC endpoints
  - `src/routes/admin.rs` - Admin endpoints
  - `src/routes/analytics.rs` - Analytics endpoints

- Middleware:
  - `src/middleware/auth.rs` - JWT validation
  - `src/middleware/rate_limit.rs` - Rate limiting
  - `src/middleware/logging.rs` - Request logging

- Database:
  - PostgreSQL schema migrations
  - Database connection pool
  - Repository layer for data access

### 3. Frontend (SvelteKit)
Modern web application for users and administrators.

**Location**: `velopay-web/` (Not yet created)

**Pages To Be Implemented**:
- Landing page
- User authentication (login/register)
- Wallet dashboard
  - Balance display
  - Transaction history
  - Send/receive VCS
- Mint/burn requests
  - Create new requests
  - View request history
  - Request status tracking
- Block explorer
  - Browse blocks
  - View transactions
  - Search functionality
- Admin dashboard
  - Pending mint/burn approvals
  - KYC verification queue
  - System statistics
  - Validator management
- KYC submission portal
  - Document upload
  - Status tracking

**Components To Be Implemented**:
- API client library
- State management stores
- Reusable UI components
- Form validation
- WebSocket for real-time updates

## Technology Stack

**Blockchain**:
- Substrate (Polkadot SDK stable2407 branch)
- Rust programming language
- Aura consensus (PoA)
- GRANDPA finality

**Backend API**:
- Actix-web 4.x
- Subxt (Substrate client)
- PostgreSQL database
- SQLx for database access
- JWT authentication
- bcrypt password hashing

**Frontend** (Planned):
- SvelteKit
- TypeScript
- Tailwind CSS
- Polkadot.js API

## Build Instructions

### Prerequisites

**Linux/macOS**:
- Rust 1.70+ with nightly toolchain
- Node.js 18+ (for frontend)
- PostgreSQL 14+
- Git
- Build essentials (gcc, clang, make)
- OpenSSL development libraries

**Windows 10/11**:
- Rust 1.70+ with nightly toolchain
- Node.js 18+ (for frontend)
- PostgreSQL 14+
- Git
- Visual Studio Build Tools with C++ workload
- Set `OPENSSL_VENDORED=1` environment variable before building

### Install Rust (All Platforms)

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
rustup target add wasm32-unknown-unknown
```

**On Windows** (PowerShell):
```powershell
# Download and run rustup-init.exe from https://rustup.rs/
rustup target add wasm32-unknown-unknown
```

### Build Velo Chain

**On Linux/macOS**:
```bash
cd velo-chain

rustup target add wasm32-unknown-unknown

cargo build --release

./target/release/velo-node --dev
```

**On Windows**:
```powershell
cd velo-chain

rustup target add wasm32-unknown-unknown
rustup component add rust-src

# Set environment variable for vendored OpenSSL
$env:OPENSSL_VENDORED = "1"

cargo build --release

# Run with helper script
.\quick-start.bat

# Or run manually
.\target\release\velo-node.exe --chain local --alice --tmp --rpc-external --rpc-cors all
```

**Build time**: 15-45 minutes on first build

**See `velo-chain/BUILD.md` for detailed build instructions and troubleshooting.**

### Build API Gateway

```bash
cd velopay-api

cp .env.example .env

cargo build --release

cargo run
```

### Run Development Node

**Linux/macOS**:
```bash
cd velo-chain
cargo run --release -- --dev --tmp
```

**Windows**:
```powershell
cd velo-chain
$env:OPENSSL_VENDORED = "1"
cargo run --release -- --dev --tmp
```

### Create Production Chain Spec

```bash
./target/release/velo-node build-spec --chain local --disable-default-bootnode > chain-spec.json

./target/release/velo-node build-spec --chain chain-spec.json --raw > chain-spec-raw.json
```

## Database Setup

```sql
CREATE DATABASE velopay;

CREATE TYPE transaction_status AS ENUM ('pending', 'confirmed', 'failed');
CREATE TYPE mint_request_status AS ENUM ('pending', 'approved', 'rejected', 'completed');
CREATE TYPE burn_request_status AS ENUM ('pending', 'reserved', 'approved', 'rejected', 'completed');
CREATE TYPE kyc_status AS ENUM ('notsubmitted', 'pending', 'verified', 'rejected');

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    wallet_address VARCHAR(66) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE transactions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    from_address VARCHAR(66) NOT NULL,
    to_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    fee VARCHAR(40) NOT NULL,
    transaction_hash VARCHAR(66),
    block_number BIGINT,
    status transaction_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE mint_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    bank_reference VARCHAR(256) NOT NULL,
    status mint_request_status NOT NULL DEFAULT 'pending',
    chain_request_id BIGINT,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE burn_requests (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    amount VARCHAR(40) NOT NULL,
    bank_account VARCHAR(256) NOT NULL,
    status burn_request_status NOT NULL DEFAULT 'pending',
    chain_request_id BIGINT,
    approved_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE TABLE kyc_submissions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES users(id),
    wallet_address VARCHAR(66) NOT NULL,
    document_hash VARCHAR(128) NOT NULL,
    full_name VARCHAR(255) NOT NULL,
    date_of_birth TIMESTAMP WITH TIME ZONE NOT NULL,
    country VARCHAR(2) NOT NULL,
    status kyc_status NOT NULL DEFAULT 'notsubmitted',
    verified_by UUID REFERENCES users(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_transactions_from ON transactions(from_address);
CREATE INDEX idx_transactions_to ON transactions(to_address);
CREATE INDEX idx_transactions_hash ON transactions(transaction_hash);
CREATE INDEX idx_mint_requests_user ON mint_requests(user_id);
CREATE INDEX idx_mint_requests_status ON mint_requests(status);
CREATE INDEX idx_burn_requests_user ON burn_requests(user_id);
CREATE INDEX idx_burn_requests_status ON burn_requests(status);
CREATE INDEX idx_kyc_user ON kyc_submissions(user_id);
CREATE INDEX idx_kyc_wallet ON kyc_submissions(wallet_address);
```

## API Endpoints (Planned)

### Authentication
- `POST /api/v1/auth/register` - Create new user account
- `POST /api/v1/auth/login` - Login and get JWT token

### Wallet
- `GET /api/v1/wallet/{address}/balance` - Get VCS balance
- `GET /api/v1/wallet/{address}/transactions` - Transaction history
- `POST /api/v1/wallet/create` - Create new wallet

### Payments
- `POST /api/v1/payment/send` - Send VCS to another address
- `GET /api/v1/payment/{tx_hash}` - Get transaction details
- `GET /api/v1/payment/estimate-fee` - Calculate transaction fee

### Mint/Burn
- `POST /api/v1/mint/request` - Request VCS minting
- `GET /api/v1/mint/requests` - List user mint requests
- `POST /api/v1/mint/approve/{id}` - Admin approve mint
- `POST /api/v1/burn/request` - Request VCS burning
- `GET /api/v1/burn/requests` - List user burn requests
- `POST /api/v1/burn/approve/{id}` - Admin approve burn

### KYC
- `POST /api/v1/kyc/submit` - Submit KYC documents
- `GET /api/v1/kyc/status/{address}` - Check KYC status
- `POST /api/v1/kyc/verify/{address}` - Admin verify KYC

### Analytics
- `GET /api/v1/stats/supply` - Total VCS supply
- `GET /api/v1/stats/transactions` - Transaction statistics
- `GET /api/v1/stats/volume` - Trading volume
- `GET /api/v1/stats/users` - User statistics

### Admin
- `GET /api/v1/admin/validators` - List validator nodes
- `POST /api/v1/admin/pause` - Emergency pause system
- `POST /api/v1/admin/unpause` - Unpause system
- `GET /api/v1/admin/audit-log` - View audit trail

## Token Economics

**Token**: VCS (Velo Cash)
**Type**: Fiat-backed Stablecoin
**Peg**: 1 VCS = 1 USD
**Decimals**: 12 (Substrate standard)
**Transaction Fee**: 0.1% (configurable)
**Minimum Transfer**: 0.01 VCS
**Maximum Transfer**: 1,000,000 VCS per transaction
**Daily Limit**: 10,000,000 VCS per user

## Security Features

**Chain Level**:
- Proof of Authority consensus (trusted validators only)
- Multi-signature support for critical operations
- Emergency pause mechanism
- KYC gating for all mint/burn operations
- Compliance monitoring and alerts

**API Level**:
- JWT authentication with expiration
- Password hashing with bcrypt
- Rate limiting (100 requests/minute default)
- Input validation on all endpoints
- SQL injection protection via parameterized queries

**Operational**:
- 1:1 USD reserve backing
- Regular reserve audits
- AML transaction monitoring
- Comprehensive audit logging
- Regulatory reporting capabilities

## Deployment

### Validator Node Setup

```bash
./velo-node \
  --chain chain-spec-raw.json \
  --base-path /data/velo \
  --port 30333 \
  --rpc-port 9933 \
  --ws-port 9944 \
  --validator \
  --name "Validator-1" \
  --rpc-cors all \
  --unsafe-rpc-external \
  --unsafe-ws-external
```

### API Gateway Deployment

```bash
docker build -t velopay-api .
docker run -d \
  --name velopay-api \
  -p 8080:8080 \
  --env-file .env \
  velopay-api
```

## Monitoring

**Prometheus Metrics**: Available on validator nodes
**Grafana Dashboards**: For real-time monitoring
**Alert Manager**: For system health alerts
**Database Monitoring**: PostgreSQL performance metrics

## License

Apache-2.0

## Development Status

**Completed**:
- ✅ Complete Substrate blockchain implementation
- ✅ Custom pallets (VeloPay, KYC, Compliance)
- ✅ Runtime configuration
- ✅ Node service and chain specification
- ✅ API Gateway project structure
- ✅ Data models and configuration

**In Progress**:
- 🔄 API services and routes implementation
- 🔄 Middleware (authentication, rate limiting)
- 🔄 Database integration

**Pending**:
- ⏳ Frontend SvelteKit application
- ⏳ Block explorer functionality
- ⏳ Admin dashboard
- ⏳ Comprehensive testing suite
- ⏳ Production deployment configuration
- ⏳ Documentation and API specs

## Additional Files Needed for Complete Implementation

The following files need to be created to complete the system:

**API Services** (`velopay-api/src/services/`):
- auth.rs - JWT generation and validation
- wallet.rs - Wallet operations via Subxt
- payment.rs - Payment transaction submission
- mint.rs - Mint request management
- burn.rs - Burn request management
- kyc.rs - KYC submission and verification
- analytics.rs - System statistics

**API Routes** (`velopay-api/src/routes/`):
- mod.rs - Routes module exports
- auth.rs - Auth endpoints
- wallet.rs - Wallet endpoints
- payment.rs - Payment endpoints
- mint.rs - Mint endpoints
- burn.rs - Burn endpoints
- kyc.rs - KYC endpoints
- admin.rs - Admin endpoints

**API Middleware** (`velopay-api/src/middleware/`):
- mod.rs - Middleware module exports
- auth.rs - JWT authentication middleware
- rate_limit.rs - Rate limiting implementation
- logging.rs - Request/response logging

**Frontend Application** (`velopay-web/`):
- Complete SvelteKit project structure
- All pages and components
- API client integration
- WebSocket real-time updates

**Deployment**:
- Dockerfile for API gateway
- Docker Compose for full stack
- Kubernetes manifests
- Monitoring and logging configuration

**Testing**:
- Unit tests for all pallets
- Integration tests for API
- End-to-end tests for frontend
- Load testing scripts

**Documentation**:
- API specification (OpenAPI/Swagger)
- User guides
- Admin manual
- Deployment guides
