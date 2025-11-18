@echo off
REM Run VelocityPay blockchain in development mode
REM This creates a temporary chain with Alice as validator

echo Starting VelocityPay Development Chain...
echo.
echo This will run a single-node development blockchain.
echo Press Ctrl+C to stop the node.
echo.

target\release\velocity-node --dev --tmp --rpc-external --rpc-cors all
