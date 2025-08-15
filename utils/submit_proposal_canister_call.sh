#!/usr/bin/env bash
set -e

# Check if at least 4 arguments are provided
if [ "$#" -lt 4 ]; then
  echo "Usage: $0 arg1_network arg2_proposalSummary arg3_targetCanister arg4_targetMethod arg5_methodArg"
  exit 1
fi

# First argument - network i.e. 'local' (for testing)  or 'ic' (for production)
NETWORK="$1"

# Second argument - summary text that will accompany the proposal
PROPOSAL_SUMMARY="$2"

# Third argument - principal of canister targeted by the proposal 
TARGET_CANISTER="$3"

# Fourth argument - name of the method that will be called
TARGET_METHOD="$4"

# Optional fifth argument - args for the method being called
METHOD_ARG="$5"

THRESHOLD_CANISTER="$6"
# Set production threshold canister in case argument not provided
if [ -z "$THRESHOLD_CANISTER" ]; then
  THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"
fi

PROPOSAL_TYPE="$7"

GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo
echo "Submit proposal to threshold canister '$THRESHOLD_CANISTER'"
echo "to call '$TARGET_METHOD' on canister '$TARGET_CANISTER' on the $NETWORK network?"
echo -e "${GREEN}(y/n): ${NC}"
read -r answer

# Check the answer
if [[ "$answer" =~ ^[Yy]$ ]]; then
  echo "Proceeding..."
else
  echo "Aborting."
  exit 1
fi

mkdir -p canister_call_pipeline

if [ -z "$METHOD_ARG" ]; then
  echo "No method args to encode"
  BLOB="vec { }"
else
  echo "About to encode METHOD_ARG $METHOD_ARG"
  if [ "$PROPOSAL_TYPE" ]; then
    didc encode --defs ../src/codelta_backend/codelta_backend.did --types $PROPOSAL_TYPE "$METHOD_ARG" > canister_call_pipeline/arg.bin
  else
    didc encode "$METHOD_ARG" > canister_call_pipeline/arg.bin
  fi
  # Generate the blob from the canister call argument (didc encode converted to backslash-escaped vec hex sequence)
  BLOB=$(cat canister_call_pipeline/arg.bin \
    | tr -d ' \n' \
    | xxd -r -p \
    | od -An -tu1 \
    | xargs echo -n \
    | sed 's/ /; /g' \
    | sed 's/^/vec {/' \
    | sed 's/$/}/')
  echo "Encoded method args: $BLOB"
  fi
echo

# embed the vec in the canister call proposal submission
dfx canister call $THRESHOLD_CANISTER submit "(
    \"$PROPOSAL_SUMMARY\",
    record {
        principal \"$TARGET_CANISTER\";
        \"$TARGET_METHOD\";
        $BLOB
    }
)" --network=$NETWORK

echo
echo "=== Done! ==="
