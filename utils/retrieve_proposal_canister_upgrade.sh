#!/usr/bin/env bash
set -e

# Check if at least 2 arguments are provided
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 arg1_network arg2_proposal_id"
  exit 1
fi

# First argument - network i.e. "local" (for testing) or "ic" (for production)
NETWORK="$1"

# Second argument - proposal id passed in as argument
PROPOSAL_ID="$2"

THRESHOLD_CANISTER="$3"
# Set production threshold canister in case argument not provided
if [ -z "$THRESHOLD_CANISTER" ]; then
  THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"
fi

mkdir -p canister_upgrade_pipeline
rm -f canister_upgrade_pipeline/module.wasm
echo

echo "Attempting to retrieve canister arg for upgrade proposal id $PROPOSAL_ID ..."
# Get the raw blob hex output (without backslashes) into a variable.
CANISTER_ARG=$(dfx canister call $THRESHOLD_CANISTER getUpgradeArgBlob $PROPOSAL_ID --network=$NETWORK --output idl | sed -E 's/^\(\s*opt blob\s+"([^"]*)"\s*\)$/\1/'  | tr -d '\\')
echo $CANISTER_ARG
# Check if CANISTER_ARG is empty or not
if [ -z "$CANISTER_ARG" ]; then
  echo "No canister args to decode"
else
  echo "$CANISTER_ARG" | didc decode
fi
echo

echo "Attempting to retrieve canister WASM for upgrade proposal id $PROPOSAL_ID ..."
dfx canister call $THRESHOLD_CANISTER getUpgradeWasmBlob $PROPOSAL_ID --network=$NETWORK --output idl \
  | sed -n 's/.*blob "\([^"]*\)".*/\1/p'  \
  | tr -d '\n'  \
  | xxd -r -p > canister_upgrade_pipeline/module.wasm
echo "Saved WASM to canister_upgrade_pipeline/module.wasm"
echo
echo "Attempting to compute WASM Sha256..."
sha256sum canister_upgrade_pipeline/module.wasm
echo
echo "=== Done! ==="
echo
echo "If the proposal is active you can accept or reject by executing 'dfx canister call $THRESHOLD_CANISTER accept $PROPOSAL_ID --network=$NETWORK' or 'dfx canister call $THRESHOLD_CANISTER reject $PROPOSAL_ID --network=$NETWORK'. You can confirm your vote was received by calling 'dfx canister call $THRESHOLD_CANISTER getProposal $PROPOSAL_ID --network=$NETWORK' and observing the vote tally"
