@echo off
REM Test VeloPay Chain via RPC
echo ========================================
echo VeloPay Chain RPC Testing
echo ========================================
echo.

echo 1. Chain Information:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_chain\"}" http://localhost:9944
echo.
echo.

echo 2. Chain Health:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_health\"}" http://localhost:9944
echo.
echo.

echo 3. Current Block Number:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"chain_getHeader\"}" http://localhost:9944
echo.
echo.

echo 4. Network Peers:
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_peers\"}" http://localhost:9944
echo.
echo.

echo 5. Account Balance (Alice):
curl -s -H "Content-Type: application/json" -d "{\"id\":1, \"jsonrpc\":\"2.0\", \"method\": \"system_accountNextIndex\", \"params\": [\"5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY\"]}" http://localhost:9944
echo.
echo.

echo ========================================
echo RPC Endpoint: http://localhost:9944
echo WebSocket: ws://localhost:9944
echo ========================================
