#!/bin/bash

echo "== [ Starts $0 ] =="

SOLANA_DEVNET="https://api.devnet.solana.com"

echo "== [ Exporting Anchor env variables ] ==================================="
export ANCHOR_PROVIDER_URL=$SOLANA_DEVNET
export ANCHOR_WALLET=~/.config/solana/id.json

# CONTROLLER_PROGRAM_NAME="Controller"
# DEPOSITORY_PROGRAM_NAME="Depository"
# CONTROLLER_PROGRAM_ID="2PCPrsHdeZq6CsHyqnu3NVMcWtJGjZE8mWKpF6ipTDT4"
# DEPOSITORY_PROGRAM_ID="FJjdNskuQnHvRozc7zTQ5GPTyo5HZokgubvXYdR9tKHi"

# CONTROLLER_DEVNET_DEPLOYMENT_KEY_PATH=$PWD/target/deploy/controller-keypair.json
# DEPOSITORY_DEVNET_DEPLOYMENT_KEY_PATH=$PWD/target/deploy/depository-keypair.json
# CONTROLLER_SHARED_LIB_OUT=$PWD/target/deploy/controller.so
# DEPOSITORY_SHARED_LIB_OUT=$PWD/target/deploy/depository.so

echo "== [ Set local Solana env to Devnet ] ==================================="
solana config set --url $SOLANA_DEVNET

echo "== [ Building... ] ======================================================"
anchor build

echo "== [ Deploy... ] ========================================================"
anchor --provider.cluster $ANCHOR_PROVIDER_URL deploy

# solana account $CONTROLLER_PROGRAM_ID &> /dev/null
# if [ $? -eq 0 ]; then
#     echo "== [ Upgrading Controller... ] ==================================="
#     anchor upgrade \
#         --provider.cluster $SOLANA_DEVNET \
#         --program-id $CONTROLLER_PROGRAM_ID \
#         $CONTROLLER_SHARED_LIB_OUT
# else
#     echo "== [ Deploying Controller... ] ==================================="
#     anchor deploy \
#         --provider.cluster $SOLANA_DEVNET \
#         --provider.wallet $CONTROLLER_DEVNET_DEPLOYMENT_KEY_PATH \
#         --program-name $CONTROLLER_PROGRAM_NAME
# fi

# solana account $DEPOSITORY_PROGRAM_ID &> /dev/null
# if [ $? -eq 0 ]; then
#     echo "== [ Upgrading Depository... ] ==================================="
#     anchor upgrade \
#         --provider.cluster $SOLANA_DEVNET \
#         --program-id $DEPOSITORY_PROGRAM_ID \
#         $DEPOSITORY_SHARED_LIB_OUT
# else
#     echo "== [ Deploying Controller... ] ==================================="
#     anchor deploy \
#         --provider.cluster $SOLANA_DEVNET \
#         --provider.wallet $DEPOSITORY_DEVNET_DEPLOYMENT_KEY_PATH \
#         --program-name $DEPOSITORY_PROGRAM_NAME
# fi

echo "== [ Run test suit against Devnet ] ====================================="
TEST_SUIT=tests/test_*.ts
TEST_TIMEOUT=100000

yarn ts-mocha -p ./tsconfig.json -t $TEST_TIMEOUT $TEST_SUIT

echo "== [ $0 completed ] =="