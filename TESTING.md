# VeloPay Local Testing Guide

## Prerequisites

1. **PostgreSQL Database Running**
   - Install PostgreSQL if not already installed
   - Create database: `createdb velopay`
   - Or adjust DATABASE_URL in .env if using different credentials

2. **velo-chain node running**
   ```bash
   cd velo-chain
   cargo run --release -- --dev
   ```

3. **VeloPay API server running**
   ```bash
   cd velopay-api
   cargo run --release
   ```

## Environment Configuration

The `.env` file is already configured for local development:

- **Database**: `postgresql://postgres:postgres@localhost:5432/velopay`
- **Blockchain RPC**: `ws://127.0.0.1:9944`
- **Server**: `http://127.0.0.1:8080`
- **Admin Account**: `//Alice` (has funds in --dev mode)
- **Admin API Key**: `dev-admin-api-key-for-local-testing-only-change-in-prod`

## Testing Commands (Windows cmd.exe)

### 1. Direct Blockchain Mint Request
```cmd
curl -X POST http://localhost:8080/admin/blockchain/mint -H "X-Admin-API-Key: dev-admin-api-key-for-local-testing-only-change-in-prod" -H "Content-Type: application/json" -d "{\"amount\": \"100.50\"}"
```

### 2. Direct Blockchain Burn Request
```cmd
curl -X POST http://localhost:8080/admin/blockchain/burn -H "X-Admin-API-Key: dev-admin-api-key-for-local-testing-only-change-in-prod" -H "Content-Type: application/json" -d "{\"amount\": \"50.25\"}"
```

### 3. Direct Blockchain Transfer (Alice -> Bob)
```cmd
curl -X POST http://localhost:8080/admin/blockchain/transfer -H "X-Admin-API-Key: dev-admin-api-key-for-local-testing-only-change-in-prod" -H "Content-Type: application/json" -d "{\"to_address\": \"5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty\", \"amount\": \"25.00\"}"
```

**Common Dev Addresses:**
- Alice: `5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY`
- Bob: `5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty`
- Charlie: `5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y`

## Testing with PowerShell (Alternative)

```powershell
# Setup headers
$headers = @{
    "X-Admin-API-Key" = "dev-admin-api-key-for-local-testing-only-change-in-prod"
    "Content-Type" = "application/json"
}

# Test mint request
$mintBody = @{ amount = "100.50" } | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:8080/admin/blockchain/mint" -Method Post -Headers $headers -Body $mintBody

# Test burn request
$burnBody = @{ amount = "50.25" } | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:8080/admin/blockchain/burn" -Method Post -Headers $headers -Body $burnBody

# Test transfer
$transferBody = @{
    to_address = "5FHneW46xGXgs5mUiveU4sbTyGBzmstUspZC92UhjJM694ty"
    amount = "25.00"
} | ConvertTo-Json
Invoke-RestMethod -Uri "http://localhost:8080/admin/blockchain/transfer" -Method Post -Headers $headers -Body $transferBody
```

## Expected Response Format

```json
{
  "tx_hash": "0x1234567890abcdef...",
  "blockchain_request_id": 0,
  "amount": "100.50"
}
```

## Troubleshooting

### Server won't start
- Check PostgreSQL is running: `pg_isready`
- Check velo-chain node is running and accessible at ws://127.0.0.1:9944
- Verify .env file exists and has correct values

### Blockchain transaction failed
- Ensure velo-chain node is running in --dev mode
- Check node logs for errors
- Verify ADMIN_SEED=//Alice in .env

### Database errors
- Run migrations: `sqlx migrate run`
- Check DATABASE_URL connection string
- Ensure database 'velopay' exists

## Full Workflow Test

1. Start velo-chain node
2. Start VeloPay API server
3. Submit mint request (creates blockchain request)
4. Check API logs for tx_hash
5. Check velo-chain logs for imported block
6. Test approve/reject endpoints with database-tracked requests

## Database Workflow Testing

### Create a mint request in database first
```cmd
curl -X POST http://localhost:8080/mint/request -H "Content-Type: application/json" -d "{\"wallet_address\": \"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\", \"amount\": \"100.00\", \"user_id\": \"00000000-0000-0000-0000-000000000001\"}"
```

### Then approve it (triggers blockchain)
```cmd
curl -X POST http://localhost:8080/admin/mint/{request_id}/approve -H "X-Admin-API-Key: dev-admin-api-key-for-local-testing-only-change-in-prod" -H "Content-Type: application/json" -d "{\"admin_id\": \"00000000-0000-0000-0000-000000000002\", \"chain_request_id\": 0}"
```

Replace `{request_id}` with the UUID from the first response.
