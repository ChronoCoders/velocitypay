#!/bin/bash
# Development Database Setup Script for VelocityPay
# This script sets up the PostgreSQL database for local development

set -e  # Exit on error

echo "========================================="
echo "VelocityPay Development Database Setup"
echo "========================================="
echo ""

# Database configuration
DB_NAME="velocitypay"
DB_USER="${POSTGRES_USER:-postgres}"
DB_HOST="${POSTGRES_HOST:-localhost}"
DB_PORT="${POSTGRES_PORT:-5432}"

echo "Configuration:"
echo "  Database: $DB_NAME"
echo "  User: $DB_USER"
echo "  Host: $DB_HOST"
echo "  Port: $DB_PORT"
echo ""

# Check if PostgreSQL is installed
if ! command -v psql &> /dev/null; then
    echo "❌ ERROR: PostgreSQL is not installed or not in PATH"
    echo ""
    echo "Install PostgreSQL:"
    echo "  - Ubuntu/Debian: sudo apt-get install postgresql"
    echo "  - macOS: brew install postgresql"
    echo "  - Arch Linux: sudo pacman -S postgresql"
    exit 1
fi

echo "✓ PostgreSQL found"

# Check if PostgreSQL is running
if ! pg_isready -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" &> /dev/null; then
    echo "⚠️  WARNING: PostgreSQL does not appear to be running"
    echo ""
    echo "Start PostgreSQL:"
    echo "  - Linux: sudo systemctl start postgresql"
    echo "  - macOS: brew services start postgresql"
    echo ""
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 1
    fi
fi

# Check if database already exists
if psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -lqt | cut -d \| -f 1 | grep -qw "$DB_NAME"; then
    echo ""
    echo "⚠️  Database '$DB_NAME' already exists!"
    echo ""
    read -p "Drop and recreate? This will DELETE ALL DATA! (y/N) " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        echo "Dropping database..."
        dropdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME" || {
            echo "❌ Failed to drop database. You may need to disconnect all clients."
            exit 1
        }
        echo "✓ Database dropped"
    else
        echo "Skipping database creation..."
        echo ""
        read -p "Run migrations anyway? (y/N) " -n 1 -r
        echo
        if [[ ! $REPLY =~ ^[Yy]$ ]]; then
            echo "Exiting..."
            exit 0
        fi
        SKIP_CREATE=true
    fi
fi

# Create database if not skipped
if [ "$SKIP_CREATE" != "true" ]; then
    echo ""
    echo "Creating database '$DB_NAME'..."
    createdb -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" "$DB_NAME" || {
        echo "❌ Failed to create database"
        exit 1
    }
    echo "✓ Database created"
fi

# Run migrations
echo ""
echo "Running migrations..."
MIGRATION_FILE="velocitypay-api/migrations/001_init.sql"

if [ ! -f "$MIGRATION_FILE" ]; then
    echo "❌ Migration file not found: $MIGRATION_FILE"
    exit 1
fi

psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -f "$MIGRATION_FILE" || {
    echo "❌ Failed to run migrations"
    exit 1
}

echo "✓ Migrations completed"

# Verify tables were created
echo ""
echo "Verifying database setup..."
TABLE_COUNT=$(psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -t -c "SELECT COUNT(*) FROM information_schema.tables WHERE table_schema = 'public' AND table_type = 'BASE TABLE';")

if [ "$TABLE_COUNT" -ge 5 ]; then
    echo "✓ Found $TABLE_COUNT tables"
else
    echo "⚠️  Warning: Expected at least 5 tables, found $TABLE_COUNT"
fi

# Display created tables
echo ""
echo "Created tables:"
psql -h "$DB_HOST" -p "$DB_PORT" -U "$DB_USER" -d "$DB_NAME" -c "\dt"

echo ""
echo "========================================="
echo "✅ Database setup complete!"
echo "========================================="
echo ""
echo "Database URL for .env:"
echo "  DATABASE_URL=postgresql://$DB_USER:password@$DB_HOST:$DB_PORT/$DB_NAME"
echo ""
echo "Next steps:"
echo "  1. Update velocitypay-api/.env with your database credentials"
echo "  2. Start the blockchain: cd velocity-chain && cargo run --release -- --dev --tmp"
echo "  3. Start the API: cd velocitypay-api && cargo run"
echo ""
