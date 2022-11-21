#!/bin/bash
# Run this after deploying and getting the code ID
# User should pass in the code ID to the contract like:
# ./instantiate-local.sh 2
if [ -z "$1" ]
then
    echo "Must provide code ID (Example ./instantiate-local.sh 19 tp...)"
    exit 1
else
    CODE_ID=$1
fi

#INIT='{"allowed": ["tp1gvc0l4upc88arx673tmg7u3g7zsssnyyle5ph5"]}'
INIT='{"allowed": []}'
provenanced tx wasm instantiate "$CODE_ID" "$INIT" --label "authzdemo" --from validator --node http://localhost:26657 --chain-id testing --gas-prices 1900nhash --gas auto --gas-adjustment 1.3 --output json -b block --no-admin -y