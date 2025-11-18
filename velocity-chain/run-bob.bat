@echo off
REM Run VelocityPay blockchain as Bob validator
REM This connects to Alice to form a 2-validator network

echo Starting VelocityPay as Bob (Validator)...
echo Data directory: .\chain-data\bob
echo RPC endpoint: http://localhost:9945
echo.
echo Make sure Alice is running first!
echo.

target\release\velocity-node ^
  --base-path .\chain-data\bob ^
  --chain dev ^
  --bob ^
  --port 30334 ^
  --rpc-port 9945 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWEyoppNCUx8Yx66oV9fJnriXwCcXwDDUA2kj6vnc6iDEp
