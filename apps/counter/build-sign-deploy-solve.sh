set -eo pipefail
set -x # Debug output.

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

# Construct a solution with the `42` decision var and the address we want to change the state to.
# TODO: This is super unwieldy - would be great if pintc could generate this.
# TODO: Don't use `Signed<Solution>`, instead just use `Solution`.
ANSWER="42"
SOLUTION=$(jq -n \
  --argjson intent_addr "$INTENT_ADDRESS_INIT" \
  --argjson answer "$ANSWER" \
'
{
  data: {
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
            value: 0
          }
        ]
      }
    ]
  },
  signature: [
    [
      227,149,64,152,61,122,243,188,139,161,53,210,43,86,106,204,89,249,201,75,200,88,214,81,248,111,37,27,148,225,87,74,110,213,26,68,171,171,18,221,207,212,83,56,94,250,152,9,44,100,237,37,49,208,239,95,229,91,202,99,66,13,148,225
    ],
    0
  ]
}')

# Submit the solution.
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$SOLUTION" \
  "http://localhost:$SERVER_PORT/submit-solution")

# Convert the solution CA into expected hash format (base64) via hexadecimal.
SOLUTION_CA_JSON="$RESPONSE"
SOLUTION_CA_HEX=$(echo $SOLUTION_CA_JSON | jq -r '.[]' | awk '{ printf "%02x", $1 }')
SOLUTION_CA_BASE64=$(echo $SOLUTION_CA_HEX | xxd -r -p | base64)
SOLUTION_CA_BASE64URL=$(echo $SOLUTION_CA_BASE64 | tr '+/' '-_')

# Check the outcome of the solution.
curl -X GET -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/solution-outcome/$SOLUTION_CA_BASE64URL"
