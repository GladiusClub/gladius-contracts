#!/bin/bash
# Enable the 'exit immediately' shell option
set -e

# Accept command-line arguments
NETWORK="$1"
source /workspace/scripts/setup.sh $NETWORK

# Define the arguments to be passed to the 'soroban contract' commands
ARGS="--network $NETWORK --source gladius-admin"

echo -e "${RED}===${NC}"
echo -e "${GREEN}CONTRACTS COMPILATION${NC}"

# Compile the gladius-coin-emitter contract
echo "Compiling the gladius-coin-emitter contract"
cd /workspace/contracts/gladius-coin-emitter
make build

echo "--"
echo "--"

# Compile the gladius-coin-emitter contract
echo "Compiling the token contract"
cd /workspace/contracts/token
make build


# Define the paths to the compiled WASM files
GLADIUS_COIN_EMITTER_WASM="/workspace/contracts/gladius-coin-emitter/target/wasm32-unknown-unknown/release/gladius_coin_emitter.wasm"
TOKEN_WASM="/workspace/contracts/token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"

echo -e "${RED}===${NC}"
echo -e "${GREEN}PEGGED TOKEN CONTRACT DEPLOYMENT${NC}"

# Deploy the TOKEN contract
echo "Deploy and setup the Pegged Token contract"
TOKEN_ID="$(
  soroban contract deploy $ARGS \
    --wasm $TOKEN_WASM
)"
echo "$TOKEN_ID" > /workspace/.soroban/pegged_token_id
echo "Pegged token (EURC) deployed successfully with TOKEN_ID: $TOKEN_ID"

NAME="EURC Token"
SYMBOL="EURC"
PEGGED_TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"
echo Initializing token with NAME $NAME  and SYMBOL $SYMBOL

soroban contract invoke \
  --network $NETWORK --source token-admin \
  --id $TOKEN_ID \
  -- \
  initialize \
  --admin "$PEGGED_TOKEN_ADMIN_ADDRESS" \
  --decimal 7 \
  --name "$NAME" \
  --symbol "$SYMBOL"


echo -e "${RED}===${NC}"
echo -e "${GREEN}GLADIUS COINT EMITTER CONTRACT DEPLOYMENT${NC}"
# Deploy the GLADIUS_COIN_EMITTER contract
GLADIUS_COIN_EMITTER_ID="$(
  soroban contract deploy $ARGS \
    --wasm $GLADIUS_COIN_EMITTER_WASM
)"
echo "$GLADIUS_COIN_EMITTER_ID" > /workspace/.soroban/gladius_coin_emitter_id
echo "GladiusCoinEmitter deployed successfully with GLADIUS_COIN_EMITTER_ID: $GLADIUS_COIN_EMITTER_ID"
echo "   "
RATIO="1000"
echo "Initializing the GladiusCoinEmitter to use the $NAME pegged token, GLADIUS_ADMIN_ADDRESS=$GLADIUS_ADMIN_ADDRESS and RATIO=$RATIO"
GLADIUS_ADMIN_ADDRESS="$(soroban keys address gladius-admin)"

soroban contract invoke \
  $ARGS \
  --id $GLADIUS_COIN_EMITTER_ID \
  -- \
  initialize \
  --admin $GLADIUS_ADMIN_ADDRESS \
  --pegged $TOKEN_ID \
  --ratio $RATIO

# echo "--"
# echo "--"



NEW_DEPLOYMENTS_OBJECT="{ \"network\": \"$NETWORK\", \"payment_token_admin_public\": \"$PEGGED_TOKEN_ADMIN_ADDRESS\", \"gladius_admin_public\": \"$GLADIUS_ADMIN_ADDRESS\", \"token_id\": \"$TOKEN_ID\",  \"gladius_emitter_id\": \"$GLADIUS_COIN_EMITTER_ID\"}"

DEPLOYMENTS_FILE="/workspace/.soroban/deployments.json"
if [[ ! -f "$DEPLOYMENTS_FILE" ]]; then
    # echo file not found
    echo "[]" > "$DEPLOYMENTS_FILE"
fi

CURRENT_DEPLOYMENTS_JSON=$(cat $DEPLOYMENTS_FILE)
# echo "CURRENT_DEPLOYMENTS_JSON: $CURRENT_DEPLOYMENTS_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_DEPLOYMENTS_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
# echo "This network already exist in the factory.json? : $exists"

NEW_DEPLOYMENTS_JSON="{}"
if [[ -n "$exists" ]]; then
    # if the network exists, update the factory for that network
    # echo "  network exists, replace"
    NEW_DEPLOYMENTS_JSON=$(echo "$CURRENT_DEPLOYMENTS_JSON" | jq '
        map(if .network == "'$NETWORK'" then '"$NEW_DEPLOYMENTS_OBJECT"' else . end)'
    )
else
    # if the network doesn't exist, append the new object to the list
    # echo "  network does not exist, append"
    NEW_DEPLOYMENTS_JSON=$(echo "$CURRENT_DEPLOYMENTS_JSON" | jq '. += ['"$NEW_DEPLOYMENTS_OBJECT"']')
fi

# echo "NEW_DEPLOYMENTS_JSON: $NEW_DEPLOYMENTS_JSON"
echo "$NEW_DEPLOYMENTS_JSON" > "$DEPLOYMENTS_FILE"


# # Save the network and factory information in a JSON file
# jq -n \
#   --arg network "$NETWORK" \
#   --arg factory_id "$FACTORY_ID" \
#   --arg factory_address "$FACTORY_ADDRESS" \
#   '[{"network": $network, "factory_id": $factory_id, "factory_address": $factory_address}]' \
#   > /workspace/.soroban/factory.json



# Output the file path and contents
echo "Dedployments information available in $DEPLOYMENTS_FILE"
cat $DEPLOYMENTS_FILE
