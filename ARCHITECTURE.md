# VeloPay Architecture Documentation

## Table of Contents
1. [System Overview](#system-overview)
2. [Architecture Layers](#architecture-layers)
3. [Component Diagram](#component-diagram)
4. [Security Architecture](#security-architecture)
5. [Data Flow](#data-flow)
6. [API Design](#api-design)
7. [Deployment Architecture](#deployment-architecture)

---

## System Overview

VeloPay is an enterprise-grade blockchain payment system built on Substrate, featuring a USD-backed stablecoin (VCS - Velo Cash) with integrated KYC/AML compliance and regulatory monitoring.

### Core Components
- **velo-chain**: Substrate blockchain with custom pallets
- **velopay-api**: REST API gateway (Actix-web)
- **PostgreSQL**: Relational database for off-chain data
- **Blockchain RPC**: WebSocket connection to Substrate node

---

## Architecture Layers

### Layer 1: Blockchain (velo-chain)

#### Custom Pallets
```
┌─────────────────────────────────────┐
│         VeloPay Pallet              │
│  - Mint/Burn workflows              │
│  - Transfer with fees               │
│  - Authority management             │
└─────────────────────────────────────┘
           ▲  ▲
           │  │
    ┌──────┘  └──────┐
    │                │
┌───┴────┐      ┌────┴────┐
│   KYC  │      │Compliance│
│ Pallet │      │  Pallet  │
└────────┘      └──────────┘
```

**VeloPay Pallet** (`pallets/velopay`)
- Request-approval workflow for mint/burn
- Reserved burn mechanism (locks tokens before burning)
- Configurable transaction fees (basis points)
- Emergency pause/unpause controls
- Integration with KYC and Compliance checks

**KYC Pallet** (`pallets/kyc`)
- Document hash submission (privacy-preserving)
- Multi-status workflow: NotSubmitted → Pending → Verified/Rejected
- KYC verifier role management
- Gates mint/burn operations

**Compliance Pallet** (`pallets/compliance`)
- Account flagging system
- Suspicious activity alerts (Low/Medium/High/Critical)
- Daily transaction volume tracking
- Transaction limit monitoring (10M VCS daily, 100K threshold)
- Compliance officer role management

#### Runtime Configuration
- **Consensus**: Aura (Proof of Authority)
- **Finality**: GRANDPA
- **Block Time**: 6 seconds
- **Token Decimals**: 12 (like DOT)
- **Standard Pallets**: Balances, Sudo, Multisig, Utility, Transaction Payment

### Layer 2: API Gateway (velopay-api)

#### Layered Architecture
```
┌─────────────────────────────────────────┐
│          HTTP Routes Layer              │
│  (auth, payment, mint, burn, kyc, admin)│
└────────────┬────────────────────────────┘
             │
┌────────────┴────────────────────────────┐
│        Services Layer                   │
│  (Business logic & validation)          │
└────────┬────────────────┬───────────────┘
         │                │
    ┌────┴────┐      ┌────┴─────┐
    │ Database│      │Blockchain│
    │  Layer  │      │  Client  │
    └─────────┘      └──────────┘
```

**Routes** (`src/routes/`)
- Public: auth routes (login, register, token refresh)
- Protected: payment, mint, burn, KYC routes
- Admin: approval routes, blockchain operations

**Services** (`src/services/`)
- **AuthService**: JWT generation, password hashing
- **PaymentService**: Transaction creation, fee calculation
- **MintService**: Mint request workflow
- **BurnService**: Burn request workflow
- **KYCService**: KYC submission and verification

**Database Layer** (`src/db/`)
- Repository pattern with SQLx
- Compile-time query verification
- Connection pooling (50 max, 5 min)
- Automatic migrations

**Blockchain Client** (`src/chain/`)
- Subxt client for Substrate
- PairSigner for transaction signing
- Event parsing for request IDs
- 30-second operation timeouts

### Layer 3: Middleware

```
Request → CORS → Rate Limit → Logger → Auth → Handler
```

- **CORS**: Validated origins, no wildcards in production
- **Rate Limiting**:
  - Auth endpoints: 5 req/min
  - General endpoints: 100 req/min
- **Authentication**: JWT validation (HS256)
- **Admin Auth**: API key validation

---

## Component Diagram

```
┌─────────────────────────────────────────────────────────┐
│                     Frontend (Future)                    │
│                  (SvelteKit / React)                     │
└───────────────────────┬─────────────────────────────────┘
                        │ HTTP/REST
                        ▼
┌─────────────────────────────────────────────────────────┐
│                   VeloPay API Gateway                    │
│  ┌──────────┐  ┌──────────┐  ┌─────────┐  ┌─────────┐ │
│  │  Auth    │  │ Payment  │  │  Mint   │  │  Burn   │ │
│  │ Service  │  │ Service  │  │ Service │  │ Service │ │
│  └──────────┘  └──────────┘  └─────────┘  └─────────┘ │
└──────┬─────────────────────────────────────────┬────────┘
       │                                          │
       │ SQL                                      │ WebSocket
       ▼                                          ▼
┌─────────────┐                          ┌───────────────┐
│ PostgreSQL  │                          │  Velo-Chain   │
│             │                          │   (Substrate) │
│ - Users     │                          │               │
│ - Txns      │                          │ ┌───────────┐ │
│ - Requests  │                          │ │  VeloPay  │ │
│ - KYC       │                          │ │   Pallet  │ │
└─────────────┘                          │ └───────────┘ │
                                         │ ┌───────────┐ │
                                         │ │    KYC    │ │
                                         │ │   Pallet  │ │
                                         │ └───────────┘ │
                                         │ ┌───────────┐ │
                                         │ │Compliance │ │
                                         │ │   Pallet  │ │
                                         │ └───────────┘ │
                                         └───────────────┘
```

---

## Security Architecture

### Authentication & Authorization

**User Authentication**
- JWT tokens with HS256 signing
- 24-hour token expiration (configurable)
- Refresh token support
- Password hashing with bcrypt (cost: 12)

**Password Requirements**
- Minimum 12 characters
- At least one uppercase letter
- At least one lowercase letter
- At least one number
- At least one special character

**Admin Authentication**
- Separate API key authentication
- Minimum 32-character API keys
- Rate limited independently

### Network Security

**Rate Limiting**
- Auth endpoints: 5 requests/minute (prevents brute force)
- General endpoints: 100 requests/minute (prevents DoS)
- Per-IP tracking with token bucket algorithm

**CORS Policy**
- Whitelist-only origins
- No wildcard (*) in production
- Validated at startup

**Input Validation**
- SS58 address format validation
- Email format validation
- Amount validation (non-zero, positive)
- SQL injection prevention (SQLx compile-time checks)

### Blockchain Security

**Transaction Timeouts**
- 30-second timeout for all blockchain operations
- Prevents indefinite hanging

**Access Control**
- Mint Authority: Can approve mint requests
- Burn Authority: Can approve burn requests
- KYC Verifier: Can verify KYC submissions
- Compliance Officer: Can flag accounts and create alerts
- Sudo: Root access for emergency operations

**Audit Trail**
- All blockchain events logged
- Request IDs tracked in database
- Transaction hashes stored
- Status transitions recorded

---

## Data Flow

### Mint Request Flow

```
1. User → API: POST /api/v1/mint/request
         ↓
2. API validates amount, KYC status
         ↓
3. API → DB: Create mint request (status: pending)
         ↓
4. API → Blockchain: Submit mint request extrinsic
         ↓
5. Blockchain → Event: MintRequested(request_id)
         ↓
6. API parses event, extracts request_id
         ↓
7. API → DB: Update request with chain_request_id
         ↓
8. API → User: Return request details

---

9. Admin → API: POST /api/v1/admin/mint/approve
         ↓
10. API → Blockchain: Submit approve_mint extrinsic
         ↓
11. Blockchain: Checks status, mints tokens
         ↓
12. API → DB: Update request status (approved)
         ↓
13. API → Admin: Return success
```

### Transfer Flow

```
1. User → API: POST /api/v1/payment/transfer
         ↓
2. API validates: KYC, compliance, balance
         ↓
3. API → Blockchain: Submit transfer extrinsic
         ↓
4. Blockchain:
   - Check KYC for sender & recipient
   - Check compliance for both parties
   - Calculate fee
   - Execute transfer
   - Transfer fee to authority
         ↓
5. API → DB: Record transaction
         ↓
6. API → User: Return transaction hash
```

---

## API Design

### Endpoint Categories

**Public Endpoints**
- `POST /api/v1/auth/register` - User registration
- `POST /api/v1/auth/login` - User login
- `POST /api/v1/auth/refresh` - Token refresh
- `GET /health` - Health check
- `GET /api/v1/status` - API status

**Protected Endpoints** (JWT required)
- `GET /api/v1/auth/profile` - Get user profile
- `PUT /api/v1/auth/wallet` - Update wallet address
- `POST /api/v1/payment/transfer` - Transfer VCS
- `GET /api/v1/payment/balance/{address}` - Get balance
- `GET /api/v1/payment/history` - Transaction history
- `POST /api/v1/mint/request` - Request mint
- `GET /api/v1/mint/requests` - Get user's mint requests
- `POST /api/v1/burn/request` - Request burn
- `GET /api/v1/burn/requests` - Get user's burn requests
- `POST /api/v1/kyc/submit` - Submit KYC
- `GET /api/v1/kyc/status` - Check KYC status

**Admin Endpoints** (Admin API key required)
- `GET /api/v1/admin/mint/pending` - Get pending mints
- `POST /api/v1/admin/mint/approve` - Approve mint
- `POST /api/v1/admin/mint/reject` - Reject mint
- `GET /api/v1/admin/burn/pending` - Get pending burns
- `POST /api/v1/admin/burn/approve` - Approve burn
- `POST /api/v1/admin/burn/reject` - Reject burn
- `GET /api/v1/admin/kyc/pending` - Get pending KYC
- `POST /api/v1/admin/kyc/verify` - Verify KYC
- `POST /api/v1/admin/kyc/reject` - Reject KYC

### Response Format

**Success Response**
```json
{
  "status": "success",
  "data": { ... }
}
```

**Error Response**
```json
{
  "status": "error",
  "message": "Error description"
}
```

---

## Deployment Architecture

### Production Setup (Recommended)

```
                    Internet
                       │
                       ▼
              ┌──────────────┐
              │ Load Balancer│
              │   (Nginx)    │
              └───┬──────┬───┘
                  │      │
        ┌─────────┘      └─────────┐
        ▼                          ▼
  ┌──────────┐              ┌──────────┐
  │ API Node │              │ API Node │
  │    1     │              │    2     │
  └────┬─────┘              └────┬─────┘
       │                         │
       └────────┬────────────────┘
                │
       ┌────────┴────────┐
       │                 │
       ▼                 ▼
  ┌──────────┐      ┌──────────┐
  │PostgreSQL│      │Blockchain│
  │ Primary  │      │   Node   │
  └────┬─────┘      └──────────┘
       │
       ▼
  ┌──────────┐
  │PostgreSQL│
  │ Replica  │
  └──────────┘
```

### Environment Configuration

**Required Environment Variables**
- `DATABASE_URL` - PostgreSQL connection string
- `CHAIN_RPC_URL` - Blockchain WebSocket URL
- `JWT_SECRET` - JWT signing secret (32+ chars)
- `ADMIN_API_KEY` - Admin API key (32+ chars)
- `ADMIN_SEED` - Blockchain admin seed phrase
- `CORS_ALLOWED_ORIGINS` - Comma-separated origins

**Optional Environment Variables**
- `SERVER_HOST` - API host (default: 127.0.0.1)
- `SERVER_PORT` - API port (default: 8080)
- `JWT_EXPIRATION` - Token lifetime in seconds (default: 86400)
- `RATE_LIMIT_REQUESTS` - Requests per window (default: 100)
- `RATE_LIMIT_WINDOW_SECONDS` - Window size (default: 60)

### Monitoring

**Health Checks**
- Database connectivity
- Blockchain RPC connectivity
- Configuration validation
- HTTP 200 if healthy, 503 if unhealthy

**Metrics to Monitor**
- Request rate per endpoint
- Response times
- Error rates
- Database connection pool utilization
- Blockchain synchronization status

---

## Performance Considerations

### Database Optimization
- Connection pooling (50 max connections)
- Indexed queries on frequently accessed columns
- Prepared statements (SQLx compile-time)
- Connection testing before use

### Blockchain Optimization
- Event parsing instead of storage queries where possible
- Batch operations when feasible
- Timeout handling to prevent hanging

### API Optimization
- Async/await throughout (Tokio runtime)
- Rate limiting to prevent abuse
- CORS caching (max-age: 3600)

---

## Future Enhancements

1. **Multi-signature Support** - Require multiple approvals for high-value operations
2. **Circuit Breaker** - Prevent cascading failures
3. **Event Sourcing** - Complete audit trail of state changes
4. **Caching Layer** - Redis for frequently accessed data
5. **Monitoring** - Prometheus metrics, Grafana dashboards
6. **Cross-chain Bridge** - Integration with other blockchains
7. **Frontend** - SvelteKit web application
8. **Mobile Apps** - iOS/Android applications

---

## References

- [Substrate Documentation](https://docs.substrate.io/)
- [Actix-web Documentation](https://actix.rs/)
- [SQLx Documentation](https://github.com/launchbadge/sqlx)
- [Subxt Documentation](https://github.com/paritytech/subxt)
