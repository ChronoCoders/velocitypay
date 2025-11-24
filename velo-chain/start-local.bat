@echo off
REM Start local testnet node (avoids GenesisBuilder error)

echo VeloPay - Local Testnet Node
echo ====================================
echo.

REM Check if chain spec exists
if not exist "chain-spec-raw.json" (
    echo Chain specification not found.
    echo Generating it now...
    echo.
    call generate-spec.bat
    if errorlevel 1 (
        echo [ERROR] Failed to generate chain spec
        pause
        exit /b 1
    )
)

echo Starting local testnet node with Alice...
echo RPC endpoint: http://localhost:9944
echo WebSocket: ws://localhost:9944
echo.
echo IMPORTANT: Note the peer ID shown below for multi-node setup.
echo.
echo Press Ctrl+C to stop
echo.

REM Purge old chain
target\release\velo-node.exe purge-chain --chain chain-spec-raw.json --base-path .\chain-data\alice -y 2>nul

REM Create directory
if not exist "chain-data\alice" mkdir "chain-data\alice"

REM Start node with no-mdns to reduce bootnode warnings in single-node mode
target\release\velo-node.exe ^
  --base-path .\chain-data\alice ^
  --chain chain-spec-raw.json ^
  --alice ^
  --port 30333 ^
  --rpc-port 9944 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --rpc-methods Unsafe ^
  --no-mdns ^
  --unsafe-force-node-key-generation