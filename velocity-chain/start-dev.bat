@echo off
REM Simple development node startup
REM This purges the chain first to avoid GenesisBuilder errors

echo VelocityPay - Simple Dev Node
echo ================================
echo.
echo This will:
echo 1. Purge any existing dev chain
echo 2. Start a fresh development node
echo.

REM Purge existing chain to avoid errors
target\release\velocity-node.exe purge-chain --dev -y 2>nul

echo Starting fresh development node...
echo RPC endpoint: http://localhost:9944
echo WebSocket: ws://localhost:9944
echo.
echo Press Ctrl+C to stop
echo.

target\release\velocity-node.exe --dev
