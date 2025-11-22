@echo off
REM VeloPay Node Key Generator v2
REM Generates persistent Ed25519 node keys for Substrate validators

echo ========================================
echo VeloPay Node Key Generator v2
echo ========================================
echo.

REM Create node-keys directory if it doesn't exist
if not exist "node-keys" (
    echo [1/3] Creating node-keys directory...
    mkdir node-keys
    echo [OK] Directory created
) else (
    echo [1/3] node-keys directory already exists
)
echo.

REM Generate Alice's node key
echo [2/3] Generating Alice's node key...
powershell -Command "$rng = New-Object System.Security.Cryptography.RNGCryptoServiceProvider; $bytes = New-Object byte[] 32; $rng.GetBytes($bytes); $hex = [System.BitConverter]::ToString($bytes).Replace('-','').ToLower(); Set-Content -Path 'node-keys\alice-node-key' -Value $hex -NoNewline"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to generate Alice's node key!
    pause
    exit /b 1
)

echo [OK] Alice's node key generated: node-keys\alice-node-key
echo.

REM Generate Bob's node key
echo [3/3] Generating Bob's node key...
powershell -Command "$rng = New-Object System.Security.Cryptography.RNGCryptoServiceProvider; $bytes = New-Object byte[] 32; $rng.GetBytes($bytes); $hex = [System.BitConverter]::ToString($bytes).Replace('-','').ToLower(); Set-Content -Path 'node-keys\bob-node-key' -Value $hex -NoNewline"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to generate Bob's node key!
    pause
    exit /b 1
)

echo [OK] Bob's node key generated: node-keys\bob-node-key
echo.

echo ========================================
echo [SUCCESS] Node keys generated!
echo ========================================
echo.
echo Files created:
echo   - node-keys\alice-node-key (64 hex characters)
echo   - node-keys\bob-node-key (64 hex characters)
echo.
echo ========================================
echo SECURITY WARNING
echo ========================================
echo.
echo These keys determine your validator's identity!
echo.
echo [!] Keep these files SECURE and BACKED UP
echo [!] Same key = Same Peer ID (consistent identity)
echo [!] Lost keys = Cannot recover same identity
echo [!] DO NOT commit these to version control
echo [!] DO NOT share these publicly
echo.
echo Add to .gitignore:
echo   node-keys/
echo   alice-peer-id.txt
echo   chain-data/
echo.
echo ========================================
echo Next Steps:
echo ========================================
echo.
echo 1. Start Alice validator:
echo    run-alice-final.bat
echo.
echo 2. Note Alice's "Local node identity is:" peer ID
echo.
echo 3. Start Bob validator:
echo    run-bob-final.bat
echo.
pause
