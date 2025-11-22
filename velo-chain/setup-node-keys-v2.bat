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

REM Generate Alice's node key using VBScript (pure Windows, no PowerShell needed)
echo [2/3] Generating Alice's node key...

REM Create temporary VBScript for cryptographic random generation
echo Set objFSO = CreateObject("Scripting.FileSystemObject") > %TEMP%\genkey.vbs
echo Set objRNG = CreateObject("System.Security.Cryptography.RNGCryptoServiceProvider") >> %TEMP%\genkey.vbs
echo arrBytes = Array() >> %TEMP%\genkey.vbs
echo ReDim arrBytes(31) >> %TEMP%\genkey.vbs
echo objRNG.GetBytes(arrBytes) >> %TEMP%\genkey.vbs
echo strHex = "" >> %TEMP%\genkey.vbs
echo For i = 0 To 31 >> %TEMP%\genkey.vbs
echo   strHex = strHex ^& Right("0" ^& Hex(arrBytes(i)), 2) >> %TEMP%\genkey.vbs
echo Next >> %TEMP%\genkey.vbs
echo Set objFile = objFSO.CreateTextFile(WScript.Arguments(0), True) >> %TEMP%\genkey.vbs
echo objFile.Write LCase(strHex) >> %TEMP%\genkey.vbs
echo objFile.Close >> %TEMP%\genkey.vbs

cscript //nologo %TEMP%\genkey.vbs "%CD%\node-keys\alice-node-key"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to generate Alice's node key!
    del %TEMP%\genkey.vbs 2>nul
    pause
    exit /b 1
)

echo [OK] Alice's node key generated: node-keys\alice-node-key
echo.

REM Generate Bob's node key
echo [3/3] Generating Bob's node key...

cscript //nologo %TEMP%\genkey.vbs "%CD%\node-keys\bob-node-key"

if %ERRORLEVEL% NEQ 0 (
    echo [ERROR] Failed to generate Bob's node key!
    del %TEMP%\genkey.vbs 2>nul
    pause
    exit /b 1
)

REM Clean up temp script
del %TEMP%\genkey.vbs 2>nul

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
