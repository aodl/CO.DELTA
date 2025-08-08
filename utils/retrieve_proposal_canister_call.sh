#!/usr/bin/env bash
set -e

# Check if at least 2 argument is provided
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

PAYLOAD_TYPE="$4"

echo "Attempting to retrieve proposal id $PROPOSAL_ID ..."
RESPONSE=$(dfx canister call $THRESHOLD_CANISTER getProposal "($PROPOSAL_ID:nat)" --network=$NETWORK)
echo
echo $RESPONSE
echo
echo "Attempting to decode payload blob ..."
# Get the raw blob hex output (without backslashes) into a variable.
CANISTER_ARG=$(dfx canister call $THRESHOLD_CANISTER getProposalPayload "($PROPOSAL_ID:nat)" --network=$NETWORK --output idl | sed -E 's/^\(\s*opt blob\s+"([^"]*)"\s*\)$/\1/'  | tr -d '\\')

# Check if CANISTER_ARG is empty or not
if [ -z "$CANISTER_ARG" ]; then
  echo "No canister args to decode"
else
  if [ "$PAYLOAD_TYPE" ]; then
    echo "$CANISTER_ARG" | didc decode --defs ../src/codelta_backend/codelta_backend.did --types $PAYLOAD_TYPE
  else
    echo "$CANISTER_ARG" | didc decode
  fi
fi
echo

GREEN='\033[0;32m'
NC='\033[0m' # No Color

if echo \""$RESPONSE"\" | grep -Eq ';*active = true;'; then
  echo -e "${GREEN}Proposal appears to be active. Type 'accept' or 'reject' and then press ENTER to vote, or type anything else to exit...${NC}"
  read -r user_input

  if [ "$user_input" == "accept" ]; then
    echo "Accepting the proposal..."
    dfx canister call $THRESHOLD_CANISTER accept "($PROPOSAL_ID:nat)" --network=$NETWORK
  elif [ "$user_input" == "reject" ]; then
    echo "Rejecting the proposal..."
    dfx canister call $THRESHOLD_CANISTER reject "($PROPOSAL_ID:nat)" --network=$NETWORK
  else
    echo "No action taken. Exiting."
  fi
else
  echo "Proposal is not active."
fi
