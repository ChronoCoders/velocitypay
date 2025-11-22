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

REM Generate Alice's node key (using %RANDOM% - sufficient for dev/test)
echo [2/3] Generating Alice's node key...

setlocal enabledelayedexpansion
set "hex="
for /L %%i in (1,1,16) do (
    set /a "r1=%RANDOM% %% 256"
    set /a "r2=%RANDOM% %% 256"
    call :tohex !r1! h1
    call :tohex !r2! h2
    set "hex=!hex!!h1!!h2!"
)

REM Write to file without newline
<nul set /p "=!hex!" > node-keys\alice-node-key
endlocal

echo [OK] Alice's node key generated: node-keys\alice-node-key
echo.

REM Generate Bob's node key
echo [3/3] Generating Bob's node key...

setlocal enabledelayedexpansion
set "hex="
for /L %%i in (1,1,16) do (
    set /a "r1=%RANDOM% %% 256"
    set /a "r2=%RANDOM% %% 256"
    call :tohex !r1! h1
    call :tohex !r2! h2
    set "hex=!hex!!h1!!h2!"
)

<nul set /p "=!hex!" > node-keys\bob-node-key
endlocal

goto after_tohex

:tohex
set "n=%~1"
set "h=0123456789abcdef"
set /a "d1=%n% / 16"
set /a "d2=%n% %% 16"
for /f %%a in ("!d1!") do set "c1=!h:~%%a,1!"
for /f %%a in ("!d2!") do set "c2=!h:~%%a,1!"
set "%~2=!c1!!c2!"
exit /b

:after_tohex

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
