# VeloPay API Gateway

RESTful API backend for VeloPay blockchain payment system.

## Features

- JWT authentication and user management
- Wallet operations and balance queries
- Payment processing with blockchain integration
- Mint/burn request workflows
- KYC submission and verification
- Admin endpoints for approvals and monitoring
- PostgreSQL database with SQLx
- Rate limiting and middleware

## Architecture

```
velopay-api/
├── src/
│   ├── main.rs           # Server entry point
│   ├── config.rs         # Configuration management
│   ├── models/           # Data models
│   ├── db/               # Database repositories
│   ├── services/         # Business logic layer
│   ├── routes/           # HTTP route handlers
│   ├── middleware/       # Auth, rate limiting
│   └── chain/            # Blockchain client (Subxt)
└── migrations/           # Database migrations

