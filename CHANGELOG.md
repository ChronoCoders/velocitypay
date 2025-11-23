# Changelog

All notable changes to VeloPay will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Comprehensive project documentation (CONTRIBUTING.md, DEPLOYMENT.md)
- GitHub issue and pull request templates
- Enhanced .env.example with detailed comments

## [1.1.0] - 2024-11-23

### Added
- Password strength validation (12+ chars, complexity requirements)
- Wallet address format validation (SS58)
- Email format validation
- Transaction timeout handling (30-second timeout for blockchain operations)
- Request ID extraction from blockchain events
- Comprehensive health checks (database, blockchain, configuration)
- CORS origin validation at startup
- Strict rate limiting for auth endpoints (5 requests/minute)
- Production-ready database connection pool configuration
- Utils module for shared validation functions
- ARCHITECTURE.md - Comprehensive system architecture documentation
- SECURITY.md - Detailed security guidelines and best practices

### Changed
- Improved blockchain weights from hard-coded values to proper Weight::from_parts()
- Enhanced logging to sanitize sensitive data (amounts at debug level)
- Generic error messages to prevent email enumeration
- Updated database pool: 50 max connections, 5 min connections, with proper timeouts
- General endpoints rate limiting: 100 requests/minute

### Fixed
- Critical bug: Request ID extraction from blockchain events (was hard-coded to 0)
- Email enumeration vulnerability with generic error messages
- Missing input validation on critical operations
- Removed global warning suppressions that masked issues

### Security
- All critical security vulnerabilities from audit addressed
- Multi-layered authentication and authorization
- SQL injection prevention with SQLx parameterized queries
- Comprehensive input validation
- Secure logging practices

## [1.0.0] - 2024-11-20

### Added
- Initial release of VeloPay blockchain payment system
- Substrate blockchain with custom pallets:
  - VeloPay pallet (mint/burn workflows, transfers with fees)
  - KYC pallet (privacy-preserving document verification)
  - Compliance pallet (transaction monitoring, account flagging)
- REST API Gateway with Actix-web:
  - User authentication (JWT-based)
  - Payment processing
  - Mint/burn request workflows
  - KYC submission and verification
  - Admin approval system
- PostgreSQL database integration:
  - User management
  - Transaction history
  - Request tracking
  - KYC submissions
- Security features:
  - JWT authentication
  - Bcrypt password hashing
  - Rate limiting
  - CORS configuration
  - Admin API key authentication
- Request-approval workflow for mint/burn operations
- Reserved burn mechanism
- Configurable transaction fees (basis points)
- Emergency pause/unpause controls
- KYC verification gates for operations
- Compliance monitoring with daily limits
- Account flagging system
- Suspicious activity alerts
- Windows-compatible validator management scripts

### Documentation
- README.md with project overview
- TESTING.md with local testing guide
- API endpoint documentation
- Database schema

### Infrastructure
- SQLx for compile-time verified queries
- Subxt for blockchain integration
- Connection pooling
- Database migrations
- Docker support (planned)

---

## Version History

### Version Numbering

VeloPay follows Semantic Versioning (MAJOR.MINOR.PATCH):

- **MAJOR**: Incompatible API changes
- **MINOR**: Backwards-compatible functionality additions
- **PATCH**: Backwards-compatible bug fixes

### Release Types

- **Major Release (x.0.0)**: Breaking changes, major features
- **Minor Release (0.x.0)**: New features, backwards-compatible
- **Patch Release (0.0.x)**: Bug fixes, security patches

---

## Upgrade Guides

### Upgrading to 1.1.0 from 1.0.0

**Required Actions:**

1. **Update Environment Variables:**
   ```bash
   # Ensure JWT_SECRET is 32+ characters
   JWT_SECRET=$(openssl rand -base64 48)

   # Ensure ADMIN_API_KEY is 32+ characters
   ADMIN_API_KEY=$(openssl rand -base64 48)

   # Add CORS origins (no wildcards)
   CORS_ALLOWED_ORIGINS=https://app.example.com
   ```

2. **Database Migration:**
   ```bash
   # No schema changes, but connection pool configuration updated
   # Restart API service to apply new pool settings
   sudo systemctl restart velopay-api
   ```

3. **Update Password Requirements:**
   - All new passwords must meet strength requirements
   - Existing users will be prompted to update on next login

4. **Review Security Settings:**
   - Check SECURITY.md for new best practices
   - Review and update firewall rules
   - Configure monitoring per recommendations

