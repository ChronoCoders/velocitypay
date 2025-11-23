# VeloPay API - PowerShell Build Script
# This script builds the API with SQLx offline mode enabled

Write-Host "Building VeloPay API with SQLx offline mode..." -ForegroundColor Cyan
Write-Host ""

# Set SQLx offline mode
$env:SQLX_OFFLINE = "true"

# Build the project
cargo build --release

# Check if build was successful
if ($LASTEXITCODE -eq 0) {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Green
    Write-Host "Build completed successfully!" -ForegroundColor Green
    Write-Host "Executable: target\release\velopay-api.exe" -ForegroundColor Green
    Write-Host "========================================" -ForegroundColor Green
} else {
    Write-Host ""
    Write-Host "========================================" -ForegroundColor Red
    Write-Host "Build failed with error code $LASTEXITCODE" -ForegroundColor Red
    Write-Host "========================================" -ForegroundColor Red
    exit $LASTEXITCODE
}
