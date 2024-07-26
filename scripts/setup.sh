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
if [[ -n "$PAYMENT_TOKEN_ADMIN_SECRET" ]]; then
  echo "  Environmental variable PAYMENT_TOKEN_ADMIN_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$PAYMENT_TOKEN_ADMIN_SECRET\"" > .soroban/identity/token-admin.toml
  else
    echo "  Environmental variable PAYMENT_TOKEN_ADMIN_SECRET is not set"
    if !(soroban config identity ls | grep token-admin 2>&1 >/dev/null); then
      echo Create the token-admin identity
      soroban keys generate --no-fund --network $NETWORK token-admin
    fi
fi
PAYMENT_TOKEN_ADMIN_SECRET="$(soroban keys show token-admin)"
PEGGED_TOKEN_ADMIN_ADDRESS="$(soroban keys address token-admin)"

echo "  PAYMENT_TOKEN_ADMIN_SECRET: $PAYMENT_TOKEN_ADMIN_SECRET"
echo "  PEGGED_TOKEN_ADMIN_ADDRESS: $PEGGED_TOKEN_ADMIN_ADDRESS"
echo "   "

echo "$PAYMENT_TOKEN_ADMIN_SECRET" > .soroban/token_admin_secret
echo "$PEGGED_TOKEN_ADMIN_ADDRESS" > .soroban/token_admin_address


# GLADIUS SPORTS CLUB ADMIN
if [[ -n "$SPORT_CLUB_SECRET" ]]; then
  echo "  Environmental variable SPORT_CLUB_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$SPORT_CLUB_SECRET\"" > .soroban/identity/club-admin.toml
  else
    echo "  Environmental variable SPORT_CLUB_SECRET is not set"
    if !(soroban config identity ls | grep club-admin 2>&1 >/dev/null); then
      echo Create the club-admin identity
      soroban keys generate --no-fund --network $NETWORK club-admin
    fi
fi
SPORT_CLUB_SECRET="$(soroban keys show club-admin)"
SPORT_CLUB_ADDRESS="$(soroban keys address club-admin)"

echo "  SPORT_CLUB_SECRET: $SPORT_CLUB_SECRET"
echo "  SPORT_CLUB_ADDRESS: $SPORT_CLUB_ADDRESS"
echo "   "

echo "$SPORT_CLUB_SECRET" > .soroban/club_admin_secret
echo "$SPORT_CLUB_ADDRESS" > .soroban/club_admin_address

# PARENT
if [[ -n "$PARENT_SECRET" ]]; then
  echo "  Environmental variable PARENT_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$PARENT_SECRET\"" > .soroban/identity/club-parent.toml
  else
    echo "  Environmental variable PARENT_SECRET is not set"
    if !(soroban config identity ls | grep club-parent 2>&1 >/dev/null); then
      echo Create the club-parent identity
      soroban keys generate --no-fund --network $NETWORK club-parent
    fi
fi
PARENT_SECRET="$(soroban keys show club-parent)"
PARENT_ADDRESS="$(soroban keys address club-parent)"

echo "  PARENT_SECRET: $PARENT_SECRET"
echo "  PARENT_ADDRESS: $PARENT_ADDRESS"
echo "   "

echo "$PARENT_SECRET" > .soroban/parent_admin_secret
echo "$PARENT_ADDRESS" > .soroban/parent_admin_address

# STUDENT
if [[ -n "$STUDENT_SECRET" ]]; then
  echo "  Environmental variable STUDENT_SECRET is already set"
  echo "  Setting up the account with this secret key"
  echo "secret_key = \"$STUDENT_SECRET\"" > .soroban/identity/club-STUDENT.toml
  else
    echo "  Environmental variable STUDENT_SECRET is not set"
    if !(soroban config identity ls | grep club-STUDENT 2>&1 >/dev/null); then
      echo Create the club-STUDENT identity
      soroban keys generate --no-fund --network $NETWORK club-STUDENT
    fi
fi
STUDENT_SECRET="$(soroban keys show club-STUDENT)"
STUDENT_ADDRESS="$(soroban keys address club-STUDENT)"

echo "  STUDENT_SECRET: $STUDENT_SECRET"
echo "  STUDENT_ADDRESS: $STUDENT_ADDRESS"
echo "   "

echo "$STUDENT_SECRET" > .soroban/club_student_secret
echo "$STUDENT_ADDRESS" > .soroban/club_student_address




# NEW_KEYS_OBJECT="{ \"network\": \"$NETWORK\", \
# \"gladius_admin_public\": \"$GLADIUS_ADMIN_ADDRESS\", \
# \"gladius_admin_secret\": \"$GLADIUS_ADMIN_SECRET\" \
# \"token_admin_secret\": \"$PEGGED_TOKEN_ADMIN_ADDRESS\" \
# \"token_admin_public\": \"$payment_token_admin_public\" \
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
echo "   "
echo Funding club-admin account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$SPORT_CLUB_ADDRESS" > /dev/null
echo -e "${RED}===${NC}"
echo "   "
echo Funding club-parent account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$PARENT_ADDRESS" > /dev/null
echo -e "${RED}===${NC}"
echo "   "
echo Funding club-STUDENT account from friendbot
echo This will fail if the account already exists, but it\' still be fine.
curl  -X POST "$FRIENDBOT_URL?addr=$STUDENT_ADDRESS" > /dev/null
echo -e "${RED}===${NC}"