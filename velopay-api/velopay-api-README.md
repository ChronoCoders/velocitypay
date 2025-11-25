# VeloPay API - REST Gateway

REST API Gateway for VeloPay blockchain - provides HTTP interface for user interactions, database persistence, and blockchain integration.

---

## 🏗️ Architecture

### Components

- **Routes**: HTTP endpoint handlers
- **Services**: Business logic layer
- **Middleware**: Authentication & authorization
- **Database**: PostgreSQL repositories
- **Chain Client**: Substrate blockchain integration

---

## 🚀 Quick Start

### Prerequisites

- Rust 1.70+
- PostgreSQL 14+
- Running velo-chain node

### Setup Database

```bash
# Create database
psql -U postgres
CREATE DATABASE velopay_db;
CREATE USER velopay_user WITH PASSWORD 'your_password';
GRANT ALL PRIVILEGES ON DATABASE velopay_db TO velopay_user;
\q

# Run migrations
sqlx migrate run
```

### Configure

Create `.env` file:

```env
DATABASE_URL=postgresql://velopay_user:password@localhost:5432/velopay_db
HOST=127.0.0.1
PORT=8080
JWT_SECRET=your-jwt-secret-min-48-chars
JWT_EXPIRATION=86400
ADMIN_API_KEY=your-admin-api-key
ADMIN_SEED=//Alice
CHAIN_ENDPOINT=ws://127.0.0.1:9944
CORS_ALLOWED_ORIGINS=http://localhost:5173
```

### Build & Run

```bash
# Development
cargo run

# Production
cargo build --release
./target/release/velopay-api
```

API available at: `http://localhost:8080`

---

## 📚 API Endpoints

### Health Check

```bash
GET /health
```

Returns blockchain, database, and configuration status.

### Authentication

#### Register
```bash
POST /api/v1/auth/register
Content-Type: application/json

{
  "email": "user@example.com",
  "password": "SecurePass123!",
  "wallet_address": "5GrwvaEF..."
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

### Payments

#### Send Payment
```bash
POST /api/v1/payments
Authorization: Bearer <JWT>
Content-Type: application/json

{
  "from_address": "5GrwvaEF...",
  "to_address": "5FHneW46...",
  "amount": "100.50"
}
```

#### Transaction History
```bash
GET /api/v1/payments/history/{wallet}?limit=10&offset=0
Authorization: Bearer <JWT>
```

### Mint Operations

#### Create Mint Request
```bash
POST /api/v1/mint
Authorization: Bearer <JWT>

{
  "wallet_address": "5GrwvaEF...",
  "amount": "1000.00",
  "bank_reference": "BANK-REF-001"
}
```

#### My Mint Requests
```bash
GET /api/v1/mint/my-requests
Authorization: Bearer <JWT>
```

### Burn Operations

#### Create Burn Request
```bash
POST /api/v1/burn
Authorization: Bearer <JWT>

{
  "wallet_address": "5GrwvaEF...",
  "amount": "500.00",
  "bank_account": "TR123456789"
}
```

#### My Burn Requests
```bash
GET /api/v1/burn/my-requests
Authorization: Bearer <JWT>
```

### KYC

#### Submit KYC
```bash
POST /api/v1/kyc
Authorization: Bearer <JWT>

{
  "document_hash": "0xabcd...",
  "full_name": "John Doe",
  "date_of_birth": "1990-01-01",
  "country": "US",
  "wallet_address": "5GrwvaEF..."
}
```

#### My KYC Status
```bash
GET /api/v1/kyc/my-submission
Authorization: Bearer <JWT>
```

### Admin Endpoints

All admin endpoints require `X-Admin-API-Key` header.

#### Pending Mint Requests
```bash
GET /admin/v1/mint/pending
X-Admin-API-Key: <KEY>
```

#### Approve Mint
```bash
POST /admin/v1/mint/{id}/approve
X-Admin-API-Key: <KEY>

