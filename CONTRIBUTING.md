# Contributing to VeloPay

Thank you for your interest in contributing to VeloPay! This document provides guidelines and instructions for contributing to the project.

## Table of Contents
- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Setup](#development-setup)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Testing Guidelines](#testing-guidelines)
- [Security Guidelines](#security-guidelines)
- [Pull Request Process](#pull-request-process)
- [Commit Message Guidelines](#commit-message-guidelines)

---

## Code of Conduct

By participating in this project, you agree to maintain a respectful and inclusive environment for all contributors.

### Our Standards
- Be respectful and considerate
- Welcome newcomers and help them get started
- Focus on what is best for the community
- Show empathy towards other community members

---

## Getting Started

### Prerequisites
- **Rust**: 1.70 or higher
- **PostgreSQL**: 14 or higher
- **Node.js**: 16+ (for tooling)
- **Git**: Latest version

### Quick Start
```bash
# Clone the repository
git clone https://github.com/ChronoCoders/velopay.git
cd velopay

# Set up environment
cp velopay-api/.env.example velopay-api/.env
# Edit .env with your configuration

# Build blockchain
cd velo-chain
cargo build --release

# Set up database
cd ../velopay-api
sqlx database create
sqlx migrate run

# Run API
cargo run
```

---

## Development Setup

### 1. Blockchain Development

**Build the chain:**
```bash
cd velo-chain
cargo build --release
```

**Run local node:**
```bash
./target/release/velo-chain --dev --tmp
```

**Run two-node testnet:**
```bash
# Terminal 1 - Alice
./run-alice-final.bat

# Terminal 2 - Bob
./run-bob-final.bat
```

### 2. API Development

**Install SQLx CLI:**
```bash
cargo install sqlx-cli --no-default-features --features postgres
```

**Set up database:**
```bash
cd velopay-api
cp .env.example .env
# Edit .env with your DATABASE_URL

sqlx database create
sqlx migrate run
```

**Run API in development mode:**
```bash
cargo watch -x run
```

### 3. Environment Configuration

See `.env.example` for required environment variables. At minimum, set:
- `DATABASE_URL`
- `CHAIN_RPC_URL`
- `JWT_SECRET` (32+ characters)
- `ADMIN_API_KEY` (32+ characters)
- `ADMIN_SEED`

---

## Development Workflow

### 1. Create a Branch
```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/bug-description
```

Branch naming conventions:
- `feature/` - New features
- `fix/` - Bug fixes
- `docs/` - Documentation updates
- `refactor/` - Code refactoring
- `test/` - Test additions/modifications

### 2. Make Changes
- Write clean, well-documented code
- Follow Rust best practices
- Add tests for new functionality
- Update documentation as needed

### 3. Test Your Changes
```bash
# Run tests
cargo test

# Check formatting
cargo fmt --check

# Run linter
cargo clippy -- -D warnings

# Build to verify
cargo build --release
```

### 4. Commit Your Changes
Follow the [commit message guidelines](#commit-message-guidelines).

```bash
git add .
git commit -m "feat: add new feature description"
```

### 5. Push and Create PR
```bash
git push origin feature/your-feature-name
```

Then create a Pull Request on GitHub.

---

## Coding Standards

### Rust Code Style

**Follow Rust conventions:**
- Use `cargo fmt` for formatting
- Use `cargo clippy` for linting
- No compiler warnings allowed

**Naming conventions:**
```rust
// Types: PascalCase
struct UserAccount { }
enum RequestStatus { }

// Functions and variables: snake_case
fn process_payment() { }
let user_balance = 100;

// Constants: SCREAMING_SNAKE_CASE
const MAX_TRANSACTION_FEE: u32 = 1000;

// Traits: PascalCase with descriptive names
trait PaymentProcessor { }
```

**Error Handling:**
```rust
// ✅ Good - Use Result types
pub async fn create_user(email: &str) -> Result<User> {
    validate_email(email)?;
    // ...
}

// ✅ Good - Provide context
.map_err(|e| anyhow!("Failed to connect to database: {}", e))?

// ❌ Bad - Don't unwrap in library code
let user = find_user(id).unwrap();

// ❌ Bad - Don't ignore errors
let _ = database.execute(query);
```

**Documentation:**
```rust
/// Process a payment transaction
///
/// # Arguments
/// * `from` - Sender account address
/// * `to` - Recipient account address
/// * `amount` - Transaction amount
///
/// # Returns
/// * `Ok(String)` - Transaction hash
/// * `Err` - If transaction fails
///
/// # Example
/// ```
/// let tx_hash = process_payment("Alice", "Bob", 100).await?;
/// ```
pub async fn process_payment(
    from: &str,
    to: &str,
    amount: u128
) -> Result<String> {
    // Implementation
}
```

### Database Queries

**Always use SQLx parameterized queries:**
```rust
// ✅ Good
sqlx::query!("SELECT * FROM users WHERE email = $1", email)

// ❌ Never do this
sqlx::query(&format!("SELECT * FROM users WHERE email = '{}'", email))
```

### Security Practices

1. **Never commit secrets**
   - Use environment variables
   - Add sensitive files to `.gitignore`
   - Use `.env.example` for templates

2. **Validate all inputs**
   - Check wallet addresses
   - Validate email format
   - Verify amounts are positive

3. **Use proper error messages**
   - Don't leak sensitive information
   - Provide helpful context
   - Log errors appropriately

4. **Follow authentication best practices**
   - Use bcrypt for passwords
   - Implement rate limiting
   - Validate JWT tokens properly

---

## Testing Guidelines

### Unit Tests

**Write tests for all business logic:**
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_validation() {
        // Test valid password
        assert!(validate_password("SecurePass123!").is_ok());

        // Test invalid password (too short)
        assert!(validate_password("short").is_err());

        // Test invalid password (no special char)
        assert!(validate_password("NoSpecial123").is_err());
    }

    #[tokio::test]
    async fn test_user_registration() {
        // Setup
        let pool = setup_test_db().await;

        // Execute
        let result = register_user(&pool, "test@example.com", "SecurePass123!").await;

        // Assert
        assert!(result.is_ok());

        // Cleanup
        cleanup_test_db(&pool).await;
    }
}
```

### Integration Tests

**Test API endpoints:**
```rust
#[actix_web::test]
async fn test_login_endpoint() {
    let app = test::init_service(App::new().configure(routes::configure)).await;

    let req = test::TestRequest::post()
        .uri("/api/v1/auth/login")
        .set_json(&json!({
            "email": "test@example.com",
            "password": "SecurePass123!"
        }))
        .to_request();

    let resp = test::call_service(&app, req).await;
    assert_eq!(resp.status(), StatusCode::OK);
}
```

### Test Coverage

Aim for **70%+ code coverage** for critical paths:
- Authentication flows
- Payment processing
- Mint/burn workflows
- KYC verification

---

## Security Guidelines

### Before Submitting Code

**Security Checklist:**
- [ ] No secrets in code or commits
- [ ] All inputs validated
- [ ] SQL injection prevented (parameterized queries)
- [ ] XSS prevention (proper encoding)
- [ ] CSRF tokens where needed
- [ ] Rate limiting implemented
- [ ] Proper error handling (no info leakage)
- [ ] Authentication/authorization checked
- [ ] Logs don't contain sensitive data

### Reporting Security Issues

**DO NOT** create public GitHub issues for security vulnerabilities.

Instead:
1. Email security@velopay.com
2. Include:
   - Description of vulnerability
   - Steps to reproduce
   - Potential impact
   - Suggested fix (if any)
3. Allow 48 hours for initial response

See [SECURITY.md](SECURITY.md) for full details.

---

## Pull Request Process

### Before Creating a PR

1. **Update your branch:**
   ```bash
   git checkout main
   git pull origin main
   git checkout your-branch
   git rebase main
   ```

2. **Run all checks:**
   ```bash
   cargo test
   cargo fmt --check
   cargo clippy -- -D warnings
   cargo build --release
   ```

3. **Update documentation:**
   - Update README if needed
   - Add/update code comments
   - Update CHANGELOG.md

### Creating the PR

1. **Write a clear title:**
   - `feat: Add user authentication`
   - `fix: Resolve database connection leak`
   - `docs: Update API documentation`

2. **Fill out the PR template:**
   - Describe what changed
   - Explain why the change was needed
   - List any breaking changes
   - Include testing steps

3. **Link related issues:**
   - `Fixes #123`
   - `Relates to #456`

### PR Review Process

1. **Automated checks must pass:**
   - All tests passing
   - No clippy warnings
   - Code formatted correctly

2. **Code review:**
   - At least one approval required
   - Address all review comments
   - Respond to feedback promptly

3. **Merge:**
   - Squash and merge for clean history
   - Delete branch after merge

---

## Commit Message Guidelines

### Format
```
<type>(<scope>): <subject>

<body>

<footer>
```

### Types
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Maintenance tasks

### Examples

**Simple commit:**
```
feat: add password strength validation

Implement comprehensive password validation requiring:
- Minimum 12 characters
- Uppercase, lowercase, numbers, special characters
```

**Breaking change:**
```
feat!: update authentication flow

BREAKING CHANGE: JWT tokens now require refresh tokens.
Clients must implement token refresh logic.

Migration guide:
1. Update client to handle refresh tokens
2. Store refresh token securely
3. Implement token refresh before expiration
```

**Bug fix with issue reference:**
```
fix: resolve database connection leak

Properly close database connections in error paths.

Fixes #123
```

---

## Development Tips

### Useful Commands

```bash
# Watch for changes and rebuild
cargo watch -x run

# Run specific test
cargo test test_name

# Check for outdated dependencies
cargo outdated

# Generate documentation
cargo doc --open

# Run with backtrace
RUST_BACKTRACE=1 cargo run

# Profile compilation time
cargo build --timings
```

### Debugging

**Enable debug logging:**
```bash
RUST_LOG=debug cargo run
```

**Use rust-analyzer:**
Install rust-analyzer extension in your IDE for:
- Auto-completion
- Inline errors
- Go to definition
- Refactoring tools

### Performance

**Profile before optimizing:**
```bash
cargo build --release
perf record ./target/release/velopay-api
perf report
```

**Benchmark critical paths:**
```rust
#[bench]
fn bench_password_hashing(b: &mut Bencher) {
    b.iter(|| {
        hash("test_password", DEFAULT_COST)
    });
}
```

---

## Getting Help

### Resources
- **Documentation**: See [ARCHITECTURE.md](ARCHITECTURE.md)
- **Security**: See [SECURITY.md](SECURITY.md)
- **Testing**: See [TESTING.md](TESTING.md)
- **Substrate Docs**: https://docs.substrate.io/
- **Rust Book**: https://doc.rust-lang.org/book/

### Communication
- **GitHub Issues**: For bugs and feature requests
- **GitHub Discussions**: For questions and ideas
- **Email**: team@velopay.com for general inquiries

### Common Issues

**"Cannot connect to database"**
- Check DATABASE_URL in .env
- Ensure PostgreSQL is running
- Verify database exists: `sqlx database create`

**"Blockchain RPC connection failed"**
- Ensure blockchain node is running
- Check CHAIN_RPC_URL in .env
- Verify WebSocket port is accessible

**"Compilation errors after update"**
```bash
cargo clean
cargo build
```

---

## Recognition

Contributors will be acknowledged in:
- CHANGELOG.md for their contributions
- GitHub contributors page
- Release notes

Significant contributions may be highlighted in release announcements.

---

## License

By contributing to VeloPay, you agree that your contributions will be licensed under the same license as the project.

---

**Thank you for contributing to VeloPay!** 🚀

Your contributions help make blockchain payments more secure, accessible, and reliable for everyone.
