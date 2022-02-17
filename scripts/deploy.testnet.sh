#!/bin/bash
set -e

ID=murkwoodtale_v1.sergantche.testnet

# recreate account
# near delete $ID sergantche.testnet
# near create-account murkwoodtale_v1.sergantche.testnet --masterAccount=sergantche.testnet --initial-balance 50
near create-account $ID --masterAccount=sergantche.testnet --initial-balance 50

# deploy contract
# near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId $ID
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId murkwoodtale_v1.sergantche.testnet
# near call $ID new --accountId $ID
near call murkwoodtale_v1.sergantche.testnet new --accountId murkwoodtale_v1.sergantche.testnet

# copy credentials for later deploy
# cp ~/.near-credentials/testnet/$ID.json ./creds
cp ~/.near-credentials/testnet/murkwoodtale_v1.sergantche.testnet.json ./creds