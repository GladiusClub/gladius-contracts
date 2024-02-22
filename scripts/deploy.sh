#!/bin/bash

# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"

source /workspace/scripts/setup.sh $NETWORK

# Define the arguments to be passed to the 'soroban contract' commands
ARGS="--network $NETWORK --source gladius-admin"
echo "Using ARGS: $ARGS"

# Create a directory for Soroban files if it doesn't exist
mkdir -p .soroban

echo "--"
echo "--"

# Compile the gladius-coin-emitter contract
echo "Compiling the gladius-coin-emitter contract"
cd /workspace/contracts/gladius-coin-emitter
make build

echo "--"
echo "--"

# Define the paths to the compiled WASM files
GLADIUS_COIN_EMITTER_WASM="/workspace/contracts/gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.optimized.wasm"

# Deploy the GLADIUS_COIN_EMITTER contract
echo "Deploy the Gladius Coint Emitter contract"
GLADIUS_COIN_EMITTER_ID="$(
  soroban contract deploy $ARGS \
    --wasm $GLADIUS_COIN_EMITTER_WASM
)"
echo "$GLADIUS_COIN_EMITTER_ID" > /workspace/.soroban/gladius_coin_emitter_id
echo "SoroswapFactory deployed successfully with GLADIUS_COIN_EMITTER_ID: $GLADIUS_COIN_EMITTER_ID"

echo "--"
echo "--"

# Get the admin address
ADMIN_ADDRESS="$(soroban keys address gladius-admin)"

# echo "Initialize the SoroswapFactory contract"
# soroban contract invoke \
#   $ARGS \
#   --id $FACTORY_ID \
#   -- \
#   initialize \
#   --setter "$TOKEN_ADMIN_ADDRESS" \
#   --pair_wasm_hash "$PAIR_WASM_HASH"

# echo "--"
# echo "--"

# FACTORY_ADDRESS="$(node /workspace/scripts/address_workaround.js $FACTORY_ID)"

# # Create the new FACTORY object with the updated factory id and addresses
# NEW_FACTORY_OBJECT="{ \"network\": \"$NETWORK\", \"factory_id\": \"$FACTORY_ID\", \"factory_address\": \"$FACTORY_ADDRESS\" }"
# echo "New factory object: $NEW_FACTORY_OBJECT"
# # NEW_FACTORY_OBJECT="{ \"network\": \"futurenet\", \"factory_id\": \"5adb2e4748f175bcc1ab4e11c0f03bc275701ef556cd9d2b10becb37ea6a33c9\", \"factory_address\": \"CBNNWLSHJDYXLPGBVNHBDQHQHPBHK4A66VLM3HJLCC7MWN7KNIZ4SLNG\"}"

# FACTORY_FILE="/workspace/.soroban/factory.json"
# # Initialize factory.json if it does not exist
# if [[ ! -f "$FACTORY_FILE" ]]; then
#     echo file not found
#     echo "[]" > "$FACTORY_FILE"
# fi


# CURRENT_FACTORY_JSON=$(cat $FACTORY_FILE)
# echo "CURRENT_FACTORY_JSON: $CURRENT_FACTORY_JSON"


# # check if the network already exists in that json
# exists=$(echo "$CURRENT_FACTORY_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
# echo "This network already exist in the factory.json? : $exists"

# NEW_FACTORY_JSON="{}"
# if [[ -n "$exists" ]]; then
#     # if the network exists, update the factory for that network
#     echo network exists, replace
#     NEW_FACTORY_JSON=$(echo "$CURRENT_FACTORY_JSON" | jq '
#         map(if .network == "'$NETWORK'" then '"$NEW_FACTORY_OBJECT"' else . end)'
#     )
# else
#     # if the network doesn't exist, append the new object to the list
#     echo network does not exist, append
#     NEW_FACTORY_JSON=$(echo "$CURRENT_FACTORY_JSON" | jq '. += ['"$NEW_FACTORY_OBJECT"']')
# fi

# # echo "NEW_FACTORY_JSON: $NEW_FACTORY_JSON"
# echo "$NEW_FACTORY_JSON" > "$FACTORY_FILE"

# echo "end creating the factory" 


# # # Save the network and factory information in a JSON file
# # jq -n \
# #   --arg network "$NETWORK" \
# #   --arg factory_id "$FACTORY_ID" \
# #   --arg factory_address "$FACTORY_ADDRESS" \
# #   '[{"network": $network, "factory_id": $factory_id, "factory_address": $factory_address}]' \
# #   > /workspace/.soroban/factory.json



# # Output the file path and contents
# echo "Factory information available in /workspace/.soroban/factory.json"
# cat /workspace/.soroban/factory.json
