@echo off
REM Quick health check for VelocityPay node
echo ========================================
echo VelocityPay Node Health Check
echo ========================================
echo.

REM Check if binary exists
if not exist "target\release\velocity-node.exe" (
    echo [ERROR] velocity-node.exe not found!
    echo Please run: cargo build --release
    exit /b 1
)

echo [OK] velocity-node binary found
echo.

REM Start node in background for testing
echo Starting node for health check...
start /B target\release\velocity-node --dev --tmp > nul 2>&1

REM Wait for node to start
echo Waiting for node to start (10 seconds)...
timeout /t 10 /nobreak > nul

REM Check if node is responding
echo.
echo Checking RPC endpoint...
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" http://localhost:9944 > health.tmp 2>nul

if errorlevel 1 (
    echo [ERROR] Node is not responding on port 9944
    echo Please check if the node started correctly
    taskkill /F /IM velocity-node.exe > nul 2>&1
    del health.tmp > nul 2>&1
    exit /b 1
)

echo [OK] Node is responding
echo.

echo Chain Info:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_chain\"}" http://localhost:9944
echo.

echo Node Version:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_version\"}" http://localhost:9944
echo.

echo.
echo ========================================
echo [SUCCESS] Node is healthy and running!
echo ========================================
echo.
echo Stopping test node...
taskkill /F /IM velocity-node.exe > nul 2>&1

del health.tmp > nul 2>&1

echo.
echo You can now start the node with:
echo   - run-dev.bat (development mode)
echo   - run-alice.bat (persistent single node)
echo.
echo Or connect to it at:
echo   https://polkadot.js.org/apps/?rpc=ws://127.0.0.1:9944
