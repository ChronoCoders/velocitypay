@echo off
REM Quickest way to start VelocityPay - uses local chain instead of dev

echo VelocityPay - Quick Start
echo ============================
echo.
echo Starting local chain with Alice as validator...
echo This avoids the GenesisBuilder error.
echo.
echo RPC endpoint: http://localhost:9944
echo WebSocket: ws://localhost:9944
echo.
echo Press Ctrl+C to stop
echo.

REM Use --tmp for temporary storage, local chain type, Alice validator
target\release\velocity-node.exe ^
  --chain local ^
  --alice ^
  --tmp ^
  --rpc-port 9944 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --rpc-methods Unsafe
