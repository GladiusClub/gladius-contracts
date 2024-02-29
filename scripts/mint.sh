#!/bin/bash

soroban contract invoke \
 --id $TOKEN_CONTRACT_ID \
 --network testnet \
 --source-account $CONTRACT_OWNER \
 --rpc-url https://rpc-futurenet.stellar.org:443 \
 --network-passphrase 'Test SDF Future Network ; October 2022' \
 -- mint \
 --to $TO_ACCOUNT \
 --amount 10
