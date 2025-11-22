@echo off
REM VeloPay Network Status Checker
REM Displays the status of both Alice and Bob validator nodes

echo ========================================
echo VeloPay Network Status
echo ========================================
echo.
echo Checked at: %date% %time%
echo.

REM Check if curl is available
where curl >nul 2>nul
if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] curl is not installed or not in PATH
    echo.
    echo Please install curl to use this script:
    echo   https://curl.se/windows/
    echo.
    echo Or check status manually at:
    echo   Alice: http://localhost:9944
    echo   Bob:   http://localhost:9945
    echo.
    pause
    exit /b 1
)

echo ========================================
echo Alice Validator (Port 9944)
echo ========================================
echo.

REM Check Alice's health
curl -s -X POST -H "Content-Type: application/json" ^
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
  http://127.0.0.1:9944 >nul 2>nul

if %ERRORLEVEL% NEQ 0 (
    echo Status: [OFFLINE] Cannot connect to Alice
    echo.
) else (
    echo Status: [ONLINE]
    echo.

    REM Get chain name
    echo Chain:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_chain\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9944
    echo.
    echo.

    REM Get health
    echo Health:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9944
    echo.
    echo.

    REM Get current block
    echo Current Block:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9944
    echo.
    echo.

    REM Get peer count
    echo Peers:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_peers\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9944 | find /c "peerId"
)

echo.
echo ========================================
echo Bob Validator (Port 9945)
echo ========================================
echo.

REM Check Bob's health
curl -s -X POST -H "Content-Type: application/json" ^
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
  http://127.0.0.1:9945 >nul 2>nul

if %ERRORLEVEL% NEQ 0 (
    echo Status: [OFFLINE] Cannot connect to Bob
    echo.
) else (
    echo Status: [ONLINE]
    echo.

    REM Get chain name
    echo Chain:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_chain\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9945
    echo.
    echo.

    REM Get health
    echo Health:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9945
    echo.
    echo.

    REM Get current block
    echo Current Block:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"chain_getHeader\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9945
    echo.
    echo.

    REM Get peer count
    echo Peers:
    curl -s -X POST -H "Content-Type: application/json" ^
      -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_peers\",\"params\":[],\"id\":1}" ^
      http://127.0.0.1:9945 | find /c "peerId"
)

echo.
echo ========================================
echo Quick Status Summary
echo ========================================
echo.

REM Quick summary check
set ALICE_UP=0
set BOB_UP=0

curl -s -X POST -H "Content-Type: application/json" ^
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
  http://127.0.0.1:9944 >nul 2>nul
if %ERRORLEVEL% EQU 0 set ALICE_UP=1

curl -s -X POST -H "Content-Type: application/json" ^
  -d "{\"jsonrpc\":\"2.0\",\"method\":\"system_health\",\"params\":[],\"id\":1}" ^
  http://127.0.0.1:9945 >nul 2>nul
if %ERRORLEVEL% EQU 0 set BOB_UP=1

if %ALICE_UP% EQU 1 (
    echo [+] Alice is RUNNING
) else (
    echo [-] Alice is STOPPED
)

if %BOB_UP% EQU 1 (
    echo [+] Bob is RUNNING
) else (
    echo [-] Bob is STOPPED
)

echo.

if %ALICE_UP% EQU 1 (
    if %BOB_UP% EQU 1 (
        echo Network Status: FULLY OPERATIONAL
    ) else (
        echo Network Status: PARTIAL ^(Bob is down^)
    )
) else (
    if %BOB_UP% EQU 1 (
        echo Network Status: PARTIAL ^(Alice is down^)
    ) else (
        echo Network Status: OFFLINE ^(both validators down^)
    )
)

echo.
echo ========================================
echo Note
echo ========================================
echo.
echo If both nodes are online but show 0 peers, they
echo may not be connected to each other. Check:
echo   - Alice's peer ID is correct in alice-peer-id.txt
echo   - Bob's bootnode configuration
echo   - Firewall settings
echo.
pause
