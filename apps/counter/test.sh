# This test script does the following:
#
# 1. Builds the pint contract.
# 2. Signs the contract.
# 3. Deploys the contract.
# 4. Solves the `init` predicate and waits for inclusion in a block.
# 5. Solves the `increment` predicate and waits for inclusion in a block.

set -eo pipefail
temp_dir=$(mktemp -d)

# Take the server port as an argument.
SERVER_PORT="45539"
if [ -n "$1" ]; then
  SERVER_PORT=$1
fi

# ---------------------------------------------------------
# BUILD
# ---------------------------------------------------------

NAME="counter"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PINT_FILE="$SCRIPT_DIR/pint/pint.toml"
echo "Building $PINT_FILE"
pint --manifest-path "$PINT_FILE" 
echo "Built $PINT_FILE"

# ---------------------------------------------------------
# SIGN
# ---------------------------------------------------------

# Create a keypair and sign the contract.
INTENT_SET_JSON_FILE="$temp_dir/$NAME.json"
echo "Signing $INTENT_SET_JSON_FILE"
KEYPAIR_JSON=$(essential generate-keys)
PRIVATE_KEY_JSON=$(echo $KEYPAIR_JSON | jq -c ."private")
SIGNED_INTENT_SET_JSON=$(essential sign-contract \
  --private-key-json "$PRIVATE_KEY_JSON" $INTENT_SET_JSON_FILE)

# ---------------------------------------------------------
# DEPLOY
# ---------------------------------------------------------

# Deploy the contract. Assumes the following server port.
echo "Deploying signed contract"
echo $SIGNED_INTENT_SET_JSON | jq '.'
RESPONSE=$(curl -X POST --http2-prior-knowledge -H "Content-Type: application/json" \
  -d "$SIGNED_INTENT_SET_JSON" \
  "http://localhost:$SERVER_PORT/deploy-predicate-set")
echo "$RESPONSE" | jq '.'

# Retrieve the predicate addresses (ordered by name).
INTENT_ADDRESSES=$(essential predicate-addresses $INTENT_SET_JSON_FILE)
INTENT_ADDRESS_INCREMENT=$(echo $INTENT_ADDRESSES | jq -c '.[0]')
INTENT_ADDRESS_INIT=$(echo $INTENT_ADDRESSES | jq -c '.[1]')
INTENT_SET_CA=$(echo $INTENT_ADDRESSES | jq -c '.[0]."set"')

# Make sure the deploy response matches our contract CA.
if [ "$RESPONSE" != "$INTENT_SET_CA" ]; then
  echo "Error: RESPONSE does not match INTENT_SET_CA"
  echo "RESPONSE: $RESPONSE"
  echo "INTENT_SET_CA: $INTENT_SET_CA"
  exit 1
fi

# ---------------------------------------------------------
# SOLVE `init`
# ---------------------------------------------------------

# Construct a solution to initialise the `counter` to `0`.
SOLUTION=$(jq -n \
  --argjson predicate_addr "$INTENT_ADDRESS_INIT" \
'
{
  data: [
    {
      predicate_to_solve: $predicate_addr,
      decision_variables: [],
      state_mutations: [
        {
          key: [0],
          value: [0]
        }
      ],
      transient_data: []
    }
  ]
}')

echo "Submitting 'init' solution"
echo $SOLUTION | jq '.'
RESPONSE=$(curl -X POST --http2-prior-knowledge -H "Content-Type: application/json" \
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
    RESPONSE=$(curl -s -X GET --http2-prior-knowledge -H "Content-Type: application/json" \
      "http://localhost:$SERVER_PORT/solution-outcome/$SOLUTION_CA")
    echo $RESPONSE | jq '.'

    # Check for success.
    SUCCESS=$(echo $RESPONSE | jq 'if type == "array" and length == 0 then false else map(has("Success")) | any end')
    if [ "$SUCCESS" == "true" ]; then
      break
    fi

    # Check for failure.
    FAIL=$(echo $RESPONSE | jq 'if type == "array" and length == 0 then false else map(has("Fail")) | any end')
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
KEY="AAAAAAAAAAAAAAAA" # Key `[0u8; 8]` as base64url
echo "Querying state $ADDRESS/$KEY"
RESPONSE=$(curl -X GET --http2-prior-knowledge -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/query-state/$ADDRESS/$KEY")
echo "$RESPONSE" | jq .

# ---------------------------------------------------------
# SOLVE `increment`
# ---------------------------------------------------------

# Construct a solution to increment the counter.
PREV_COUNT=$(echo $RESPONSE | jq '.[0]')
NEXT_COUNT=$(expr $PREV_COUNT + 1)
SOLUTION=$(jq -n \
  --argjson answer "42" \
  --argjson predicate_addr "$INTENT_ADDRESS_INCREMENT" \
  --argjson next_count "$NEXT_COUNT" \
'
{
  data: [
    {
      predicate_to_solve: $predicate_addr,
      decision_variables: [[$answer]],
      state_mutations: [
        {
          key: [0],
          value: [$next_count]
        }
      ],
      transient_data: []
    }
  ]
}')

# Submit the solution.
echo "Submitting 'increment' solution"
echo $SOLUTION | jq '.'
RESPONSE=$(curl -X POST --http2-prior-knowledge -H "Content-Type: application/json" \
  -d "$SOLUTION" \
  "http://localhost:$SERVER_PORT/submit-solution")

# Await inclusion in a block.
SOLUTION_CA=$(echo $RESPONSE | jq -r '.')
await_solution_outcome $SOLUTION_CA
