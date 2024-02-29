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

# Creating the .soroban folder if does not exist yet.
mkdir -p .soroban
# Also the .soroban/identity folder if does not exist
mkdir -p .soroban/identity


################# NETWORK CONFIGURATION
echo -e "${RED}===${NC}"
echo -e "${GREEN}NETWORK CONFIGURATION${NC}"
echo "   "
echo "  SOROBAN_NETWORK_PASSPHRASE: $SOROBAN_NETWORK_PASSPHRASE"
echo "  FRIENDBOT_URL: $FRIENDBOT_URL"
echo "  SOROBAN_RPC_URL: $SOROBAN_RPC_URL"
echo "   "

# setting the network config to the CLI client
echo "  Adding the $NETWORK network to cli client"
soroban config network add "$NETWORK" \
  --rpc-url "$SOROBAN_RPC_URL" \
  --network-passphrase "$SOROBAN_NETWORK_PASSPHRASE"
echo "   "
echo -e "${RED}===${NC}"


#################### ACCOUNTS CONFIGURATION
echo -e "${RED}===${NC}"

echo -e "${GREEN}ACCOUNTS CONFIGURATION${NC}"
echo "   "
echo "  Checking secret keys from .env . If not, we will create new ones"
export $(grep -v '^#' .env | xargs)

# GLADIUS ADMIN
if [[ -n "$GLADIUS_ADMIN_SECRET" ]]; then
  echo "  Environmental variable GLADIUS_ADMIN_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$GLADIUS_ADMIN_SECRET\"" > .soroban/identity/gladius-admin.toml
  else
    echo "  Environmental variable GLADIUS_ADMIN_SECRET is not set"
    if !(soroban config identity ls | grep gladius-admin 2>&1 >/dev/null); then
      echo Create the gladius-admin identity
      soroban keys generate --no-fund --network $NETWORK gladius-admin
    fi
fi
GLADIUS_ADMIN_SECRET="$(soroban keys show gladius-admin)"
GLADIUS_ADMIN_ADDRESS="$(soroban keys address gladius-admin)"

echo "  GLADIUS_ADMIN_SECRET: $GLADIUS_ADMIN_SECRET"
echo "  GLADIUS_ADMIN_ADDRESS: $GLADIUS_ADMIN_ADDRESS"
echo "   "

echo "$GLADIUS_ADMIN_SECRET" > .soroban/gladius_admin_secret
echo "$GLADIUS_ADMIN_ADDRESS" > .soroban/gladius_admin_address


# GLADIUS ADMIN
if [[ -n "$PEGGED_TOKEN_ADMIN_SECRET" ]]; then
  echo "  Environmental variable PEGGED_TOKEN_ADMIN_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$PEGGED_TOKEN_ADMIN_SECRET\"" > .soroban/identity/token-admin.toml
  else
    echo "  Environmental variable PEGGED_TOKEN_ADMIN_SECRET is not set"
    if !(soroban config identity ls | grep token-admin 2>&1 >/dev/null); then
      echo Create the token-admin identity
      soroban keys generate --no-fund --network $NETWORK token-admin
    fi
fi
PEGGED_TOKEN_ADMIN_SECRET="$(soroban keys show token-admin)"
PEGGED_TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"

echo "  PEGGED_TOKEN_ADMIN_SECRET: $PEGGED_TOKEN_ADMIN_SECRET"
echo "  PEGGED_TOKEN_ADMIN_ADDRESS: $PEGGED_TOKEN_ADMIN_ADDRESS"
echo "   "

echo "$PEGGED_TOKEN_ADMIN_SECRET" > .soroban/token_admin_secret
echo "$PEGGED_TOKEN_ADMIN_ADDRESS" > .soroban/token_admin_address


# NEW_KEYS_OBJECT="{ \"network\": \"$NETWORK\", \
# \"gladius_admin_public\": \"$GLADIUS_ADMIN_ADDRESS\", \
# \"gladius_admin_secret\": \"$GLADIUS_ADMIN_SECRET\" \
# \"token_admin_secret\": \"$PEGGED_TOKEN_ADMIN_ADDRESS\" \
# \"token_admin_public\": \"$PEGGED_TOKEN_ADMIN_PUBLIC\" \
# \}"
# # echo "New keys object: $NEW_KEYS_OBJECT"

# KEYS_FILE="/workspace/.soroban/keys.json"
# touch $KEYS_FILE
# CURRENT_KEYS_JSON=$(cat $KEYS_FILE)
# # echo "CURRENT_KEYS_JSON: $CURRENT_KEYS_JSON"


# # check if the network already exists in that json
# exists=$(echo "$CURRENT_KEYS_JSON" | jq '.[] | select(.network == "'$NETWORK'")')
# # echo "This network already exist in the keys.json? : $exists"

# NEW_KEYS_JSON="{}"
# if [[ -n "$CURRENT_KEYS_JSON" ]]; then
#     if [[ -n "$exists" ]]; then
#         # if the network exists, update the keys for that network
#         # echo "Network exists, replacing"
#         NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '
#             map(if .network == "'"$NETWORK"'" then '"$NEW_KEYS_OBJECT"' else . end)'
#         )
#     else
#         # if the network doesn't exist, append the new object to the list
#         # echo "Network does not exist, appending"
#         NEW_KEYS_JSON=$(echo "$CURRENT_KEYS_JSON" | jq '. + ['"$NEW_KEYS_OBJECT"']')
#     fi
# else
#     # if the file is empty, initialize with a new object
#     # echo "File is empty, initializing"
#     NEW_KEYS_JSON=$(echo '['"$NEW_KEYS_OBJECT"']')
# fi

# # echo "NEW_KEYS_JSON: $NEW_KEYS_JSON"
# echo "$NEW_KEYS_JSON" > "$KEYS_FILE"

# echo "Token gladius-admin information available in /workspace/.soroban/keys.json"
# # cat /workspace/.soroban/token_admin_keys.json

echo "   "
echo -e "${RED}===${NC}"


# This will fail if the account already exists, but it'll still be fine.
echo Funding gladius-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$GLADIUS_ADMIN_ADDRESS" > /dev/null
echo "   "
echo Funding token-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$PEGGED_TOKEN_ADMIN_ADDRESS" > /dev/null
echo -e "${RED}===${NC}"