@echo off
REM VeloPay Node Key Inspector
REM Displays information about generated node keys

echo ========================================
echo VeloPay Node Key Inspector
echo ========================================
echo.

REM Check if node-keys directory exists
if not exist "node-keys" (
    echo [ERROR] node-keys directory not found!
    echo.
    echo Please run setup-node-keys-v2.bat first to generate node keys.
    echo.
    pause
    exit /b 1
)

echo [OK] node-keys directory found
echo.

REM Inspect Alice's key
echo ========================================
echo Alice's Node Key
echo ========================================
echo.
if exist "node-keys\alice-node-key" (
    echo File: node-keys\alice-node-key
    echo.
    echo Content (hex):
    type node-keys\alice-node-key
    echo.
    echo.

    REM Get file size and modified date
    for %%A in (node-keys\alice-node-key) do (
        echo Size: %%~zA bytes ^(should be 64 for hex representation^)
        echo Modified: %%~tA
    )
    echo.
) else (
    echo [NOT FOUND] node-keys\alice-node-key
    echo.
)

echo ========================================
echo Bob's Node Key
echo ========================================
echo.
if exist "node-keys\bob-node-key" (
    echo File: node-keys\bob-node-key
    echo.
    echo Content (hex):
    type node-keys\bob-node-key
    echo.
    echo.

    REM Get file size and modified date
    for %%A in (node-keys\bob-node-key) do (
        echo Size: %%~zA bytes ^(should be 64 for hex representation^)
        echo Modified: %%~tA
    )
    echo.
) else (
    echo [NOT FOUND] node-keys\bob-node-key
    echo.
)

echo ========================================
echo Peer ID Information
echo ========================================
echo.
echo Note: Peer IDs are derived from node keys and can
echo only be seen by starting the actual node.
echo.
echo To see Alice's Peer ID:
echo   run-alice-final.bat
echo   Look for: "Local node identity is: 12D3Koo..."
echo.
echo To see Bob's Peer ID:
echo   run-bob-final.bat
echo   Look for: "Local node identity is: 12D3Koo..."
echo.

REM Check if saved peer ID exists
if exist "alice-peer-id.txt" (
    echo ========================================
    echo Saved Alice Peer ID
    echo ========================================
    echo.
    echo File: alice-peer-id.txt
    echo.
    echo Content:
    type alice-peer-id.txt
    echo.
    echo.
)

echo ========================================
echo Security Reminder
echo ========================================
echo.
echo [!] Keep these node keys SECURE and BACKED UP
echo [!] DO NOT share publicly or commit to git
echo [!] Lost keys = Cannot recover same identity
echo.
pause
