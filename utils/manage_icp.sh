#!/usr/bin/env bash
set -e

THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"

# Check if at least 1 arguments are provided
if [ "$#" -lt 1 ]; then
  echo "Usage: $0 arg1_network"
  exit 1
fi

# network i.e. 'local' (for testing)  or 'ic' (for production)
NETWORK="$1"

echo "Attempting to retrieve current balance in the codelta_backend default account ..."
echo
dfx canister call codelta_backend check_balance --network=$NETWORK
echo

GREEN='\033[0;32m'
NC='\033[0m' # No Color

echo -e "${GREEN}Do you want to submit a proposal to trigger a distribution to CO.DELTA team members? (y/n): ${NC}"
read -r 
if [[ $REPLY =~ ^[Yy]$ ]]; then
  echo "Attempting to retrieve codelta_backend principal for specifying in the proposal..."
  CODELTA_BACKEND=$(dfx canister id codelta_backend --network=$NETWORK)
  echo $CODELTA_BACKEND
  # Submit proposal to distibute ICP to CO.DELTA team
  ./submit_proposal_canister_call.sh $NETWORK 'Distribute ICP evenly between CO.DELTA team' $CODELTA_BACKEND distribute_icp '(  
    vec {    
      "c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad";
      "f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df";
      "a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69" 
    }
  )'
else
  echo "No action taken."
fi
