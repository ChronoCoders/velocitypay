<div align="center">

# VeloPay

### Enterprise-Grade Blockchain Payment System with Fiat-Backed Stablecoin

*Built on Substrate | Production-Ready | Regulatory Compliant*

[![License](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](https://opensource.org/licenses/Apache-2.0)
[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
[![Substrate](https://img.shields.io/badge/Substrate-stable2407-brightgreen.svg)](https://substrate.io/)
[![Platform](https://img.shields.io/badge/Platform-Windows%20%7C%20Linux%20%7C%20macOS-lightgrey.svg)](https://github.com/ChronoCoders/velopay)

[Features](#features) • [Architecture](#architecture) • [Quick Start](#quick-start) • [Documentation](#documentation) • [Contributing](#contributing)

---

</div>

## Overview

VeloPay is a professional blockchain-based payment infrastructure designed for enterprises requiring transparent, auditable, and compliant fiat-backed stablecoin operations. Built on Substrate framework with Proof of Authority consensus, it provides a complete solution for digital payment processing with integrated KYC, compliance monitoring, and multi-signature governance.

### Key Highlights

- **Fiat-Backed Stablecoin (VCS)** - 1:1 USD peg with full reserve backing
- **Enterprise-Ready** - PoA consensus with trusted validators
- **Regulatory Compliance** - Built-in KYC/AML workflows and monitoring
- **Multi-Validator Setup** - Windows-compatible scripts for easy deployment
- **REST API Gateway** - Complete backend with 2,600+ lines of production code
- **Audit Trail** - Comprehensive logging for all operations

---

## Features

### Blockchain Core (Substrate)

<table>
<tr>
<td width="50%">

**VeloPay Pallet**
- Request-based minting with approval workflow
- Reserved burn system (locks before burning)
- Configurable transaction fees (basis points)
- Emergency pause/unpause mechanism
- Total supply tracking and limits
- Authority-based access control

</td>
<td width="50%">

**KYC Pallet**
- Document hash submission
- Multi-tier verification workflow
- KYC verifier role management
- Integration with mint/burn operations
- Privacy-preserving design
- Compliance reporting

</td>
</tr>
<tr>
<td width="50%">

**Compliance Pallet**
- Account flagging system
- Suspicious activity alerts
- Transaction limit monitoring
- Compliance officer roles
- Alert resolution workflow
- AML integration ready

</td>
<td width="50%">

**Consensus & Finality**
- Aura PoA consensus (trusted validators)
- GRANDPA finality gadget
- Multi-signature support
- Utility batch operations
- Session key management
- Telemetry integration

</td>
</tr>
</table>

### API Gateway (Actix-web + PostgreSQL)

- **Authentication** - JWT-based with bcrypt password hashing
- **Wallet Operations** - Balance queries, transaction history, address validation
- **Payment Processing** - VCS transfers with fee estimation and tracking
- **Mint/Burn Workflows** - Admin approval system with blockchain integration
- **KYC Management** - Document verification and status tracking
- **Admin Dashboard** - Pending approvals, system statistics, user management
- **Rate Limiting** - Configurable per-endpoint throttling
- **Database Layer** - PostgreSQL with SQLx (compile-time verified queries)

### Windows Validator Management

Production-ready batch scripts for multi-validator setup:

```
setup-node-keys-v2.bat    - Generate persistent Ed25519 node keys
run-alice-final.bat       - Start Alice validator with stable peer ID
run-bob-final.bat         - Start Bob validator with automatic peering
inspect-node-keys.bat     - View node key information
reset-testnet.bat         - Clean blockchain data (preserves keys)
check-network-status.bat  - Monitor validator connectivity
```

**Features:**
- Pure CMD (no PowerShell dependency)
- Persistent node identities across restarts
- Automatic peer ID management
- Plain ASCII output (CMD-compatible)
- No ANSI codes or emojis

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────────┐
│                         VeloPay Ecosystem                           │
└─────────────────────────────────────────────────────────────────────┘

┌─────────────────────┐       ┌─────────────────────┐       ┌─────────────────────┐
│   Web Frontend      │       │   Mobile App        │       │   Admin Panel       │
│   (SvelteKit)       │◄─────►│   (React Native)    │◄─────►│   (React)           │
│                     │       │                     │       │                     │
└──────────┬──────────┘       └──────────┬──────────┘       └──────────┬──────────┘
           │                             │                             │
           └─────────────────────────────┼─────────────────────────────┘
                                         │
                                         ▼
                          ┌──────────────────────────────┐
                          │      API Gateway             │
                          │   (Actix-web + Subxt)        │
                          │                              │
                          │  • JWT Authentication        │
                          │  • REST Endpoints            │
                          │  • Rate Limiting             │
                          │  • PostgreSQL Database       │
                          └──────────────┬───────────────┘
                                         │
                                         ▼
                          ┌──────────────────────────────┐
                          │    VeloPay Blockchain        │
                          │    (Substrate)               │
                          │                              │
                          │  ┌─────────────────────────┐ │
                          │  │  VeloPay Pallet         │ │
                          │  │  • Mint/Burn            │ │
                          │  │  • Transfer             │ │
                          │  └─────────────────────────┘ │
                          │                              │
                          │  ┌─────────────────────────┐ │
                          │  │  KYC Pallet             │ │
                          │  │  • Verification         │ │
                          │  │  • Document Hash        │ │
                          │  └─────────────────────────┘ │
                          │                              │
                          │  ┌─────────────────────────┐ │
                          │  │  Compliance Pallet      │ │
                          │  │  • AML Monitoring       │ │
                          │  │  • Alert System         │ │
                          │  └─────────────────────────┘ │
                          │                              │
                          │  Consensus: Aura (PoA)       │
                          │  Finality: GRANDPA           │
                          └──────────────────────────────┘
```

---

## Quick Start

### Prerequisites

**All Platforms:**
- Rust 1.70+ ([Install](https://rustup.rs/))
- PostgreSQL 14+ (for API)
- Git

**Platform-Specific:**
- **Windows:** Visual Studio Build Tools with C++ workload
- **Linux/macOS:** Build essentials (gcc, clang, make)

### 1. Build the Blockchain

**Linux/macOS:**
```bash
cd velo-chain
rustup target add wasm32-unknown-unknown
cargo build --release

# Run single validator (development)
./target/release/velo-node --dev
```

**Windows:**
```cmd
cd velo-chain
rustup target add wasm32-unknown-unknown
set OPENSSL_VENDORED=1
cargo build --release

REM Multi-validator setup
setup-node-keys-v2.bat
run-alice-final.bat      REM Terminal 1
run-bob-final.bat        REM Terminal 2
```

### 2. Setup API Gateway

```bash
cd velopay-api

# Database setup
createdb velopay
sqlx migrate run

# Configuration
cp .env.example .env
# Edit .env with your settings

# Build and run
cargo sqlx prepare
cargo build --release
cargo run --release
```

### 3. Test the API

```bash
# Register user
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"SecurePass123"}'

# Login
curl -X POST http://localhost:8080/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"user@example.com","password":"SecurePass123"}'
```

**Build time:** 15-45 minutes (first build)

---

## Technology Stack

### Blockchain Layer
![Rust](https://img.shields.io/badge/Rust-000000?style=for-the-badge&logo=rust&logoColor=white)
![Substrate](https://img.shields.io/badge/Substrate-282828?style=for-the-badge&logo=polkadot&logoColor=white)

- **Framework:** Substrate (Polkadot SDK stable2407)
- **Language:** Rust 1.70+
- **Consensus:** Aura (Proof of Authority)
- **Finality:** GRANDPA
- **Storage:** RocksDB

### API Gateway
![Actix](https://img.shields.io/badge/Actix-000000?style=for-the-badge&logo=rust&logoColor=white)
![PostgreSQL](https://img.shields.io/badge/PostgreSQL-316192?style=for-the-badge&logo=postgresql&logoColor=white)

- **Framework:** Actix-web 4.4
- **Database:** PostgreSQL 14+ with SQLx
- **Auth:** JWT + bcrypt
- **Blockchain Client:** Subxt 0.32
- **Runtime:** Tokio async

### Frontend (Planned)
![Svelte](https://img.shields.io/badge/Svelte-FF3E00?style=for-the-badge&logo=svelte&logoColor=white)
![TypeScript](https://img.shields.io/badge/TypeScript-007ACC?style=for-the-badge&logo=typescript&logoColor=white)

- **Framework:** SvelteKit
- **Language:** TypeScript
- **Styling:** Tailwind CSS
- **State:** Stores + Context

---

## Token Economics

| Parameter | Value |
|-----------|-------|
| **Token** | VCS (Velo Cash) |
| **Type** | Fiat-Backed Stablecoin |
| **Peg** | 1 VCS = 1 USD |
| **Decimals** | 12 (Substrate standard) |
| **Transaction Fee** | 0.1% (configurable) |
| **Minimum Transfer** | 0.01 VCS |
| **Maximum Transfer** | 1,000,000 VCS per transaction |
| **Daily Limit** | 10,000,000 VCS per user |
| **Reserve Backing** | 1:1 USD in segregated accounts |

---

## Project Structure

```
velopay/
├── velo-chain/                    # Substrate blockchain
│   ├── pallets/
│   │   ├── velopay/              # Core stablecoin logic
│   │   ├── kyc/                  # Identity verification
│   │   └── compliance/           # AML monitoring
│   ├── runtime/                  # Runtime configuration
│   ├── node/                     # Node implementation
│   ├── setup-node-keys-v2.bat   # Windows key generation
│   ├── run-alice-final.bat      # Alice validator script
│   ├── run-bob-final.bat        # Bob validator script
│   └── BUILD.md                 # Detailed build guide
│
├── velopay-api/                  # REST API Gateway
│   ├── src/
│   │   ├── services/            # Business logic (910 lines)
│   │   ├── routes/              # HTTP endpoints (950 lines)
│   │   ├── db/                  # Database repositories
│   │   ├── middleware/          # Auth, rate limiting
│   │   └── chain/               # Blockchain client
│   ├── migrations/              # Database schema
│   └── RELEASE_NOTES.md        # API v1.0.0 release notes
│
├── velopay-web/                  # Frontend (coming soon)
│
└── README.md                     # This file
```

---

## Documentation

### For Users
- [Quick Start Guide](#quick-start)
- [API Documentation](velopay-api/RELEASE_NOTES.md)
- [Windows Setup Guide](velo-chain/NODE-MANAGEMENT-README.md)
- [Token Economics](#token-economics)

### For Developers
- [Build Instructions](velo-chain/BUILD.md)
- [API Gateway Setup](velopay-api/README.md)
- [Database Schema](velopay-api/migrations/20241120000001_initial_schema.sql)
- [Technical Setup Guide](velo-chain/WORKING-SETUP-GUIDE.md)

### For Operators
- [Validator Management](velo-chain/NODE-MANAGEMENT-README.md)
- [Production Deployment](velo-chain/WORKING-SETUP-GUIDE.md#production-deployment-checklist)
- [Monitoring Setup](#monitoring)
- [Security Best Practices](velo-chain/NODE-MANAGEMENT-README.md#security-best-practices)

---

## Roadmap

### Version 1.0 (Current)
- [x] Complete Substrate blockchain implementation
- [x] Custom pallets (VeloPay, KYC, Compliance)
- [x] REST API Gateway (2,600+ lines)
- [x] PostgreSQL database integration
- [x] Windows validator management scripts
- [x] JWT authentication system
- [x] Admin approval workflows

### Version 1.1 (Q1 2025)
- [ ] Fix blockchain integration (PairSigner import)
- [ ] OpenAPI/Swagger documentation
- [ ] Docker containerization
- [ ] Integration test suite
- [ ] Load testing results
- [ ] Monitoring & metrics (Prometheus/Grafana)

### Version 2.0 (Q2 2025)
- [ ] SvelteKit frontend application
- [ ] Block explorer
- [ ] Admin dashboard UI
- [ ] WebSocket real-time updates
- [ ] Mobile app (React Native)
- [ ] Multi-language support

### Version 3.0 (Q3 2025)
- [ ] Cross-chain bridge integration
- [ ] Advanced analytics dashboard
- [ ] Machine learning fraud detection
- [ ] Automated compliance reporting
- [ ] High availability architecture
- [ ] Sentry node deployment

---

## Security

### Blockchain Level
- Proof of Authority consensus (trusted validators only)
- Multi-signature support for critical operations
- Emergency pause mechanism
- KYC gating for all mint/burn operations
- Comprehensive audit logging

### API Level
- JWT authentication with configurable expiration
- bcrypt password hashing (cost factor: 12)
- Rate limiting (100 requests/minute default)
- Input validation on all endpoints
- SQL injection protection (parameterized queries)
- CORS configuration

### Operational
- 1:1 USD reserve backing
- Regular reserve audits
- AML transaction monitoring
- Regulatory reporting capabilities
- Incident response procedures

**Security Audits:** Coming in v1.1

---

## Monitoring

### Metrics Available
- Prometheus exporter on validator nodes (port 9615)
- RPC server metrics
- Database connection pool stats
- API request/response times
- Transaction throughput

### Dashboards (Planned)
- Grafana blockchain metrics
- API performance monitoring
- Alert manager integration
- Custom business metrics

---

## Contributing

We welcome contributions! Please see our [Contributing Guidelines](CONTRIBUTING.md) for details.

### Development Workflow

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Run tests** (`cargo test`)
5. **Commit** (`git commit -m 'Add amazing feature'`)
6. **Push** (`git push origin feature/amazing-feature`)
7. **Open a Pull Request**

### Code Standards
- Follow Rust style guidelines
- Add tests for new features
- Update documentation
- Keep commits atomic and well-described

### Areas for Contribution
- Frontend development (SvelteKit)
- Additional blockchain features
- API enhancements
- Documentation improvements
- Testing and QA
- Security audits

---

## Community

- **GitHub Discussions:** [Ask Questions](https://github.com/ChronoCoders/velopay/discussions)
- **Issues:** [Report Bugs](https://github.com/ChronoCoders/velopay/issues)
- **Pull Requests:** [Contribute Code](https://github.com/ChronoCoders/velopay/pulls)

---

## License

This project is licensed under the Apache License 2.0 - see the [LICENSE](LICENSE) file for details.

```
Copyright 2024-2025 ChronoCoders

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

---

## Acknowledgments

- **Parity Technologies** - Substrate Framework
- **Polkadot Community** - Technical guidance and resources
- **Rust Community** - Amazing tooling and ecosystem
- **Contributors** - All who have helped build VeloPay

---

<div align="center">

**Built with ❤️ by ChronoCoders**

[⬆ Back to Top](#velopay)

</div>
