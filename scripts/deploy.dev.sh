#!/bin/bash

source scripts/env/dev.sh

cd contracts

forge script script/Deploy.s.sol:DeployScript --rpc-url $RPC_URL --broadcast --private-key $PRIVATE_KEY
