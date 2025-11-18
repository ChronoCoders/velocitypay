@echo off
REM Run VelocityPay blockchain as Alice validator
REM This creates a persistent chain in the data directory

echo Starting VelocityPay as Alice (Validator)...
echo Data directory: .\chain-data\alice
echo RPC endpoint: http://localhost:9944
echo.

REM Create chain-data directory if it doesn't exist
if not exist "chain-data\alice" mkdir "chain-data\alice"

target\release\velocity-node.exe ^
  --base-path .\chain-data\alice ^
  --chain dev ^
  --alice ^
  --port 30333 ^
  --rpc-port 9944 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --unsafe-force-node-key-generation
