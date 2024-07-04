# This test script does the following:
#
# 1. Builds the pint contract.
# 2. Signs the contract.
# 3. Deploys the contract.
# 4. Solves the `increment` predicate for the first time and waits for inclusion in a block.
# 5. Solves the `increment` predicate again and waits for inclusion in a block.

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

NAME="contract"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PINT_FILE="$SCRIPT_DIR/pint/src/$NAME.pnt"
echo "Building $PINT_FILE"
cp $PINT_FILE "$temp_dir/$NAME.pnt"
cd $temp_dir
# Due to https://github.com/essential-contributions/pint/issues/714 we must copy
# the source to tmp dir and compile it there.
# pintc "$PINT_FILE" --output "$temp_dir/$NAME.json"
pintc "$temp_dir/$NAME.pnt" --output "$temp_dir/$NAME.json"
echo "Built $PINT_FILE"
cd $SCRIPT_DIR

# ---------------------------------------------------------
# SIGN
# ---------------------------------------------------------

# Create a keypair and sign the contract.
CONTRACT_JSON_FILE="$temp_dir/$NAME.json"
echo "Signing $CONTRACT_JSON_FILE"
KEYPAIR_JSON=$(essential generate-keys)
PRIVATE_KEY_JSON=$(echo $KEYPAIR_JSON | jq -c ."private")
SIGNED_CONTRACT_JSON=$(essential sign-contract \
  --private-key-json "$PRIVATE_KEY_JSON" $CONTRACT_JSON_FILE)

# ---------------------------------------------------------
# DEPLOY
# ---------------------------------------------------------

# Deploy the contract. Assumes the following server port.
echo "Deploying signed contract"
echo $SIGNED_CONTRACT_JSON | jq '.'
RESPONSE=$(curl -X POST --http2-prior-knowledge -H "Content-Type: application/json" \
  -d "$SIGNED_CONTRACT_JSON" \
  "http://localhost:$SERVER_PORT/deploy-contract")
echo "$RESPONSE" | jq '.'

# Retrieve the predicate addresses.
PREDICATE_ADDRESSES=$(essential predicate-addresses $CONTRACT_JSON_FILE)
PREDICATE_ADDRESS_INCREMENT=$(echo $PREDICATE_ADDRESSES | jq -c '.[0]')
CONTRACT_CA=$(echo $PREDICATE_ADDRESSES | jq -c '.[0]."contract"')

# Make sure the deploy response matches our contract content address.
if [ "$RESPONSE" != "$CONTRACT_CA" ]; then
  echo "Error: RESPONSE does not match CONTRACT_CA"
  echo "RESPONSE: $RESPONSE"
  echo "CONTRACT_CA: $CONTRACT_CA"
  exit 1
fi

# ---------------------------------------------------------
# SOLVE `increment`
# ---------------------------------------------------------

# Construct a solution to increment the counter for the first time.
SOLUTION=$(jq -n \
  --argjson predicate_addr "$PREDICATE_ADDRESS_INCREMENT" \
'
{
  data: [
    {
      predicate_to_solve: $predicate_addr,
      decision_variables: [],
      state_mutations: [
        {
          key: [0],
          value: [1]
        }
      ],
      transient_data: []
    }
  ]
}')

echo "Submitting 'increment' solution"
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

ADDRESS=$(echo $CONTRACT_CA | jq -r '.')
KEY="0000000000000000" # Key `[0u8; 8]` as hex
echo "Querying state $ADDRESS/$KEY"
RESPONSE=$(curl -X GET --http2-prior-knowledge -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/query-state/$ADDRESS/$KEY")
echo "$RESPONSE" | jq .

# ---------------------------------------------------------
# SOLVE `increment`
# ---------------------------------------------------------

# Construct a solution to increment the counter to 2.
PREV_COUNT=$(echo $RESPONSE | jq '.[0]')
NEXT_COUNT=$(expr $PREV_COUNT + 1)
SOLUTION=$(jq -n \
  --argjson answer "42" \
  --argjson predicate_addr "$PREDICATE_ADDRESS_INCREMENT" \
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