{
  "admin_id": "uuid",
  "chain_request_id": 0
}
```

#### Pending Burn Requests
```bash
GET /admin/v1/burn/pending
X-Admin-API-Key: <KEY>
```

#### Pending KYC
```bash
GET /admin/v1/kyc/pending
X-Admin-API-Key: <KEY>
```

---

## 🧪 Testing

### Automated Tests

```bash
# Run test suite
test-api.bat  # Windows
./test-api.sh # Linux/Mac
```

### Manual Testing

```bash
# Health check
curl http://localhost:8080/health

# Register user
curl -X POST http://localhost:8080/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{"email":"test@example.com","password":"Test123!","wallet_address":"5GrwvaEF..."}'
```

---

## 🔧 Configuration

### Environment Variables

- `DATABASE_URL`: PostgreSQL connection string
- `HOST`: Server bind address
- `PORT`: Server port
- `JWT_SECRET`: Secret for JWT signing (min 48 chars)
- `JWT_EXPIRATION`: Coin expiration in seconds
- `ADMIN_API_KEY`: Admin authentication key
- `ADMIN_SEED`: Blockchain admin account seed
- `CHAIN_ENDPOINT`: WebSocket endpoint for chain
- `CORS_ALLOWED_ORIGINS`: Comma-separated allowed origins
- `RATE_LIMIT_PER_SECOND`: Rate limit configuration
- `RATE_LIMIT_BURST`: Burst size for rate limiting

### Database Migrations

Located in `migrations/`:
- `20241120000001_initial_schema.sql`

Add new migration:
```bash
sqlx migrate add migration_name
```

---

## 🏗️ Project Structure

```
velopay-api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── config.rs            # Configuration management
│   ├── utils.rs             # Utility functions
│   ├── chain/               # Blockchain integration
│   │   ├── client.rs        # Chain client
│   │   └── operations.rs    # Chain operations
│   ├── db/                  # Database layer
│   │   ├── mod.rs
│   │   ├── pool.rs          # Connection pool
│   │   └── *_repository.rs  # Data repositories
│   ├── middleware/          # HTTP middleware
│   │   ├── auth.rs          # JWT authentication
│   │   └── admin.rs         # Admin authentication
│   ├── models/              # Data models
│   │   ├── user.rs
│   │   ├── transaction.rs
│   │   └── ...
│   ├── routes/              # API endpoints
│   │   ├── auth_routes.rs
│   │   ├── payment_routes.rs
│   │   ├── mint_routes.rs
│   │   ├── burn_routes.rs
│   │   ├── kyc_routes.rs
│   │   └── admin_routes.rs
│   └── services/            # Business logic
│       ├── auth_service.rs
│       ├── payment_service.rs
│       └── ...
└── migrations/              # Database migrations
```

---

## 🔐 Security

### Authentication

- **JWT Coins**: Bearer coin authentication for users
- **Admin API Keys**: Static key authentication for admin operations
- **Password Hashing**: Bcrypt with cost factor 12

### Rate Limiting

- General endpoints: 2 requests/second, burst of 10
- Auth endpoints: 5 requests/minute

### CORS

Configure allowed origins in `.env`:
```env
CORS_ALLOWED_ORIGINS=http://localhost:5173,https://velopay.com
```

---

## 📖 Development

### Add New Endpoint

1. Create route handler in `routes/`
2. Add service logic in `services/`
3. Register route in `main.rs`
4. Test endpoint

### Database Schema Changes

1. Create migration: `sqlx migrate add name`
2. Edit SQL in `migrations/`
3. Run: `sqlx migrate run`
4. Update repositories

---

## 🐛 Troubleshooting

### Database Connection Failed
- Check PostgreSQL is running
- Verify credentials in `.env`
- Ensure database exists

### Blockchain Connection Failed
- Check velo-chain is running
- Verify `CHAIN_ENDPOINT` in `.env`
- Check firewall settings

### Authentication Errors
- Verify JWT_SECRET is set
- Check coin expiration
- Ensure user exists in database

---

## 📝 API Response Format

### Success Response
```json
{
  "id": "uuid",
  "field": "value",
  ...
}
```

### Error Response
```json
{
  "error": "Error message description"
}
```

---

**Part of VeloPay Project**
