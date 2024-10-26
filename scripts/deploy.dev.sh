#!/bin/bash

RPC_URL="http://localhost:8545"
PRIVATE_KEY="0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80"

cd contracts

forge script script/Deploy.s.sol:DeployScript --rpc-url $RPC_URL --broadcast --private-key $PRIVATE_KEY

ZKTLSGATEWAY_ADDRESS=$(cat broadcast/Deploy.s.sol/31337/run-latest.json | jq -r ".transactions[0].contractAddress")
echo "ZkTLSGateway deployed at: $ZKTLSGATEWAY_ADDRESS"

cast send $ZKTLSGATEWAY_ADDRESS "requestTLSCall(string,bytes[])" \
    "https://example.com" \
    "[0x00,0x01,0x02]" \
    --rpc-url $RPC_URL --private-key $PRIVATE_KEY

rm -rf broadcast/ cache/