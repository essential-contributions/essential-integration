set -eo pipefail
set -x # Debug output.

# Directory of this script.
SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"

# Compile the intent set.
pintc "$SCRIPT_DIR/forty-two.pnt"

# Sign the single inner intent and update JSON.
# TODO: This is currently nonsense - should add a minimal `essential-sign` CLI tool for this.
JSON_FILE="$SCRIPT_DIR/forty-two.json"
JSON_DATA=$(jq '{"data":[."::answer_question"],"signature":[[100,25,101,148,47,130,47,56,47,165,202,216,89,197,144,111,42,202,172,74,97,30,127,140,102,214,209,174,205,231,153,25,117,170,44,154,227,176,209,112,199,140,57,172,196,159,236,175,202,60,19,233,44,50,192,49,175,17,62,171,223,151,50,57],1]}' "$JSON_FILE")

# Deploy the intent set. Assumes the following server port.
SERVER_PORT="45539"
curl -X POST -H "Content-Type: application/json" \
  -d "$JSON_DATA" \
  "http://localhost:$SERVER_PORT/deploy-intent-set"

# List all intents.
curl -X GET -H "Content-Type: application/json" \
  "http://localhost:$SERVER_PORT/list-intent-sets?start=0&end=1&page=0"
