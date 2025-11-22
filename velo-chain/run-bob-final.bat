@echo off
setlocal enabledelayedexpansion
REM VeloPay Bob Validator Startup Script
REM Starts the Substrate blockchain node as Bob validator and connects to Alice

echo ========================================
echo VeloPay - Bob Validator
echo ========================================
echo.

REM Check if node key exists
if not exist "node-keys\bob-node-key" (
    echo [ERROR] Bob's node key not found!
    echo.
    echo Please run setup-node-keys-v2.bat first to generate node keys.
    echo.
    pause
    exit /b 1
)

echo [OK] Bob's node key found
echo.

REM Try to read Alice's peer ID from file
set ALICE_PEER_ID=
if exist "alice-peer-id.txt" (
    set /p ALICE_PEER_ID=<alice-peer-id.txt
)

REM If file doesn't exist or is empty, prompt user
if "!ALICE_PEER_ID!"=="" (
    echo ========================================
    echo Alice Peer ID Required
    echo ========================================
    echo.
    echo Bob needs to connect to Alice bootnode.
    echo.
    echo Look at Alice terminal window for a line like:
    echo   Local node identity is: 12D3Koo...
    echo.
    echo Copy the peer ID and paste it here.
    echo.
    set /p ALICE_PEER_ID=Enter Alice Peer ID:

    REM Save it for next time
    echo !ALICE_PEER_ID!>alice-peer-id.txt
    echo.
    echo [OK] Peer ID saved to alice-peer-id.txt for future use
    echo.
) else (
    echo [OK] Using saved Alice peer ID from alice-peer-id.txt
    echo.
)

REM Validate peer ID format
echo !ALICE_PEER_ID! | findstr /R /C:"^12D3Koo" >nul
if %ERRORLEVEL% NEQ 0 (
    echo [WARNING] Peer ID does not look valid
    echo Continuing anyway, but connection may fail...
    echo.
)

REM Create chain data directory if needed
if not exist "chain-data\bob" (
    echo [+] Creating chain-data\bob directory...
    mkdir chain-data\bob
)

echo ========================================
echo Connection Details
echo ========================================
echo.
echo Bootnode: /ip4/127.0.0.1/tcp/30333/p2p/!ALICE_PEER_ID!
echo.
echo ========================================
echo IMPORTANT REMINDER
echo ========================================
echo.
echo Alice MUST be running before starting Bob!
echo.
echo If Bob cannot connect:
echo 1. Check Alice is running
echo 2. Verify the peer ID is correct
echo 3. Delete alice-peer-id.txt and re-enter
echo.
echo ========================================
echo Starting Bob Validator...
echo ========================================
echo.

REM Start the node (disable color/emoji output for CMD compatibility)
set RUST_LOG_STYLE=never
target\release\velo-node.exe ^
  --base-path .\chain-data\bob ^
  --chain local ^
  --bob ^
  --node-key-file node-keys\bob-node-key ^
  --port 30334 ^
  --rpc-port 9945 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --rpc-methods Unsafe ^
  --log-color never ^
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/!ALICE_PEER_ID!

echo.
echo ========================================
echo Bob validator has stopped
echo ========================================
pause
endlocal
