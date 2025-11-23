# VeloPay Security Documentation

## Table of Contents
1. [Security Overview](#security-overview)
2. [Implemented Security Controls](#implemented-security-controls)
3. [Configuration Security](#configuration-security)
4. [Deployment Security](#deployment-security)
5. [Incident Response](#incident-response)
6. [Security Checklist](#security-checklist)
7. [Vulnerability Reporting](#vulnerability-reporting)

---

## Security Overview

VeloPay implements multiple layers of security to protect user funds, data, and system integrity. This document outlines security controls, best practices, and incident response procedures.

### Security Principles
- **Defense in Depth**: Multiple security layers
- **Least Privilege**: Minimal access rights
- **Secure by Default**: Safe defaults, explicit opt-in for risky features
- **Audit Trail**: Comprehensive logging of security events
- **Fail Secure**: System fails to safe state

---

## Implemented Security Controls

### 1. Authentication & Authorization

#### Password Security
✅ **Implemented**
- Minimum 12 characters
- Complexity requirements (uppercase, lowercase, numbers, special chars)
- Bcrypt hashing with cost factor 12
- No password in logs or error messages

**Location**: `velopay-api/src/services/auth_service.rs`

```rust
// Password validation
- Min length: 12
- Uppercase: Required
- Lowercase: Required
- Numbers: Required
- Special chars: Required
```

#### JWT Tokens
✅ **Implemented**
- HS256 signing algorithm
- 24-hour expiration (configurable)
- Secret must be 32+ characters
- Validated on every protected request

**Location**: `velopay-api/src/middleware/auth.rs`

**Best Practices**:
- Rotate JWT secret periodically
- Use secure random generator for secret
- Never commit secrets to version control
- Use environment variables for secrets

#### Admin API Keys
✅ **Implemented**
- Separate authentication mechanism
- Minimum 32 characters
- Validated at startup

**Location**: `velopay-api/src/middleware/admin.rs`

### 2. Rate Limiting

✅ **Implemented**
- Auth endpoints: 5 requests/minute (prevents brute force)
- General endpoints: 100 requests/minute (prevents DoS)
- Per-IP tracking with token bucket algorithm

**Location**: `velopay-api/src/main.rs`

**Tuning Guidelines**:
- Increase limits for trusted IPs
- Decrease limits in production
- Monitor 429 responses
- Alert on repeated limit violations

### 3. Input Validation

#### Wallet Address Validation
✅ **Implemented**
- SS58 address format validation
- Prevents invalid blockchain transactions
- Early rejection of malformed input

**Location**: `velopay-api/src/utils.rs`

#### Email Validation
✅ **Implemented**
- Format validation
- Length constraints (5-255 chars)

**Location**: `velopay-api/src/utils.rs`

#### Amount Validation
✅ **Implemented**
- Non-zero validation
- Positive amount validation
- Overflow prevention

**Location**: Multiple pallets and services

### 4. SQL Injection Prevention

✅ **Implemented**
- SQLx compile-time query verification
- Parameterized queries only
- No string concatenation for queries

**Location**: All `velopay-api/src/db/*_repository.rs` files

**Example**:
```rust
// ✅ SAFE - Parameterized
sqlx::query!("SELECT * FROM users WHERE email = $1", email)

// ❌ NEVER DO THIS
sqlx::query(&format!("SELECT * FROM users WHERE email = '{}'", email))
```

### 5. CORS Policy

✅ **Implemented**
- Whitelist-only origins
- No wildcard (*) allowed in production
- Validated at startup
- Explicit allowed methods and headers

**Location**: `velopay-api/src/main.rs`, `velopay-api/src/config.rs`

**Configuration**:
```env
CORS_ALLOWED_ORIGINS=https://app.velopay.com,https://admin.velopay.com
```

### 6. Blockchain Security

#### Transaction Timeouts
✅ **Implemented**
- 30-second timeout for all operations
- Prevents indefinite hanging
- Proper error handling

**Location**: `velopay-api/src/chain/operations.rs`

#### Request ID Validation
✅ **Implemented**
- Proper event parsing
- Unique request ID extraction
- Database correlation

**Location**: `velopay-api/src/chain/operations.rs`

#### Access Control
✅ **Implemented**
- Mint Authority
- Burn Authority
- KYC Verifier
- Compliance Officer
- Sudo (emergency)

**Location**: `velo-chain/pallets/*/src/lib.rs`

### 7. Logging & Audit Trail

#### Sensitive Data Protection
✅ **Implemented**
- Amounts logged at debug level only
- Addresses sanitized from info logs
- No passwords in logs
- Transaction hashes for correlation

**Location**: `velopay-api/src/chain/operations.rs`

**Log Levels**:
- `ERROR`: System failures, security violations
- `WARN`: Configuration issues, unusual activity
- `INFO`: Request/response, state changes
- `DEBUG`: Sensitive data, detailed traces

---

## Configuration Security

### Environment Variables

**Critical Secrets** (MUST be set securely):
```env
# Generate with: openssl rand -base64 32
JWT_SECRET=<secure-random-32+-chars>

# Generate with: openssl rand -base64 32
ADMIN_API_KEY=<secure-random-32+-chars>

# Blockchain seed phrase - NEVER commit
ADMIN_SEED=<12-24-word-mnemonic>

# Database with SSL
DATABASE_URL=postgresql://user:pass@host:5432/velopay?sslmode=require
```

**Configuration** (Can be environment-specific):
```env
CORS_ALLOWED_ORIGINS=https://production.com
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
CHAIN_RPC_URL=wss://blockchain.velopay.com
```

### Secret Management

**Development**:
- Use `.env` file (never commit)
- Rotate secrets regularly
- Different secrets per developer

**Production**:
- Use secret management service (AWS Secrets Manager, HashiCorp Vault)
- Rotate secrets automatically
- Audit secret access
- Encrypt secrets at rest

### Database Security

**Connection Security**:
```env
DATABASE_URL=postgresql://user:pass@host:5432/velopay?sslmode=require
```

**Connection Pool**:
- Max 50 connections
- Acquire timeout: 10 seconds
- Idle timeout: 10 minutes
- Max lifetime: 30 minutes
- Test before acquire: enabled

**Best Practices**:
- Use SSL/TLS for connections
- Separate read/write users
- Principle of least privilege
- Regular backups
- Encryption at rest

---

## Deployment Security

### TLS/SSL Configuration

**Minimum Requirements**:
- TLS 1.2+ only
- Strong cipher suites
- HSTS headers
- Certificate pinning (optional)

**Nginx Example**:
```nginx
server {
    listen 443 ssl http2;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256';
    ssl_prefer_server_ciphers on;

    # HSTS
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;

    # Security headers
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;
}
```

### Firewall Rules

**Inbound** (Production):
- Port 443: HTTPS (from anywhere)
- Port 22: SSH (from bastion only)
- Port 5432: PostgreSQL (from API nodes only)
- Port 9944: Blockchain RPC (from API nodes only)

**Outbound**:
- Allow all (or restrict to known services)

### Container Security

**Docker Best Practices**:
```dockerfile
# Use minimal base images
FROM rust:1.70-slim

# Don't run as root
USER 1000:1000

# Read-only root filesystem
--read-only
--tmpfs /tmp

# Drop capabilities
--cap-drop=ALL

# Resource limits
--memory=2g
--cpus=2
```

### Monitoring & Alerting

**Security Metrics**:
- Failed authentication attempts
- Rate limit violations
- Unusual transaction patterns
- Database connection failures
- Blockchain synchronization lag

**Alert Thresholds**:
- >10 failed logins/minute from same IP
- >50 rate limit violations/hour
- Health check failures
- Blockchain RPC disconnection
- Database connection pool exhaustion

---

## Incident Response

### Security Incident Types

1. **Unauthorized Access**
   - Compromised credentials
   - Brute force attacks
   - Session hijacking

2. **Data Breach**
   - Database exposure
   - API key leakage
   - Configuration exposure

3. **DoS/DDoS**
   - Network flooding
   - Application-level attacks
   - Resource exhaustion

4. **Smart Contract Exploit**
   - Pallet vulnerabilities
   - Logic errors
   - Access control bypass

### Incident Response Plan

#### Phase 1: Detection
- Monitor logs and alerts
- User reports
- Automated scanning
- Health check failures

#### Phase 2: Containment
1. Assess impact and scope
2. Isolate affected systems
3. Block malicious IPs
4. Revoke compromised credentials
5. Pause system if necessary (`pallet_velopay::pause`)

#### Phase 3: Eradication
1. Identify root cause
2. Patch vulnerabilities
3. Remove malicious code
4. Update dependencies

#### Phase 4: Recovery
1. Restore from backups if needed
2. Verify system integrity
3. Resume operations (`pallet_velopay::unpause`)
4. Monitor closely

#### Phase 5: Post-Incident
1. Document incident
2. Update runbooks
3. Improve monitoring
4. Security training

### Emergency Contacts

**Security Team**:
- Primary: [security@velopay.com]
- Secondary: [on-call engineer]
- Escalation: [CTO/CISO]

**External Resources**:
- Legal counsel
- PR team
- Law enforcement (if required)
- Substrate security team

---

## Security Checklist

### Pre-Production Checklist

**Environment**:
- [ ] All secrets use secure random generation (32+ chars)
- [ ] No secrets in code or version control
- [ ] Environment variables validated at startup
- [ ] CORS origins explicitly whitelisted
- [ ] Rate limiting configured appropriately
- [ ] TLS/SSL certificates valid and configured
- [ ] Firewall rules tested

**Authentication**:
- [ ] Password complexity enforced
- [ ] JWT expiration appropriate
- [ ] Admin API keys rotated
- [ ] Session management tested

**Database**:
- [ ] SSL/TLS connections enabled
- [ ] Connection pool configured
- [ ] Backups automated
- [ ] Encryption at rest enabled
- [ ] Access control verified

**Blockchain**:
- [ ] Admin seed secure and backed up
- [ ] Authority accounts set correctly
- [ ] Emergency pause mechanism tested
- [ ] Multi-sig considered for production

**Monitoring**:
- [ ] Logging configured
- [ ] Metrics collection enabled
- [ ] Alerts configured
- [ ] Health checks working
- [ ] Incident response plan documented

### Ongoing Security Tasks

**Daily**:
- Review security logs
- Check alert dashboard
- Monitor unusual activity

**Weekly**:
- Review access logs
- Check for failed authentications
- Update security patches

**Monthly**:
- Rotate credentials
- Review user permissions
- Test backup restoration
- Security training

**Quarterly**:
- Penetration testing
- Dependency audit
- Security policy review
- Disaster recovery drill

---

## Vulnerability Reporting

### Reporting Process

If you discover a security vulnerability:

1. **DO NOT** create a public GitHub issue
2. Email [security@velopay.com] with:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if known)
3. Allow 48 hours for initial response
4. Work with security team on coordinated disclosure

### Vulnerability Severity Levels

**Critical** (fix within 24 hours):
- Remote code execution
- Authentication bypass
- Fund theft possible

**High** (fix within 7 days):
- Data exposure
- Privilege escalation
- DoS attacks

**Medium** (fix within 30 days):
- Information disclosure
- Rate limiting bypass
- CSRF vulnerabilities

**Low** (fix within 90 days):
- Minor information leaks
- Best practice violations
- Configuration issues

### Bug Bounty Program

*To be announced - watch this space*

---

## Security Updates

### Recent Security Improvements

**2024-11-23**: Comprehensive security audit fixes
- Implemented request ID extraction from events
- Added password strength validation
- Fixed email enumeration vulnerability
- Added strict rate limiting for auth endpoints
- Implemented wallet address validation
- Sanitized sensitive data in logs
- Added transaction timeout handling
- Enhanced health checks
- Validated CORS configuration

### Upcoming Security Enhancements

**Planned**:
- Multi-signature support for high-value operations
- Circuit breaker pattern
- Event sourcing for complete audit trail
- Redis caching with secure configuration
- Prometheus metrics export
- Hardware wallet support
- Automated security scanning in CI/CD

---

## Compliance

### Regulatory Considerations

**KYC/AML**:
- Document hash storage (privacy-preserving)
- Multi-status workflow
- Verification process
- Audit trail

**Data Privacy**:
- GDPR considerations
- Data minimization
- User consent
- Right to deletion

**Financial Regulations**:
- Transaction monitoring
- Suspicious activity reporting
- Daily transaction limits
- Compliance officer role

### Audit Trail

**Blockchain Events**:
- All mint/burn operations logged
- Transfer history immutable
- Request status transitions tracked

**Database Records**:
- User registrations
- Transaction history
- Request approvals/rejections
- KYC submissions

**System Logs**:
- Authentication attempts
- API requests
- Configuration changes
- Security events

---

## References

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [CWE Top 25](https://cwe.mitre.org/top25/)
- [Substrate Security](https://docs.substrate.io/build/troubleshoot-your-code/)
- [Rust Security Guidelines](https://anssi-fr.github.io/rust-guide/)
- [SQLx Security](https://github.com/launchbadge/sqlx/blob/main/SECURITY.md)

---

**Last Updated**: 2024-11-23
**Version**: 1.0.0
**Status**: Active
