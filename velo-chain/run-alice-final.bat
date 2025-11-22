@echo off
REM VeloPay Alice Validator Startup Script
REM Starts the Substrate blockchain node as Alice validator with persistent node key

echo ========================================
echo VeloPay - Alice Validator
echo ========================================
echo.

REM Check if node key exists
if not exist "node-keys\alice-node-key" (
    echo [ERROR] Alice's node key not found!
    echo.
    echo Please run setup-node-keys-v2.bat first to generate node keys.
    echo.
    pause
    exit /b 1
)

echo [OK] Alice's node key found
echo.

REM Create chain data directory if needed
if not exist "chain-data\alice" (
    echo [+] Creating chain-data\alice directory...
    mkdir chain-data\alice
)

echo ========================================
echo IMPORTANT - READ THIS!
echo ========================================
echo.
echo When the node starts, look for this line in the output:
echo.
echo   Local node identity is: 12D3Koo...
echo.
echo This is Alice's PEER ID - you'll need it to connect Bob!
echo.
echo The peer ID will be the SAME every time because we're
echo using a persistent node key file.
echo.
echo TIP: Keep this window open and start Bob in a NEW window
echo.
echo ========================================
echo Starting Alice Validator...
echo ========================================
echo.

REM Start the node (disable color/emoji output for CMD compatibility)
set RUST_LOG_STYLE=never
target\release\velo-node.exe ^
  --base-path .\chain-data\alice ^
  --chain local ^
  --alice ^
  --node-key-file node-keys\alice-node-key ^
  --port 30333 ^
  --rpc-port 9944 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --rpc-methods Unsafe ^
  --log-color never

echo.
echo ========================================
echo Alice validator has stopped
echo ========================================
pause
