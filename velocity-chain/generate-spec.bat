@echo off
REM Generate custom chain specification
REM This creates a chain spec that doesn't require GenesisBuilder

echo Generating VelocityPay Chain Specification...
echo.

REM Generate local testnet spec (works better than dev)
target\release\velocity-node.exe build-spec --chain local --disable-default-bootnode > chain-spec.json

if errorlevel 1 (
    echo [ERROR] Failed to generate chain spec
    pause
    exit /b 1
)

echo [OK] Generated chain-spec.json
echo.

REM Convert to raw format (required for running)
echo Converting to raw format...
target\release\velocity-node.exe build-spec --chain chain-spec.json --raw > chain-spec-raw.json

if errorlevel 1 (
    echo [ERROR] Failed to convert to raw format
    pause
    exit /b 1
)

echo [OK] Generated chain-spec-raw.json
echo.
echo Chain specification generated successfully!
echo.
echo Next steps:
echo   1. Run with: start-local.bat
echo   2. Or use Alice: run-local-alice.bat
echo.
pause
