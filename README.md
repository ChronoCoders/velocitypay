# VeloPay - Blockchain Payment System

![VeloPay](https://img.shields.io/badge/VeloPay-Blockchain%20Payment-blue)
![Substrate](https://img.shields.io/badge/Substrate-4.0-green)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange)
![License](https://img.shields.io/badge/license-MIT-blue)

**VeloPay** is a fully-fledged blockchain-based payment system built on Substrate, enabling secure digital payments with fiat on/off ramp capabilities through mint and burn operations.

---

## 🚀 Features

### 💸 Core Payment Features
- **Peer-to-Peer Transfers**: Send and receive VCS instantly
- **Transaction History**: Complete audit trail of all transactions
- **Multi-signature Support**: Enhanced security for high-value transactions

### 🏦 Fiat On/Off Ramp
- **Mint Operations**: Convert fiat to VCS through verified bank transfers
- **Burn Operations**: Convert VCS back to fiat currency
- **Compliance Framework**: Built-in KYC/AML verification system

### 🔐 Security & Compliance
- **KYC Verification**: Identity verification for regulatory compliance
- **Admin Controls**: Centralized oversight for mint/burn approvals
- **JWT Authentication**: Secure API access with coin-based auth
- **Rate Limiting**: DDoS protection and abuse prevention

### 🎯 Technical Highlights
- **Substrate Framework**: Built on Polkadot's battle-tested blockchain framework
- **RESTful API**: Comprehensive HTTP API for easy integration
- **PostgreSQL Database**: Robust data persistence layer
- **Real-time Updates**: WebSocket support for live transaction monitoring

---

## 📁 Project Structure

```
velopay/
├── velo-chain/              # Substrate blockchain node
│   ├── pallets/
│   │   └── velopay/         # VeloPay pallet (core logic)
│   ├── runtime/             # Chain runtime configuration
│   └── node/                # Node implementation
│
├── velopay-api/             # REST API Gateway (Actix-web + Rust)
│   ├── src/
│   │   ├── chain/           # Blockchain client integration
│   │   ├── db/              # Database repositories
│   │   ├── middleware/      # Auth & admin middleware
│   │   ├── models/          # Data models
│   │   ├── routes/          # API endpoints
│   │   └── services/        # Business logic
│   └── migrations/          # Database migrations
│
└── velopay-web/             # Frontend (SvelteKit)
    ├── src/
    │   ├── lib/             # Shared components & utilities
    │   └── routes/          # Application pages
    └── static/              # Static assets
```

---

## 🛠️ Technology Stack

### Blockchain (velo-chain)
- **Framework**: Substrate 4.0
- **Language**: Rust
- **Consensus**: Aura (Block Production) + GRANDPA (Finality)
- **Runtime**: Custom VeloPay pallet with mint/burn logic

### API Gateway (velopay-api)
- **Framework**: Actix-web 4.x
- **Language**: Rust
- **Database**: PostgreSQL 16
- **Authentication**: JWT (jsonwebcoin)
- **Rate Limiting**: Governor middleware
- **Client**: Subxt for blockchain interaction

### Frontend (velopay-web)
- **Framework**: SvelteKit
- **Language**: TypeScript
- **Styling**: TailwindCSS
- **Build Tool**: Vite
- **State Management**: Svelte stores

---

## 📋 Prerequisites

### System Requirements
- **Operating System**: Windows 10/11, Linux, or macOS
- **RAM**: Minimum 8GB (16GB recommended)
- **Disk Space**: 20GB+ for blockchain data

### Software Dependencies
- **Rust**: 1.70 or higher
- **Node.js**: 18.x or higher
- **PostgreSQL**: 14.x or higher
- **Git**: Latest version

### Development Tools
- **Cargo**: Rust package manager (comes with Rust)
- **npm/pnpm**: Node package manager
- **psql**: PostgreSQL client

---

## 🚀 Quick Start

### 1. Clone Repository

```bash
git clone https://github.com/ChronoCoders/velopay.git
cd velopay
```

### 2. Start Blockchain Node

```bash
cd velo-chain

# Build the node (first time only)
cargo build --release

# Start Alice node (development)
./target/release/velo-chain --dev --tmp
```

**Alternative**: Use provided batch scripts:
```bash
# Windows
start-local.bat

# Linux/Mac
./start-local.sh
```

### 3. Setup Database

```bash
# Create database
psql -U postgres
CREATE DATABASE velopay_db;
CREATE USER velopay_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE velopay_db TO velopay_user;
\q

# Run migrations
cd velopay-api
sqlx migrate run
```

### 4. Configure API

Create `.env` file in `velopay-api/`:

```env
# Database
DATABASE_URL=postgresql://velopay_user:your_password@localhost:5432/velopay_db

# Server
HOST=127.0.0.1
PORT=8080

# JWT
JWT_SECRET=your-super-secret-jwt-key-min-48-chars
JWT_EXPIRATION=86400

# Admin
ADMIN_API_KEY=your-admin-api-key
ADMIN_SEED=//Alice

# Chain
CHAIN_ENDPOINT=ws://127.0.0.1:9944

# CORS
CORS_ALLOWED_ORIGINS=http://localhost:5173,http://localhost:3000
```

### 5. Start API Server

```bash
cd velopay-api
cargo run --release
```

API will be available at: `http://localhost:8080`

### 6. Start Frontend

```bash
cd velopay-web
npm install
npm run dev
```

Frontend will be available at: `http://localhost:5173`

---

## 📚 API Documentation

### Authentication Endpoints

#### Register User
```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "wallet_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
}
```

#### Login
```bash
POST /api/v1/auth/login
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePass123!"
}
```

### Payment Endpoints

#### Send Payment
```bash
POST /api/v1/payments
Authorization: Bearer <JWT_TOKEN>
Content-Type: application/json

{
  "from_address": "5GrwvaEF...",
  "to_address": "5FHneW46...",
  "amount": "100.50"
}
```

#### Get Transaction History
```bash
GET /api/v1/payments/history/{wallet_address}?limit=10&offset=0
Authorization: Bearer <JWT_TOKEN>
```

### Mint/Burn Operations

#### Create Mint Request
```bash
POST /api/v1/mint
Authorization: Bearer <JWT_TOKEN>
Content-Type: application/json

{
  "wallet_address": "5GrwvaEF...",
  "amount": "1000.00",
  "bank_reference": "BANK-REF-12345"
}
```

#### Create Burn Request
```bash
POST /api/v1/burn
Authorization: Bearer <JWT_TOKEN>
Content-Type: application/json

{
  "wallet_address": "5GrwvaEF...",
  "amount": "500.00",
  "bank_account": "TR330006100519786457841326"
}
```

### KYC Endpoints

#### Submit KYC
```bash
POST /api/v1/kyc
Authorization: Bearer <JWT_TOKEN>
Content-Type: application/json

{
  "document_hash": "0xabcdef1234567890",
  "full_name": "John Doe",
  "date_of_birth": "1990-01-01",
  "country": "US",
  "wallet_address": "5GrwvaEF..."
}
```

### Admin Endpoints

#### Get Pending Mint Requests
```bash
GET /admin/v1/mint/pending
X-Admin-API-Key: <ADMIN_API_KEY>
```

#### Approve Mint Request
```bash
POST /admin/v1/mint/{request_id}/approve
X-Admin-API-Key: <ADMIN_API_KEY>
Content-Type: application/json

{
  "admin_id": "uuid-here",
  "chain_request_id": 0
}
```

---

## 🧪 Testing

### Run API Tests

```bash
cd velopay-api

# Run test suite
test-api.bat

# Or manually test endpoints
curl http://localhost:8080/health
```

### Run Chain Tests

```bash
cd velo-chain
cargo test --release
```

### Run Frontend Tests

```bash
cd velopay-web
npm run test
```

---

## 🏗️ Architecture

### System Flow

```
User → Frontend (SvelteKit) → API Gateway (Actix) → PostgreSQL
                                    ↓
                              Blockchain Node (Substrate)
```

### Key Components

1. **VeloPay Pallet**: Core blockchain logic for coin operations
2. **API Gateway**: RESTful interface for user interactions
3. **Database**: Off-chain data persistence for fast queries
4. **Frontend**: User-friendly web interface

### Security Model

- **JWT Authentication**: Secure API access
- **Admin API Keys**: Privileged operations require admin key
- **Rate Limiting**: Protection against abuse
- **KYC/AML**: Regulatory compliance built-in

---

## 🔧 Configuration

### Chain Configuration

Edit `velo-chain/node/src/chain_spec.rs`:
- Genesis accounts
- Initial balances
- Validator set

### API Configuration

All configuration via `.env` file:
- Database connection
- JWT settings
- CORS policies
- Rate limits

### Frontend Configuration

Edit `velopay-web/svelte.config.js`:
- API endpoint URLs
- Build settings
- Adapters

---

## 🚢 Deployment

### Production Considerations

1. **Use Strong Secrets**: Generate secure keys for production
2. **Enable HTTPS**: Use SSL certificates for API and frontend
3. **Database Backups**: Regular PostgreSQL backups
4. **Monitoring**: Set up logging and alerts
5. **Rate Limiting**: Adjust based on expected load

### Docker Deployment (Coming Soon)

```bash
docker-compose up -d
```

---

## 🤝 Contributing

We welcome contributions! Please follow these steps:

1. Fork the repository
2. Create feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit changes (`git commit -m 'Add AmazingFeature'`)
4. Push to branch (`git push origin feature/AmazingFeature`)
5. Open Pull Request

---

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

## 🙏 Acknowledgments

- **Substrate/Polkadot**: For the amazing blockchain framework
- **Actix-web**: For the performant Rust web framework
- **SvelteKit**: For the elegant frontend framework

---

## 📞 Support

- **Issues**: [GitHub Issues](https://github.com/ChronoCoders/velopay/issues)
- **Email**: support@velopay.com
- **Telegram**: @velopay_community

---

## 🗺️ Roadmap

- [x] Core blockchain functionality
- [x] REST API implementation
- [x] Database integration
- [x] Authentication system
- [x] Admin panel endpoints
- [ ] Frontend UI completion
- [ ] Mobile application
- [ ] Multi-chain support
- [ ] Staking mechanism
- [ ] Governance module

---

**Built with ❤️ by ChronoCoders Team**
