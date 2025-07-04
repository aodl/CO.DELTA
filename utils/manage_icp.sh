#!/usr/bin/env bash
set -e

# Check if at least 1 arguments are provided
if [ "$#" -lt 2 ]; then
  echo "Usage: $0 arg1_network $1 arg2_topic"
  exit 1
fi

# network i.e. 'local' (for testing)  or 'ic' (for production)
NETWORK="$1"

TOPIC="$2"

THRESHOLD_CANISTER="$3"
# Set production threshold canister in case argument not provided
if [ -z "$THRESHOLD_CANISTER" ]; then
  THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"
fi

echo "Attempting to retrieve current balance in the codelta_backend $TOPIC account ..."
echo
dfx canister call codelta_backend check_balance '(variant { '$TOPIC' })'  --network=$NETWORK
echo

GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Do you want to submit a proposal to trigger a distribution to CO.DELTA $TOPIC team members? (y/n): ${NC}"
read -r 
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo "Attempting to retrieve codelta_backend principal for specifying in the proposal..."
  CODELTA_BACKEND=$(dfx canister id codelta_backend --network=$NETWORK)
  echo $CODELTA_BACKEND
  # Submit proposal to distibute ICP to CO.DELTA team
  ./submit_proposal_canister_call.sh $NETWORK 'Distribute ICP evenly between CO.DELTA team' $CODELTA_BACKEND distribute_icp '(  
    variant {
      '$TOPIC'
    }
  )' $THRESHOLD_CANISTER
else
  echo "No action taken."
fi
