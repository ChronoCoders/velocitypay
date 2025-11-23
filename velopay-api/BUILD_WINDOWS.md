# Building VeloPay API on Windows

This guide explains how to build the VeloPay API on Windows.

## Prerequisites

- Rust toolchain installed (https://rustup.rs/)
- Git for Windows
- PostgreSQL (optional - only needed for runtime, not compilation)

## Quick Build

We provide two build scripts that handle the SQLx offline mode automatically:

### Option 1: Using Command Prompt (cmd)

```cmd
build.bat
```

### Option 2: Using PowerShell

```powershell
.\build.ps1
```

## Manual Build

If you prefer to build manually:

### Command Prompt (cmd)

```cmd
set SQLX_OFFLINE=true
cargo build --release
```

**Important:** Run these as separate commands, not combined with `&&`.

### PowerShell

```powershell
$env:SQLX_OFFLINE = "true"
cargo build --release
```

## Why SQLX_OFFLINE is Required

The VeloPay API uses SQLx with compile-time query verification. This means SQLx normally tries to connect to a PostgreSQL database during compilation to verify all SQL queries are correct.

By setting `SQLX_OFFLINE=true`, SQLx uses pre-generated query metadata from the `.sqlx/` directory instead of connecting to a database. This allows you to compile the project without having a database running.

## Troubleshooting

### Error: "relation 'users' does not exist"

This error means SQLx is trying to connect to a database during compilation. Make sure `SQLX_OFFLINE=true` is set **before** running cargo build.

**Solution:** Use one of the provided build scripts (`build.bat` or `build.ps1`).

### Missing .sqlx Directory

If you get errors about missing query metadata:

```cmd
git pull origin claude/review-analyze-code-01QYPf5JoerE1diE6yfi1C8b
```

This ensures you have the latest `.sqlx/` directory with all query metadata.

### Updating Query Metadata

If you modify SQL queries and need to regenerate the `.sqlx/` metadata:

1. Set up a PostgreSQL database with the schema from `migrations/`
2. Set the DATABASE_URL environment variable
3. Run: `cargo sqlx prepare`

## Build Output

After a successful build, you'll find:

- **Executable:** `target\release\velopay-api.exe`
- **Size:** ~50-100 MB (release build)

## Next Steps

After building, see the main [README.md](../README.md) for instructions on:
- Setting up the database
- Configuring environment variables
- Running the API server
