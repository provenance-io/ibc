#!/bin/bash
provenanced tx wasm store output/authzdemo.wasm --from owner --node http://localhost:26657 --chain-id testing --gas-prices 1900nhash --gas auto --gas-adjustment 1.3 --broadcast-mode block -y --output json -b block