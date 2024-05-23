set -eo pipefail

# ---------------------------------------------------------
# BUILD
# ---------------------------------------------------------

# Directory of this script.
NAME="counter"
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Compile the intent set.
pintc "$SCRIPT_DIR/$NAME.pnt"

# Use `jq` to change the JSON from an object to a list.
# TODO: `pintc` should address this upstream: essential-contributions/pint#597.
INTENT_SET_JSON_FILE="$SCRIPT_DIR/$NAME.json"
jq '[.[]]' $INTENT_SET_JSON_FILE > tmp.json && mv tmp.json $INTENT_SET_JSON_FILE

# ---------------------------------------------------------
# SIGN
# ---------------------------------------------------------

# Create a keypair to sign with.
KEYPAIR_JSON=$(essential generate-keys)
PRIVATE_KEY_JSON=$(echo $KEYPAIR_JSON | jq -c ."private")

# Sign the single inner intent and update JSON.
SIGNED_INTENT_SET_JSON_FILE="$SCRIPT_DIR/$NAME-signed.json"
essential sign-intent-set --private-key-json "$PRIVATE_KEY_JSON" $INTENT_SET_JSON_FILE > $SIGNED_INTENT_SET_JSON_FILE

# ---------------------------------------------------------
# DEPLOY
# ---------------------------------------------------------

# Deploy the intent set. Assumes the following server port.
SERVER_PORT="45539"
JSON_DATA=$(jq . $SIGNED_INTENT_SET_JSON_FILE)
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$JSON_DATA" \
  "http://localhost:$SERVER_PORT/deploy-intent-set")

# Retrieve the intent addresses (contains only the one intent address in this
# case). Note: As the intent set JSON from pint is currently an object (i.e. a
# map from name to intent), the intents are ordered by their name when converted
# to a list (not their declaration order within the pint file).
INTENT_ADDRESSES=$(essential intent-addresses $INTENT_SET_JSON_FILE)
INTENT_ADDRESS_INCREMENT=$(echo $INTENT_ADDRESSES | jq -c '.[0]')
INTENT_ADDRESS_INIT=$(echo $INTENT_ADDRESSES | jq -c '.[1]')

# Before continuing, ensure that the response we got from the server when we
# deployed the intent set matches the INTENT_SET_CA we expect.
INTENT_SET_CA=$(echo $INTENT_ADDRESSES | jq -c '.[0]."set"')

if [ "$RESPONSE" != "$INTENT_SET_CA" ]; then
  echo "Error: RESPONSE does not match INTENT_SET_CA"
  echo "RESPONSE: $RESPONSE"
  echo "INTENT_SET_CA: $INTENT_SET_CA"
  exit 1
fi

# ---------------------------------------------------------
# SOLVE
# ---------------------------------------------------------

# Construct a solution to initialise the `counter` to `0`.
# TODO: Don't use `Signed<Solution>`, instead just use `Solution`.
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
          value: [0]
        }
      ]
    }
  ]
}')

# Submit the solution.
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$SOLUTION" \
  "http://localhost:$SERVER_PORT/submit-solution")

# Convert the solution CA into expected hash format (base64) via hexadecimal.
SOLUTION_CA="$(echo $RESPONSE | awk -F'"' '{print $2}')"

# Check the outcome of the solution.
curl -X GET -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/solution-outcome/$SOLUTION_CA"
