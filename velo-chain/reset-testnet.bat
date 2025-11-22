@echo off
REM VeloPay Testnet Reset Script
REM Cleanly resets the local testnet while preserving node keys

echo ========================================
echo VeloPay Testnet Reset
echo ========================================
echo.

echo WARNING: This will DELETE all blockchain data!
echo.
echo What will be deleted:
echo   - chain-data\alice\       (Alice's blockchain database)
echo   - chain-data\bob\         (Bob's blockchain database)
echo   - alice-peer-id.txt       (saved peer ID - will re-prompt)
echo.
echo What will be PRESERVED:
echo   - node-keys\              (persistent node keys)
echo   - Your compiled binaries
echo.
echo This means your Peer IDs will remain the same after reset.
echo.

set /p CONFIRM="Are you sure you want to reset? (Y/N): "

if /i "%CONFIRM%" NEQ "Y" (
    echo.
    echo [CANCELLED] Reset cancelled - no changes made
    echo.
    pause
    exit /b 0
)

echo.
echo ========================================
echo Resetting Testnet...
echo ========================================
echo.

REM Check if any processes might be using the data
echo [!] Please make sure to STOP both Alice and Bob nodes
echo     ^(Press Ctrl+C in their terminal windows^)
echo.
pause

REM Delete Alice's chain data
if exist "chain-data\alice" (
    echo [1/3] Deleting Alice's chain data...
    rmdir /s /q chain-data\alice
    if %ERRORLEVEL% EQU 0 (
        echo [OK] Alice's data deleted
    ) else (
        echo [ERROR] Could not delete Alice's data ^(is the node still running?^)
    )
) else (
    echo [1/3] Alice's chain data not found ^(already clean^)
)

echo.

REM Delete Bob's chain data
if exist "chain-data\bob" (
    echo [2/3] Deleting Bob's chain data...
    rmdir /s /q chain-data\bob
    if %ERRORLEVEL% EQU 0 (
        echo [OK] Bob's data deleted
    ) else (
        echo [ERROR] Could not delete Bob's data ^(is the node still running?^)
    )
) else (
    echo [2/3] Bob's chain data not found ^(already clean^)
)

echo.

REM Delete saved peer ID
if exist "alice-peer-id.txt" (
    echo [3/3] Deleting saved peer ID...
    del alice-peer-id.txt
    if %ERRORLEVEL% EQU 0 (
        echo [OK] alice-peer-id.txt deleted ^(Bob will re-prompt^)
    ) else (
        echo [ERROR] Could not delete alice-peer-id.txt
    )
) else (
    echo [3/3] alice-peer-id.txt not found ^(already clean^)
)

echo.
echo ========================================
echo [SUCCESS] Testnet Reset Complete!
echo ========================================
echo.

REM Verify node keys are still there
if exist "node-keys\alice-node-key" (
    if exist "node-keys\bob-node-key" (
        echo [OK] Node keys preserved - Peer IDs will remain the same
    )
)

echo.
echo Next steps:
echo   1. Start Alice: run-alice-final.bat
echo   2. Start Bob:   run-bob-final.bat
echo.
echo Since alice-peer-id.txt was deleted, Bob will ask you
echo to enter Alice's Peer ID again.
echo.
pause