**Breaking Changes:**
- None - All changes are backwards compatible

**New Features Available:**
- Enhanced health endpoint with dependency checks
- Improved error handling and logging
- Better rate limiting protection

---

## Deprecation Notices

### Deprecated in 1.1.0
- None

### Planned Deprecations
- Direct blockchain RPC access from API (will be abstracted)
- Unversioned API endpoints (will require /v1/ prefix)

---

## Security Advisories

### 2024-11-23 - Security Audit Findings

**Severity: Critical**

Multiple security vulnerabilities were identified and fixed in version 1.1.0:

1. **Request ID Bug** (Critical)
   - Issue: Mint/burn request IDs hard-coded to 0
   - Impact: Workflow completely broken
   - Fixed in: 1.1.0
   - Action: Upgrade immediately

2. **Weak Password Requirements** (High)
   - Issue: No password complexity enforcement
   - Impact: Vulnerable to brute force
   - Fixed in: 1.1.0
   - Action: Users should update passwords

3. **Email Enumeration** (Medium)
   - Issue: Different error messages reveal valid emails
   - Impact: Information disclosure
   - Fixed in: 1.1.0
   - Action: No user action needed

4. **Missing Rate Limiting** (High)
   - Issue: Auth endpoints not rate limited
   - Impact: Brute force attacks possible
   - Fixed in: 1.1.0
   - Action: Upgrade immediately

5. **Input Validation Gaps** (Medium)
   - Issue: Wallet addresses not validated
   - Impact: Failed blockchain transactions
   - Fixed in: 1.1.0
   - Action: No user action needed

All issues have been resolved in version 1.1.0. **Upgrade is strongly recommended.**

---

## Migration Guides

### Database Migrations

**From 1.0.0 to 1.1.0:**
- No schema changes required
- Connection pool settings updated in code
- Restart API service to apply

**Future Migrations:**
- Will be handled automatically via SQLx migrations
- Always backup database before upgrading

### API Changes

**Backwards Compatibility:**
- All 1.1.0 changes are backwards compatible
- Existing API clients will continue to work
- New validation may reject previously accepted invalid inputs

---

## Known Issues

### Current Limitations

1. **Windows Node Keys** (Low Priority)
   - Windows batch scripts for node key generation
   - Recommendation: Use WSL2 or Linux for production

2. **Frontend Missing** (Planned)
   - No web interface yet
   - Planned: SvelteKit frontend
   - Timeline: Q1 2025

3. **Docker Images** (Planned)
   - Manual deployment currently required
   - Docker/Kubernetes support planned
   - Timeline: Q4 2024

### Workarounds

1. **Windows Development:**
   - Use WSL2 for best experience
   - Or use batch scripts provided

2. **API Testing:**
   - Use Postman or curl
   - API documentation in README.md

---

## Roadmap

### Version 1.2.0 (Planned - Q4 2024)
- [ ] Multi-signature support for admin operations
- [ ] Circuit breaker pattern implementation
- [ ] Event sourcing for audit trail
- [ ] Redis caching layer
- [ ] Prometheus metrics export
- [ ] Docker and Kubernetes support

### Version 1.3.0 (Planned - Q1 2025)
- [ ] SvelteKit web frontend
- [ ] Mobile app (React Native)
- [ ] Enhanced compliance reporting
- [ ] Cross-chain bridge support
- [ ] Hardware wallet integration

### Version 2.0.0 (Planned - Q2 2025)
- [ ] Complete API redesign (breaking changes)
- [ ] GraphQL support
- [ ] Advanced analytics dashboard
- [ ] Multi-tenant support
- [ ] Enhanced permissions system

---

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on:
- Reporting bugs
- Suggesting features
- Submitting pull requests
- Development workflow

---

## Support

### Getting Help

- **Documentation**: See [README.md](README.md) and [ARCHITECTURE.md](ARCHITECTURE.md)
- **Security Issues**: See [SECURITY.md](SECURITY.md)
- **Bug Reports**: Create a GitHub issue
- **Questions**: GitHub Discussions
- **Email**: team@velopay.com

### Release Notes

Detailed release notes are available in GitHub Releases:
https://github.com/ChronoCoders/velopay/releases

---

**Note**: This changelog is maintained manually. For a complete history of all changes, see the Git commit log.

[Unreleased]: https://github.com/ChronoCoders/velopay/compare/v1.1.0...HEAD
[1.1.0]: https://github.com/ChronoCoders/velopay/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/ChronoCoders/velopay/releases/tag/v1.0.0
