NETWORK=$1

RED='\033[1;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m'

case "$1" in
standalone)
    echo "Using Standalone network"
  ;;
futurenet)
    echo "Using Futurenet network"
  ;;
testnet)
  echo "Using Testnet network"
  ;;
*)
  echo "Usage: $0 standalone|futurenet|testnet"
  exit 1
  ;;
esac

CONFIG_FILE="/workspace/config.json"

FRIENDBOT_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .friendbot_url' "$CONFIG_FILE")
SOROBAN_RPC_URL=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_rpc_url' "$CONFIG_FILE")
SOROBAN_NETWORK_PASSPHRASE=$(jq -r --arg NETWORK "$NETWORK" '.networkConfig[] | select(.network == $NETWORK) | .soroban_network_passphrase' "$CONFIG_FILE")

# Creating the .soroban folder if does not exist yet
mkdir -p .soroban

echo "==="
echo "   "
echo SOROBAN_NETWORK_PASSPHRASE: $SOROBAN_NETWORK_PASSPHRASE
echo FRIENDBOT_URL: $FRIENDBOT_URL
echo SOROBAN_RPC_URL: $SOROBAN_RPC_URL
echo "   "
echo "==="

# Always set a net configuration 
echo Adding the $NETWORK network to cli client
soroban config network add "$NETWORK" \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"
echo "==="

  if !(soroban config identity ls | grep gladius-admin 2>&1 >/dev/null); then
    echo Create the gladius-admin identity
    soroban keys generate --no-fund --network $NETWORK gladius-admin
  fi
ADMIN_SECRET="$(soroban keys show gladius-admin)"
ADMIN_ADDRESS="$(soroban keys address gladius-admin)"
echo "   "
echo "ADMIN_ADDRESS: $ADMIN_ADDRESS"
echo "ADMIN_SECRET: $ADMIN_SECRET"

echo "$ADMIN_SECRET" > .soroban/gladius_admin_secret
echo "$ADMIN_ADDRESS" > .soroban/gladius_admin_address

NEW_KEYS_OBJECT="{ \"network\": \"$NETWORK\", \"admin_public\": \"$ADMIN_ADDRESS\", \"admin_secret\": \"$ADMIN_SECRET\" }"
# echo "New keys object: $NEW_KEYS_OBJECT"

KEYS_FILE="/workspace/.soroban/keys.json"
touch $KEYS_FILE
CURRENT_KEYS_JSON=$(cat $KEYS_FILE)
# echo "CURRENT_KEYS_JSON: $CURRENT_KEYS_JSON"


# check if the network already exists in that json
exists=$(echo "$CURRENT_KEYS_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
# echo "This network already exist in the keys.json? : $exists"

NEW_KEYS_JSON="{}"
if [[ -n "$CURRENT_KEYS_JSON" ]]; then
    if [[ -n "$exists" ]]; then
        # if the network exists, update the keys for that network
        # echo "Network exists, replacing"
        NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '
            map(if .network == "'"$NETWORK"'" then '"$NEW_KEYS_OBJECT"' else . end)'
        )
    else
        # if the network doesn't exist, append the new object to the list
        # echo "Network does not exist, appending"
        NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '. + ['"$NEW_KEYS_OBJECT"']')
    fi
else
    # if the file is empty, initialize with a new object
    # echo "File is empty, initializing"
    NEW_KEYS_JSON=$(echo '['"$NEW_KEYS_OBJECT"']')
fi

# echo "NEW_KEYS_JSON: $NEW_KEYS_JSON"
echo "$NEW_KEYS_JSON" > "$KEYS_FILE"

echo "Token gladius-admin information available in /workspace/.soroban/keys.json"
# cat /workspace/.soroban/token_admin_keys.json

echo "   "
echo "==="

# This will fail if the account already exists, but it'll still be fine.
echo Fund gladius-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$ADMIN_ADDRESS" > /dev/null