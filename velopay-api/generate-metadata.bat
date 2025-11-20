@echo off
REM VeloPay API - Metadata Generation Script (Windows)
REM This script generates the metadata.scale file from a running velo-chain node

echo ================================
echo VeloPay Metadata Generator
echo ================================
echo.

REM Check if subxt-cli is installed
where subxt >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] subxt-cli is not installed!
    echo.
    echo Please install it first with:
    echo   cargo install subxt-cli
    echo.
    echo This may take 5-10 minutes to compile...
    pause
    exit /b 1
)

echo [1/3] Checking if velo-chain node is running...
echo.

REM Try to connect to the node
curl -s -X POST -H "Content-Type: application/json" -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" http://127.0.0.1:9933 >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Cannot connect to velo-chain node at ws://127.0.0.1:9944
    echo.
    echo Please start the node first:
    echo   cd ..\velo-chain
    echo   cargo run --release -- --dev --tmp
    echo.
    echo Or if using the quick-start script:
    echo   cd ..\velo-chain
    echo   quick-start.bat
    echo.
    pause
    exit /b 1
)

echo [OK] Node is running
echo.

echo [2/3] Generating metadata.scale from running node...
echo This may take a few seconds...
echo.

subxt metadata -f bytes --url ws://127.0.0.1:9944 > metadata.scale

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to generate metadata!
    echo.
    echo Make sure:
    echo   1. velo-chain node is running on ws://127.0.0.1:9944
    echo   2. The node has fully started (check for "Idle" in the logs)
    echo   3. You have network access to localhost:9944
    echo.
    pause
    exit /b 1
)

echo [3/3] Verifying metadata.scale was created...
echo.

if not exist metadata.scale (
    echo [ERROR] metadata.scale file was not created!
    pause
    exit /b 1
)

for %%A in (metadata.scale) do set size=%%~zA
if %size% LSS 1000 (
    echo [ERROR] metadata.scale file is too small (possibly corrupt^)
    pause
    exit /b 1
)

echo ================================
echo [SUCCESS] Metadata generated!
echo ================================
echo.
echo File: metadata.scale
echo Size: %size% bytes
echo.
echo You can now uncomment the subxt macro in src/chain/client.rs
echo and rebuild the API server.
echo.
pause
