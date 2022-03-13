#!/bin/bash
set -e

ID=eq2.meta_irony.testnet

# create subaccount
# near delete $ID meta_irony.testnet # uncomment to delete old account
near create-account $ID --masterAccount=meta_irony.testnet --initial-balance 50

# deploy contract
near deploy --wasmFile contract/target/wasm32-unknown-unknown/release/near_backend.wasm --accountId $ID
near call $ID new --accountId $ID

# copy credentials for later deploy
cp ~/.near-credentials/testnet/$ID.json ./creds