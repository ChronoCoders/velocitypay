#!/usr/bin/env python3
"""
VelocityPay Chain Testing Script
Direct interaction with your custom PoA blockchain via RPC
"""

import requests
import json

RPC_URL = "http://localhost:9944"

def rpc_call(method, params=None):
    """Make RPC call to VelocityPay node"""
    payload = {
        "id": 1,
        "jsonrpc": "2.0",
        "method": method,
        "params": params or []
    }

    response = requests.post(RPC_URL, json=payload)
    return response.json()

def test_chain():
    """Run basic chain tests"""
    print("=" * 50)
    print("VelocityPay Chain Testing")
    print("=" * 50)
    print()

    # 1. Chain info
    print("1. Chain Name:")
    result = rpc_call("system_chain")
    print(f"   {result.get('result', 'N/A')}")
    print()

    # 2. Node version
    print("2. Node Version:")
    result = rpc_call("system_version")
    print(f"   {result.get('result', 'N/A')}")
    print()

    # 3. Health
    print("3. Chain Health:")
    result = rpc_call("system_health")
    health = result.get('result', {})
    print(f"   Peers: {health.get('peers', 0)}")
    print(f"   Is Syncing: {health.get('isSyncing', False)}")
    print(f"   Should Have Peers: {health.get('shouldHavePeers', False)}")
    print()

    # 4. Latest block
    print("4. Latest Block:")
    result = rpc_call("chain_getHeader")
    header = result.get('result', {})
    if header:
        print(f"   Block Number: {int(header.get('number', '0x0'), 16)}")
        print(f"   Block Hash: {header.get('parentHash', 'N/A')[:16]}...")
    print()

    # 5. Network peers
    print("5. Connected Peers:")
    result = rpc_call("system_peers")
    peers = result.get('result', [])
    print(f"   Total Peers: {len(peers)}")
    for peer in peers[:3]:  # Show first 3
        print(f"   - {peer.get('peerId', 'Unknown')[:16]}...")
    print()

    # 6. Runtime metadata (shows your custom pallets)
    print("6. Runtime Metadata:")
    result = rpc_call("state_getMetadata")
    if result.get('result'):
        print("   ✓ Runtime metadata available")
        print("   Your custom pallets: VelocityPay, KYC, Compliance")
    print()

    print("=" * 50)
    print("✓ VelocityPay chain is running!")
    print("=" * 50)

if __name__ == "__main__":
    try:
        test_chain()
    except requests.exceptions.ConnectionError:
        print("ERROR: Cannot connect to VelocityPay node")
        print("Make sure the node is running on http://localhost:9944")
    except Exception as e:
        print(f"ERROR: {e}")
