set -eo pipefail
set -x # Debug output.

# ---------------------------------------------------------
# BUILD
# ---------------------------------------------------------

# Directory of this script.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Compile the intent set.
pintc "$SCRIPT_DIR/forty-two.pnt"

# ---------------------------------------------------------
# SIGN
# ---------------------------------------------------------

# Sign the single inner intent and update JSON.
# TODO: This is currently nonsense - should add a minimal `essential-sign` CLI tool for this.
JSON_FILE="$SCRIPT_DIR/forty-two.json"
JSON_DATA=$(jq '{"data":[."::answer_question"],"signature":[[100,25,101,148,47,130,47,56,47,165,202,216,89,197,144,111,42,202,172,74,97,30,127,140,102,214,209,174,205,231,153,25,117,170,44,154,227,176,209,112,199,140,57,172,196,159,236,175,202,60,19,233,44,50,192,49,175,17,62,171,223,151,50,57],1]}' "$JSON_FILE")

# ---------------------------------------------------------
# DEPLOY
# ---------------------------------------------------------

# Deploy the intent set. Assumes the following server port.
SERVER_PORT="45539"
RESPONSE=$(curl -X POST -H "Content-Type: application/json" \
  -d "$JSON_DATA" \
  "http://localhost:$SERVER_PORT/deploy-intent-set")

# TODO: Do error checking of response - currenlty we assume it's correct.

# ---------------------------------------------------------
# SOLVE
# ---------------------------------------------------------

# Construct a solution with the `42` decision var and the address we want to change the state to.
# TODO: This is super unwieldy - would be great if pintc could generate this.
ANSWER="42"
INTENT_SET_CA="$RESPONSE"
SOLUTION=$(jq -n \
  --argjson intent_set_ca "$INTENT_SET_CA" \
  --argjson answer "$ANSWER" \
'
{
  data: {
    data: [
      {
        intent_to_solve: {
          set: $intent_set_ca,
          intent: [0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0]
        },
        decision_variables: [
          {
            Inline: $answer
          }
        ]
      }
    ],
    state_mutations: [
      {
        pathway: 0,
        mutations: [
          {
            key: [0,0,0,0],
            value: 1
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
curl -X POST -H "Content-Type: application/json" \
  -d "$SOLUTION" \
  "http://localhost:$SERVER_PORT/submit-solution"
