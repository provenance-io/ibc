#!/bin/bash

export PROVENANCE_DIR="$HOME/code/provenance"
export BIN="$PROVENANCE_DIR/build/provenanced"
export RUN_HOME="$PROVENANCE_DIR/build/run/provenanced"
export GAS_FLAGS="--gas auto --gas-prices 1905nhash --gas-adjustment 1.5"
export CHAIN="$BIN -t --home $RUN_HOME"
export VALIDATOR1=$($CHAIN keys show validator -a)
export CONTRACT_ADDRESS="tp1ghd753shjuwexxywmgs4xz7x2q732vcnkm6h2pyv9s6ah3hylvrqdzfary"

${CHAIN} tx wasm execute $CONTRACT_ADDRESS "{\"transfer_auth_funds\": {\"to_address\": \"tp13g9hxkljph90nt2waxtw3a40fkkz0dta3sgztv\", \"granter_address\": \"${VALIDATOR1}\", \"amount\": \"420\", \"denom\": \"nhash\"} }" --amount 100nhash --from $VALIDATOR1 --fees 382000000nhash -y -o json | jq
${CHAIN} q bank balances tp13g9hxkljph90nt2waxtw3a40fkkz0dta3sgztv -o json | jq