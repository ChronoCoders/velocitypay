# Development Setup Guide

Quick guide to get VelocityPay running locally for development and testing.

## Prerequisites

- Rust 1.70+ with wasm32-unknown-unknown target
- PostgreSQL 14+
- Git

## Quick Start

### 1. Database Setup

```bash
# Create PostgreSQL database
createdb velocitypay

# Or using psql
psql -U postgres -c "CREATE DATABASE velocitypay;"

# Run migrations
psql -U postgres -d velocitypay -f velocitypay-api/migrations/001_init.sql
```

### 2. Start Blockchain Node (Terminal 1)

```bash
cd velocity-chain

# First time: Add WASM target
rustup target add wasm32-unknown-unknown

# Build and run development node
cargo run --release -- --dev --tmp

# Node will be available at ws://127.0.0.1:9944
```

**Expected output:**
```
🏗  Initializing Genesis block/state...
📦 Highest known block at #0
🔍 Discovered new external address...
💤 Idle (0 peers), best: #0...
```

### 3. Start API Gateway (Terminal 2)

```bash
cd velocitypay-api

# Environment file already created at .env with development credentials
# Verify it exists:
cat .env

# Run the API
cargo run

# API will be available at http://127.0.0.1:8080
```

**Expected output:**
```
[INFO] Starting VelocityPay API Gateway
[INFO] Connecting to chain at: ws://127.0.0.1:9944
[INFO] Database: postgresql://postgres:password@localhost:5432/velocitypay
[INFO] Server starting on http://127.0.0.1:8080
```

### 4. Test the API

```bash
# Health check
curl http://localhost:8080/health

# Expected response:
# {"status":"healthy","service":"velocitypay-api"}

# API status
curl http://localhost:8080/api/v1/status

# Expected response:
# {"version":"1.0.0","status":"operational"}
```

## Development Environment Details

### Environment Variables (in .env)

All environment variables are pre-configured in `velocitypay-api/.env`:

- **DATABASE_URL**: PostgreSQL connection string
- **CHAIN_RPC_URL**: Local blockchain node WebSocket endpoint
- **SERVER_HOST/PORT**: API server binding address
- **JWT_SECRET**: Development JWT signing key (32+ chars)
- **ADMIN_API_KEY**: Development admin authentication key (32+ chars)
- **RATE_LIMIT_REQUESTS**: Requests per window (100/min)
- **CORS_ALLOWED_ORIGINS**: Allowed frontend origins for development

⚠️ **Important**: These are development credentials only! Never use in production.

### Database Connection

Default PostgreSQL connection:
- **Host**: localhost
- **Port**: 5432
- **Database**: velocitypay
- **User**: postgres
- **Password**: password

If your PostgreSQL uses different credentials, update `DATABASE_URL` in `.env`:
```
DATABASE_URL=postgresql://your_user:your_password@localhost:5432/velocitypay
```

## Troubleshooting

### API won't start - "JWT_SECRET must be set"

The API now requires proper secrets (32+ characters). The `.env` file has been created with development-friendly values.

**Solution**: Verify `.env` exists in `velocitypay-api/` directory:
```bash
ls -la velocitypay-api/.env
```

### Database connection failed

**Check PostgreSQL is running:**
```bash
# Linux/Mac
sudo systemctl status postgresql
# or
brew services list

# Test connection
psql -U postgres -d velocitypay -c "SELECT 1;"
```

**Create database if missing:**
```bash
createdb velocitypay
psql -U postgres -d velocitypay -f velocitypay-api/migrations/001_init.sql
```

### Blockchain node won't connect

**Verify node is running:**
```bash
# Check WebSocket is accessible
curl -H "Content-Type: application/json" \
  -d '{"id":1,"jsonrpc":"2.0","method":"system_health"}' \
  http://127.0.0.1:9933
```

**Check firewall:**
```bash
# Ensure ports 9933 (HTTP RPC) and 9944 (WS RPC) are accessible
netstat -an | grep 9944
```

### CORS errors in browser

If you get CORS errors, add your frontend URL to `CORS_ALLOWED_ORIGINS` in `.env`:
```
CORS_ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5173,http://your-frontend:port
```

Then restart the API server.

### Build errors - "wasm32-unknown-unknown not found"

```bash
rustup target add wasm32-unknown-unknown
```

### Database migration errors - "type already exists"

If you need to reset the database:
```bash
dropdb velocitypay
createdb velocitypay
psql -U postgres -d velocitypay -f velocitypay-api/migrations/001_init.sql
```

## Testing Blockchain Operations

### Using Polkadot.js Apps

1. Open https://polkadot.js.org/apps/
2. Click top-left corner → Development → Local Node
3. Connect to `ws://127.0.0.1:9944`
4. Explore pallets:
   - **VelocityPay**: Mint/burn operations, transfers
   - **KYC**: Submit and verify KYC
   - **Compliance**: Account flagging, alerts

### Using Command Line

```bash
# Install subxt CLI (if needed)
cargo install subxt-cli

# Connect and query
subxt metadata --url ws://127.0.0.1:9944
```

## Next Steps

After getting everything running:

1. **Review the audit report**: See `AUDIT_REPORT.md` for security improvements
2. **Review applied fixes**: See `FIXES_APPLIED.md` for all security fixes
3. **Read API documentation**: See `README.md` for planned endpoints
4. **Implement missing services**: API service layer needs implementation
5. **Write tests**: Add unit and integration tests

## Production Deployment

⚠️ **NEVER use development credentials in production!**

Before production deployment:

1. Generate strong secrets:
   ```bash
   # Generate JWT secret
   openssl rand -base64 32

   # Generate admin API key
   openssl rand -base64 32
   ```

2. Update `.env` with production values
3. Set `CORS_ALLOWED_ORIGINS` to your production frontend URL only
4. Use proper PostgreSQL credentials
5. Review security checklist in `FIXES_APPLIED.md`

## Development Workflow

### Making Changes

**Blockchain changes:**
```bash
cd velocity-chain
# Make changes to pallets
cargo build --release
# Restart node with --tmp for fresh state
cargo run --release -- --dev --tmp
```

**API changes:**
```bash
cd velocitypay-api
# Make changes to API code
cargo build
# Restart API
cargo run
```

### Running Tests

```bash
# Blockchain tests
cd velocity-chain
cargo test

# API tests
cd velocitypay-api
cargo test
```

## Resources

- **Substrate Documentation**: https://docs.substrate.io/
- **Polkadot.js Apps**: https://polkadot.js.org/apps/
- **Actix Web Guide**: https://actix.rs/docs/
- **Project README**: See `README.md` for full documentation

## Support

For issues or questions:
1. Check `AUDIT_REPORT.md` for known issues
2. Check `FIXES_APPLIED.md` for recent changes
3. Review GitHub issues (if applicable)

---

Happy coding! 🚀
