@echo off
REM Run VeloPay blockchain as Bob validator
REM This connects to Alice to form a 2-validator network

echo Starting VeloPay as Bob (Validator)...
echo Data directory: .\chain-data\bob
echo RPC endpoint: http://localhost:9945
echo.
echo IMPORTANT: Make sure Alice is running first!
echo           Check Alice's peer ID in her startup logs.
echo.
echo If you see bootnode peer ID mismatch warnings:
echo   Update the --bootnodes line below with Alice's actual peer ID
echo   Format: /ip4/127.0.0.1/tcp/30333/p2p/[ALICE_PEER_ID]
echo.

REM Create chain-data directory if it doesn't exist
if not exist "chain-data\bob" mkdir "chain-data\bob"

REM Note: Update the peer ID in --bootnodes with Alice's actual ID from her logs
REM Current Alice peer ID: 12D3KooWLwxDB1ucs2QG2cQzjSziEALG84Q43uxPZV6EGikX8L3J
target\release\velo-node.exe ^
  --base-path .\chain-data\bob ^
  --chain local ^
  --bob ^
  --port 30334 ^
  --rpc-port 9945 ^
  --rpc-external ^
  --rpc-cors all ^
  --validator ^
  --rpc-methods Unsafe ^
  --bootnodes /ip4/127.0.0.1/tcp/30333/p2p/12D3KooWLwxDB1ucs2QG2cQzjSziEALG84Q43uxPZV6EGikX8L3J ^
  --unsafe-force-node-key-generation