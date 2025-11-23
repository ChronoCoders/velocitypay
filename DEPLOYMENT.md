# VeloPay Deployment Guide

This guide provides step-by-step instructions for deploying VeloPay to production.

## Table of Contents
- [Pre-Deployment Checklist](#pre-deployment-checklist)
- [Infrastructure Requirements](#infrastructure-requirements)
- [Production Build](#production-build)
- [Database Setup](#database-setup)
- [Blockchain Node Deployment](#blockchain-node-deployment)
- [API Gateway Deployment](#api-gateway-deployment)
- [Security Configuration](#security-configuration)
- [Monitoring Setup](#monitoring-setup)
- [Post-Deployment Verification](#post-deployment-verification)
- [Rollback Procedures](#rollback-procedures)

---

## Pre-Deployment Checklist

Before deploying to production, ensure:

### Security
- [ ] All secrets generated using cryptographically secure random generators
- [ ] No secrets committed to version control
- [ ] TLS/SSL certificates obtained and configured
- [ ] Firewall rules defined and tested
- [ ] Database backups configured
- [ ] Incident response plan documented

### Configuration
- [ ] Production environment variables set
- [ ] CORS origins configured for production domains
- [ ] Rate limiting tuned for production load
- [ ] Database connection pool sized appropriately
- [ ] Log levels configured (INFO for production)

### Testing
- [ ] All tests passing
- [ ] Security audit completed
- [ ] Load testing performed
- [ ] Disaster recovery tested
- [ ] Health checks validated

### Documentation
- [ ] Runbooks created
- [ ] On-call procedures documented
- [ ] Deployment steps validated
- [ ] Rollback procedures tested

---

## Infrastructure Requirements

### Minimum Production Setup

**Blockchain Nodes** (2+ for redundancy):
- CPU: 4 cores
- RAM: 8 GB
- Storage: 500 GB SSD
- Network: 100 Mbps
- OS: Ubuntu 20.04 LTS or higher

**API Servers** (2+ for high availability):
- CPU: 2 cores
- RAM: 4 GB
- Storage: 50 GB SSD
- Network: 100 Mbps
- OS: Ubuntu 20.04 LTS or higher

**Database** (Primary + Replica):
- CPU: 4 cores
- RAM: 16 GB
- Storage: 500 GB SSD (with automatic backups)
- Network: 1 Gbps
- PostgreSQL 14+

**Load Balancer**:
- Nginx or HAProxy
- SSL/TLS termination
- Health check configuration

---

## Production Build

### 1. Build Blockchain Node

```bash
# Clone repository
git clone https://github.com/ChronoCoders/velopay.git
cd velopay/velo-chain

# Build in release mode
cargo build --release

# Verify binary
./target/release/velo-chain --version

# Optional: Strip binary to reduce size
strip ./target/release/velo-chain

# Copy to deployment location
sudo cp ./target/release/velo-chain /usr/local/bin/
```

### 2. Build API Gateway

```bash
cd ../velopay-api

# Build in release mode
cargo build --release

# Verify binary
./target/release/velopay-api --version

# Copy to deployment location
sudo cp ./target/release/velopay-api /usr/local/bin/
```

---

## Database Setup

### 1. Install PostgreSQL

```bash
# Ubuntu/Debian
sudo apt update
sudo apt install postgresql-14 postgresql-contrib-14

# Start PostgreSQL
sudo systemctl start postgresql
sudo systemctl enable postgresql
```

### 2. Create Production Database

```bash
# Switch to postgres user
sudo -u postgres psql

# In PostgreSQL shell:
CREATE DATABASE velopay_production;
CREATE USER velopay_api WITH ENCRYPTED PASSWORD 'STRONG_PASSWORD_HERE';
GRANT ALL PRIVILEGES ON DATABASE velopay_production TO velopay_api;

# Enable SSL
ALTER SYSTEM SET ssl = on;
SELECT pg_reload_conf();

\q
```

### 3. Configure PostgreSQL for Production

Edit `/etc/postgresql/14/main/postgresql.conf`:

```conf
# Connection Settings
max_connections = 200
shared_buffers = 4GB
effective_cache_size = 12GB
maintenance_work_mem = 1GB
checkpoint_completion_target = 0.9
wal_buffers = 16MB
default_statistics_target = 100
random_page_cost = 1.1
effective_io_concurrency = 200
work_mem = 20MB
min_wal_size = 1GB
max_wal_size = 4GB

# Security
ssl = on
password_encryption = scram-sha-256
```

Edit `/etc/postgresql/14/main/pg_hba.conf`:

```conf
# TYPE  DATABASE        USER            ADDRESS                 METHOD
hostssl velopay_production velopay_api 0.0.0.0/0              scram-sha-256
```

Restart PostgreSQL:
```bash
sudo systemctl restart postgresql
```

### 4. Run Migrations

```bash
# Install SQLx CLI
cargo install sqlx-cli --no-default-features --features postgres

# Set database URL
export DATABASE_URL="postgresql://velopay_api:PASSWORD@localhost/velopay_production?sslmode=require"

# Run migrations
cd velopay-api
sqlx migrate run
```

### 5. Set Up Database Backups

```bash
# Create backup script
sudo cat > /usr/local/bin/backup-velopay-db.sh <<'EOF'
#!/bin/bash
BACKUP_DIR="/var/backups/velopay"
DATE=$(date +%Y%m%d_%H%M%S)
mkdir -p $BACKUP_DIR

pg_dump -U velopay_api -h localhost velopay_production | \
    gzip > $BACKUP_DIR/velopay_$DATE.sql.gz

# Keep only last 30 days
find $BACKUP_DIR -name "velopay_*.sql.gz" -mtime +30 -delete
EOF

sudo chmod +x /usr/local/bin/backup-velopay-db.sh

# Add to crontab (daily at 2 AM)
sudo crontab -e
# Add: 0 2 * * * /usr/local/bin/backup-velopay-db.sh
```

---

## Blockchain Node Deployment

### 1. Generate Node Keys

```bash
# Generate node key
velo-chain key generate-node-key --file /var/lib/velo-chain/node-key

# Generate session keys
velo-chain key generate --scheme Sr25519
```

### 2. Create Systemd Service

```bash
sudo cat > /etc/systemd/system/velo-chain.service <<EOF
[Unit]
Description=VeloPay Blockchain Node
After=network.target

[Service]
Type=simple
User=velopay
WorkingDirectory=/var/lib/velo-chain
ExecStart=/usr/local/bin/velo-chain \\
    --base-path /var/lib/velo-chain \\
    --chain local \\
    --node-key-file /var/lib/velo-chain/node-key \\
    --validator \\
    --rpc-cors all \\
    --unsafe-rpc-external \\
    --unsafe-ws-external \\
    --rpc-methods=Unsafe \\
    --prometheus-external

Restart=always
RestartSec=10

[Install]
WantedBy=multi-user.target
EOF

# Create service user
sudo useradd -r -s /bin/false velopay
sudo mkdir -p /var/lib/velo-chain
sudo chown velopay:velopay /var/lib/velo-chain

# Start service
sudo systemctl daemon-reload
sudo systemctl enable velo-chain
sudo systemctl start velo-chain

# Check status
sudo systemctl status velo-chain
sudo journalctl -u velo-chain -f
```

### 3. Configure Firewall

```bash
# Allow blockchain ports
sudo ufw allow 30333/tcp  # P2P
sudo ufw allow 9944/tcp   # WebSocket RPC
sudo ufw allow 9933/tcp   # HTTP RPC
sudo ufw allow 9615/tcp   # Prometheus

# Enable firewall
sudo ufw enable
```

---

## API Gateway Deployment

### 1. Create Environment File

```bash
sudo mkdir -p /etc/velopay
sudo cat > /etc/velopay/api.env <<EOF
# Database
DATABASE_URL=postgresql://velopay_api:PASSWORD@localhost/velopay_production?sslmode=require

# Blockchain
CHAIN_RPC_URL=ws://localhost:9944

# Server
SERVER_HOST=0.0.0.0
SERVER_PORT=8080

# Security (GENERATE THESE SECURELY!)
JWT_SECRET=$(openssl rand -base64 48)
ADMIN_API_KEY=$(openssl rand -base64 48)
ADMIN_SEED="your-secure-seed-phrase-here"

# CORS
CORS_ALLOWED_ORIGINS=https://app.velopay.com,https://admin.velopay.com

# Rate Limiting
RATE_LIMIT_REQUESTS=100
RATE_LIMIT_WINDOW_SECONDS=60

# JWT
JWT_EXPIRATION=86400
EOF

sudo chmod 600 /etc/velopay/api.env
sudo chown velopay:velopay /etc/velopay/api.env
```

### 2. Create Systemd Service

```bash
sudo cat > /etc/systemd/system/velopay-api.service <<EOF
[Unit]
Description=VeloPay API Gateway
After=network.target postgresql.service

[Service]
Type=simple
User=velopay
EnvironmentFile=/etc/velopay/api.env
ExecStart=/usr/local/bin/velopay-api

Restart=always
RestartSec=10

# Security
NoNewPrivileges=true
PrivateTmp=true
ProtectSystem=strict
ProtectHome=true
ReadWritePaths=/var/log/velopay

[Install]
WantedBy=multi-user.target
EOF

# Create log directory
sudo mkdir -p /var/log/velopay
sudo chown velopay:velopay /var/log/velopay

# Start service
sudo systemctl daemon-reload
sudo systemctl enable velopay-api
sudo systemctl start velopay-api

# Check status
sudo systemctl status velopay-api
sudo journalctl -u velopay-api -f
```

### 3. Configure Nginx (Load Balancer)

```bash
sudo apt install nginx

sudo cat > /etc/nginx/sites-available/velopay <<'EOF'
upstream velopay_api {
    least_conn;
    server 127.0.0.1:8080 max_fails=3 fail_timeout=30s;
    # Add more API servers for HA:
    # server 10.0.1.2:8080 max_fails=3 fail_timeout=30s;
}

# Redirect HTTP to HTTPS
server {
    listen 80;
    listen [::]:80;
    server_name api.velopay.com;
    return 301 https://$server_name$request_uri;
}

# HTTPS Server
server {
    listen 443 ssl http2;
    listen [::]:443 ssl http2;
    server_name api.velopay.com;

    # SSL Configuration
    ssl_certificate /etc/letsencrypt/live/api.velopay.com/fullchain.pem;
    ssl_certificate_key /etc/letsencrypt/live/api.velopay.com/privkey.pem;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers 'ECDHE-ECDSA-AES128-GCM-SHA256:ECDHE-RSA-AES128-GCM-SHA256';
    ssl_prefer_server_ciphers on;
    ssl_session_cache shared:SSL:10m;

    # Security Headers
    add_header Strict-Transport-Security "max-age=31536000; includeSubDomains" always;
    add_header X-Frame-Options "DENY" always;
    add_header X-Content-Type-Options "nosniff" always;
    add_header X-XSS-Protection "1; mode=block" always;

    # Logging
    access_log /var/log/nginx/velopay-access.log;
    error_log /var/log/nginx/velopay-error.log;

    # Proxy Settings
    location / {
        proxy_pass http://velopay_api;
        proxy_http_version 1.1;
        proxy_set_header Upgrade $http_upgrade;
        proxy_set_header Connection 'upgrade';
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_cache_bypass $http_upgrade;

        # Timeouts
        proxy_connect_timeout 60s;
        proxy_send_timeout 60s;
        proxy_read_timeout 60s;
    }

    # Health check endpoint
    location /health {
        proxy_pass http://velopay_api/health;
        access_log off;
    }
}
EOF

# Enable site
sudo ln -s /etc/nginx/sites-available/velopay /etc/nginx/sites-enabled/
sudo nginx -t
sudo systemctl reload nginx
```

### 4. Configure SSL with Let's Encrypt

```bash
sudo apt install certbot python3-certbot-nginx

# Obtain certificate
sudo certbot --nginx -d api.velopay.com

# Auto-renewal is configured automatically
sudo systemctl status certbot.timer
```

---

## Security Configuration

### 1. Generate Secure Secrets

```bash
# Generate JWT secret (48 bytes = 64 base64 characters)
openssl rand -base64 48

# Generate Admin API key
openssl rand -base64 48

# For blockchain admin seed, use a proper BIP39 mnemonic generator
# NEVER use simple passphrases in production
```

### 2. Configure Firewall

```bash
# API Server
sudo ufw allow 22/tcp    # SSH (restrict to bastion IP)
sudo ufw allow 443/tcp   # HTTPS
sudo ufw deny 8080/tcp   # Block direct API access

# Blockchain Node
sudo ufw allow 30333/tcp # P2P
sudo ufw allow from API_SERVER_IP to any port 9944 proto tcp  # RPC

sudo ufw enable
```

### 3. Set Up Fail2Ban

```bash
sudo apt install fail2ban

sudo cat > /etc/fail2ban/jail.local <<EOF
[DEFAULT]
bantime = 3600
findtime = 600
maxretry = 5

[sshd]
enabled = true

[nginx-limit-req]
enabled = true
filter = nginx-limit-req
logpath = /var/log/nginx/velopay-error.log
EOF

sudo systemctl restart fail2ban
```

---

## Monitoring Setup

### 1. Install Prometheus

```bash
# Download Prometheus
wget https://github.com/prometheus/prometheus/releases/download/v2.40.0/prometheus-2.40.0.linux-amd64.tar.gz
tar xvf prometheus-2.40.0.linux-amd64.tar.gz
sudo mv prometheus-2.40.0.linux-amd64 /opt/prometheus

# Configure Prometheus
sudo cat > /opt/prometheus/prometheus.yml <<EOF
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'velo-chain'
    static_configs:
      - targets: ['localhost:9615']

  - job_name: 'node-exporter'
    static_configs:
      - targets: ['localhost:9100']
EOF

# Create systemd service
sudo cat > /etc/systemd/system/prometheus.service <<EOF
[Unit]
Description=Prometheus
After=network.target

[Service]
User=prometheus
ExecStart=/opt/prometheus/prometheus --config.file=/opt/prometheus/prometheus.yml
Restart=always

[Install]
WantedBy=multi-user.target
EOF

sudo useradd -r -s /bin/false prometheus
sudo systemctl daemon-reload
sudo systemctl enable prometheus
sudo systemctl start prometheus
```

### 2. Install Node Exporter

```bash
wget https://github.com/prometheus/node_exporter/releases/download/v1.5.0/node_exporter-1.5.0.linux-amd64.tar.gz
tar xvf node_exporter-1.5.0.linux-amd64.tar.gz
sudo mv node_exporter-1.5.0.linux-amd64/node_exporter /usr/local/bin/

sudo cat > /etc/systemd/system/node-exporter.service <<EOF
[Unit]
Description=Node Exporter
After=network.target

[Service]
User=node-exporter
ExecStart=/usr/local/bin/node_exporter

[Install]
WantedBy=multi-user.target
EOF

sudo useradd -r -s /bin/false node-exporter
sudo systemctl daemon-reload
sudo systemctl enable node-exporter
sudo systemctl start node-exporter
```

### 3. Configure Logging

```bash
# Centralized logging with rsyslog
sudo apt install rsyslog

# Log rotation
sudo cat > /etc/logrotate.d/velopay <<EOF
/var/log/velopay/*.log {
    daily
    rotate 30
    compress
    delaycompress
    missingok
    notifempty
    create 0640 velopay velopay
    sharedscripts
    postrotate
        systemctl reload velopay-api > /dev/null 2>&1 || true
    endscript
}
EOF
```

---

## Post-Deployment Verification

### 1. Health Checks

```bash
# Check API health
curl -k https://api.velopay.com/health

# Expected response:
# {
#   "status": "healthy",
#   "service": "velopay-api",
#   "checks": {
#     "database": {"status": "healthy"},
#     "blockchain": {"status": "healthy"},
#     "configuration": {"status": "healthy"}
#   }
# }
```

### 2. Functional Tests

```bash
# Test registration
curl -X POST https://api.velopay.com/api/v1/auth/register \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePassword123!",
    "wallet_address": "5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY"
  }'

# Test login
curl -X POST https://api.velopay.com/api/v1/auth/login \
  -H "Content-Type: application/json" \
  -d '{
    "email": "test@example.com",
    "password": "SecurePassword123!"
  }'
```

### 3. Performance Tests

```bash
# Install Apache Bench
sudo apt install apache2-utils

# Simple load test
ab -n 1000 -c 10 https://api.velopay.com/health

# Monitor during load test
watch -n 1 'systemctl status velopay-api'
```

---

## Rollback Procedures

### 1. API Rollback

```bash
# Stop current version
sudo systemctl stop velopay-api

# Restore previous binary
sudo cp /usr/local/bin/velopay-api.backup /usr/local/bin/velopay-api

# Start service
sudo systemctl start velopay-api

# Verify
curl https://api.velopay.com/health
```

### 2. Database Rollback

```bash
# Restore from backup
gunzip < /var/backups/velopay/velopay_YYYYMMDD_HHMMSS.sql.gz | \
  psql -U velopay_api -h localhost velopay_production
```

### 3. Blockchain Rollback

```bash
# Stop node
sudo systemctl stop velo-chain

# Remove database
rm -rf /var/lib/velo-chain/chains/*

# Restore from snapshot or re-sync
# Start node
sudo systemctl start velo-chain
```

---

## Maintenance Tasks

### Daily
- Monitor logs for errors
- Check disk space
- Review security alerts

### Weekly
- Review database performance
- Check backup integrity
- Update dependencies

### Monthly
- Security patches
- Rotate credentials
- Performance optimization
- Disaster recovery drill

---

## Troubleshooting

See [ARCHITECTURE.md](ARCHITECTURE.md) for common issues and solutions.

For emergencies, follow the incident response plan in [SECURITY.md](SECURITY.md).

---

**Deployment complete!** Your VeloPay instance should now be running in production. 🚀
