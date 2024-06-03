# This test script does the following:
#
# 1. Builds the pint contract.
# 2. Signs the intent set.
# 3. Deploys the intent set.
# 4. Solves the `mint` intent and waits for inclusion in a block.
# 5. Solves the `transfer` intent and waits for inclusion in a block.
# 6. Solves the `burn` intent and waits for inclusion in a block.

set -eo pipefail
temp_dir=$(mktemp -d)

echo "Taking server port as an argument."

# Take the server port as an argument.
SERVER_PORT="45539"
if [ -n "$1" ]; then
  SERVER_PORT=$1
fi

echo "Took server port as an argument."

# ---------------------------------------------------------
# BUILD
# ---------------------------------------------------------

NAME="fungible-token"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PINT_FILE="$SCRIPT_DIR/$NAME.pnt"
echo "Building $PINT_FILE"
pintc "$PINT_FILE" --output "$temp_dir/$NAME.json"
echo "Built $PINT_FILE"

# ---------------------------------------------------------
# SIGN
# ---------------------------------------------------------

# Create a keypair and sign the intent set.
INTENT_SET_JSON_FILE="$temp_dir/$NAME.json"
echo "Signing $INTENT_SET_JSON_FILE"
KEYPAIR_JSON=$(essential generate-keys)
PRIVATE_KEY_JSON=$(echo $KEYPAIR_JSON | jq -c ."private")
SIGNED_INTENT_SET_JSON=$(essential sign-intent-set \
  --private-key-json "$PRIVATE_KEY_JSON" $INTENT_SET_JSON_FILE)

# ---------------------------------------------------------
# DEPLOY
# ---------------------------------------------------------

# Deploy the intent set. Assumes the following server port.
echo "Deploying signed intent set"
echo $SIGNED_INTENT_SET_JSON | jq '.'
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$SIGNED_INTENT_SET_JSON" \
  "http://localhost:$SERVER_PORT/deploy-intent-set")
echo "$RESPONSE" | jq '.'

# Retrieve the intent addresses (ordered by name).
INTENT_ADDRESSES=$(essential intent-addresses $INTENT_SET_JSON_FILE)
INTENT_ADDRESS_MINT=$(echo $INTENT_ADDRESSES | jq -c '.[0]')
INTENT_ADDRESS_TRANSFER=$(echo $INTENT_ADDRESSES | jq -c '.[1]')
INTENT_ADDRESS_BURN=$(echo $INTENT_ADDRESSES | jq -c '.[2]')
INTENT_SET_CA=$(echo $INTENT_ADDRESSES | jq -c '.[0]."set"')

# Make sure the deploy response matches our intent set CA.
if [ "$RESPONSE" != "$INTENT_SET_CA" ]; then
  echo "Error: RESPONSE does not match INTENT_SET_CA"
  echo "RESPONSE: $RESPONSE"
  echo "INTENT_SET_CA: $INTENT_SET_CA"
  exit 1
fi

# ---------------------------------------------------------
# SOLVE `mint`
# ---------------------------------------------------------

# Construct a solution to mint `42`` tokens.
SOLUTION=$(jq -n \
  --argjson intent_addr "$INTENT_ADDRESS_INIT" \
'
{
  data: [
    {
      intent_to_solve: $intent_addr,
      decision_variables: []
    }
  ],
  state_mutations: [
    {
      pathway: 0,
      mutations: [
        {
          key: [0,0,0,0],
          value: [0x11111111111111111111111111111111]
        },
        {
          key: [0,0,0,1],
          value: [42]
        }
      ]
    }
  ]
}')

echo "Submitting 'mint' solution"
echo $SOLUTION | jq '.'
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$SOLUTION" \
  "http://localhost:$SERVER_PORT/submit-solution")
echo "$RESPONSE" | jq '.'

# Wait until we can verify that our solution was included in a block.
# Should never take longer than the 10 second hard-coded block time.
await_solution_outcome() {
  local SOLUTION_CA=$1
  local MAX_RETRIES=10
  local counter=0
  echo "Awaiting outcome for solution $SOLUTION_CA"
  while true; do
    # Request outcome.
    RESPONSE=$(curl -s -X GET -H "Content-Type: application/json" \
      "http://localhost:$SERVER_PORT/solution-outcome/$SOLUTION_CA")
    echo $RESPONSE | jq '.'

    # Check for success.
    SUCCESS=$(echo $RESPONSE | jq 'has("Success")')
    if [ "$SUCCESS" == "true" ]; then
      break
    fi

    # Check for failure.
    FAIL=$(echo $RESPONSE | jq 'has("Fail")')
    if [ "$FAIL" == "true" ]; then
      echo "Error: Solution failed"
      echo "Exiting."
      exit 1
    fi

    # Check for max retries.
    if [ $counter -gt $MAX_RETRIES ] || [ "$FAIL" == "true" ]; then
      echo "Error: MAX_RETRIES reached"
      echo "Exiting."
      exit 1
    fi

    counter=$((counter + 1))
    sleep 1
  done
}

SOLUTION_CA=$(echo $RESPONSE | jq -r '.')
await_solution_outcome $SOLUTION_CA

# ---------------------------------------------------------
# CHECK STATE
# ---------------------------------------------------------

ADDRESS=$(echo $INTENT_SET_CA | jq -r '.')
KEY="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA" # Key `[0u8; 32]` as base64url
echo "Querying state $ADDRESS/$KEY"
RESPONSE=$(curl -X GET -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/query-state/$ADDRESS/$KEY")
echo "$RESPONSE" | jq .

ADDRESS=$(echo $INTENT_SET_CA | jq -r '.')
KEY="AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAE" # Key `[0u8; 32]` as base64url
echo "Querying state $ADDRESS/$KEY"
RESPONSE=$(curl -X GET -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/query-state/$ADDRESS/$KEY")
echo "$RESPONSE" | jq .

# # ---------------------------------------------------------
# # SOLVE `increment`
# # ---------------------------------------------------------

# # Construct a solution to increment the counter.
# PREV_COUNT=$(echo $RESPONSE | jq '.[0]')
# NEXT_COUNT=$(expr $PREV_COUNT + 1)
# SOLUTION=$(jq -n \
#   --argjson answer "42" \
#   --argjson intent_addr "$INTENT_ADDRESS_INCREMENT" \
#   --argjson next_count "$NEXT_COUNT" \
# '
# {
#   data: [
#     {
#       intent_to_solve: $intent_addr,
#       decision_variables: [$answer]
#     }
#   ],
#   state_mutations: [
#     {
#       pathway: 0,
#       mutations: [
#         {
#           key: [0,0,0,0],
#           value: [$next_count]
#         }
#       ]
#     }
#   ]
# }')

# # Submit the solution.
# echo "Submitting 'increment' solution"
# echo $SOLUTION | jq '.'
# RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
#   -d "$SOLUTION" \
#   "http://localhost:$SERVER_PORT/submit-solution")

# # Await inclusion in a block.
# SOLUTION_CA=$(echo $RESPONSE | jq -r '.')
# await_solution_outcome $SOLUTION_CA
