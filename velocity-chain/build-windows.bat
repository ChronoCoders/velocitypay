@echo off
REM VelocityPay Windows Build Script with Network Retry
REM Handles common Windows build issues automatically

echo ========================================
echo VelocityPay Windows Build Script
echo ========================================
echo.

REM Set required environment variables
set OPENSSL_VENDORED=1

echo [1/6] Setting environment variables...
echo   OPENSSL_VENDORED = %OPENSSL_VENDORED%
echo.

REM Check if WASM target is installed
echo [2/6] Checking Rust targets...
rustup target list | findstr /C:"wasm32-unknown-unknown (installed)" >nul
if errorlevel 1 (
    echo   Installing wasm32-unknown-unknown target...
    rustup target add wasm32-unknown-unknown
) else (
    echo   ✓ wasm32-unknown-unknown already installed
)

rustup component list | findstr /C:"rust-src (installed)" >nul
if errorlevel 1 (
    echo   Installing rust-src component...
    rustup component add rust-src
) else (
    echo   ✓ rust-src already installed
)
echo.

REM Pre-fetch dependencies (helps with network issues)
echo [3/6] Pre-fetching dependencies (may take a while)...
echo   This downloads all crates before building
echo.
cargo fetch
if errorlevel 1 (
    echo.
    echo [WARNING] Dependency fetch had issues. Retrying...
    timeout /t 5 /nobreak >nul
    cargo fetch
    if errorlevel 1 (
        echo.
        echo [ERROR] Failed to fetch dependencies after retry.
        echo.
        echo Possible solutions:
        echo 1. Check your internet connection
        echo 2. Try again later (crates.io might be slow)
        echo 3. Check firewall/antivirus settings
        echo 4. See WINDOWS-BUILD-GUIDE.md for more help
        echo.
        pause
        exit /b 1
    )
)
echo.

REM Clean previous build artifacts (optional)
echo [4/6] Checking for previous builds...
if exist "target\release\velocity-node.exe" (
    echo   Found previous build. Cleaning is not required for incremental builds.
    choice /C YN /M "Do you want to clean and rebuild from scratch"
    if errorlevel 2 (
        echo   Keeping previous build for faster compilation...
    ) else (
        echo   Cleaning previous build...
        cargo clean
    )
) else (
    echo   No previous build found. This will be a full build.
)
echo.

REM Build the project
echo [5/6] Building VelocityPay (this will take 15-45 minutes on first build)...
echo   Press Ctrl+C to cancel
echo.

cargo build --release
if errorlevel 1 (
    echo.
    echo ========================================
    echo [ERROR] Build failed!
    echo ========================================
    echo.
    echo Check the error messages above.
    echo See WINDOWS-BUILD-GUIDE.md for troubleshooting steps.
    echo.
    echo Common issues:
    echo - Network timeout: Run this script again
    echo - Out of memory: Close other programs and retry
    echo - Missing tools: Install Visual Studio Build Tools
    echo.
    pause
    exit /b 1
)

echo.
echo [6/6] Verifying build...
if exist "target\release\velocity-node.exe" (
    echo   ✓ velocity-node.exe created successfully

    REM Get file size
    for %%F in ("target\release\velocity-node.exe") do set SIZE=%%~zF
    set /a SIZE_MB=%SIZE% / 1048576
    echo   File size: %SIZE_MB% MB
) else (
    echo   [ERROR] Binary not found after build!
    pause
    exit /b 1
)

echo.
echo ========================================
echo [SUCCESS] Build completed successfully!
echo ========================================
echo.
echo Binary location: target\release\velocity-node.exe
echo.
echo Next steps:
echo   1. Run health check: check-node.bat
echo   2. Start dev chain: run-dev.bat
echo   3. Or start validator: run-alice.bat
echo.
echo See TESTING.md for more information.
echo.
pause
