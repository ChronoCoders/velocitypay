@echo off
REM VeloPay API - Windows Build Script
REM This script builds the API with SQLx offline mode enabled

echo Building VeloPay API with SQLx offline mode...
echo.

REM Set SQLx offline mode
set SQLX_OFFLINE=true

REM Build the project
cargo build --release

REM Check if build was successful
if %ERRORLEVEL% EQU 0 (
    echo.
    echo ========================================
    echo Build completed successfully!
    echo Executable: target\release\velopay-api.exe
    echo ========================================
) else (
    echo.
    echo ========================================
    echo Build failed with error code %ERRORLEVEL%
    echo ========================================
    exit /b %ERRORLEVEL%
)
