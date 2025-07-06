#!/usr/bin/env bash
set -e

THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"

# Check if at least 3 arguments are provided
if [ "$#" -lt 3 ]; then
  echo "Usage: $0 arg1_network arg2_upgradeCanisterPrincipal arg3_wasmFilePath arg4_upgradeArg"
  exit 1
fi

# First argument - network i.e. "local" (for testing) or "ic" (for production)
NETWORK="$1"

# Second argument - the principal of the canister you actually want to upgrade
UPGRADE_CANISTER="$2"

# Third argument - the compiled Wasm file you want installed
WASM_FILE="$3"

# Optional fourth argument - args for the canister being upgraded
UPGRADE_ARG="$4"

THRESHOLD_CANISTER="$5"
# Set production threshold canister in case argument not provided
if [ -z "$THRESHOLD_CANISTER" ]; then
  THRESHOLD_CANISTER="6g7za-ziaaa-aaaar-qaqja-cai"
fi

echo
echo "Submit proposal to threshold canister '$THRESHOLD_CANISTER' to upgrade canister"
echo "'$UPGRADE_CANISTER' by installing WASM '$WASM_FILE' on the $NETWORK network?"
read -e -p $'\e[32m(y/n): \e[0m' answer

# Check the answer
if [[ "$answer" =~ ^[Yy]$ ]]; then
  echo "Proceeding..."
else
  echo "Aborting."
  exit 1
fi

mkdir -p canister_upgrade_pipeline
echo

# Remove previous pipeline artefacts
rm -f canister_upgrade_pipeline/arg.bin
rm -f canister_upgrade_pipeline/upgrade_request_record.did
rm -f canister_upgrade_pipeline/upgrade_request_record.bin
rm -f canister_upgrade_pipeline/proposal_submission_record.did
rm -f canister_upgrade_pipeline/module.wasm

if [ -z "$UPGRADE_ARG" ]; then
  echo "No upgrade args to encode"
  touch canister_upgrade_pipeline/arg.bin
else
  didc encode "$UPGRADE_ARG" > canister_upgrade_pipeline/arg.bin
fi
echo "Created canister_upgrade_pipeline/arg.bin"
echo
# Backslash-escaped hex bytes
ARG_ESCAPED=$(cat canister_upgrade_pipeline/arg.bin | sed 's/../\\&/g')
echo "Backslash-escaped bytes in canister_upgrade_pipeline/arg.bin"
echo

# Create the upgrade request request record in textual Candid format
# This record includes the actual .wasm bytes as a blob: "\00\61\62\63..."
# Write did to file for pipeline inspection in case this is needed
cat > canister_upgrade_pipeline/upgrade_request_record.did <<EOF
(record {
  mode = variant { upgrade = null };
  canister_id = principal "$UPGRADE_CANISTER";
  wasm_module = blob "$(hexdump -ve '1/1 "%.2x"' "$WASM_FILE" | sed 's/../\\&/g')";
  arg = blob "$ARG_ESCAPED";
})
EOF
echo "Created canister_upgrade_pipeline/upgrade_request_record.did"
echo

# Encode the upgrade request record to hex with didc
cat canister_upgrade_pipeline/upgrade_request_record.did | didc encode > canister_upgrade_pipeline/upgrade_request_record.bin
echo "Created canister_upgrade_pipeline/upgrade_request_record.bin"
echo
# Backslash-escaped hex bytes
INNER_ESCAPED=$(cat canister_upgrade_pipeline/upgrade_request_record.bin | sed 's/../\\&/g')
echo "Backslash-escaped bytes in canister_upgrade_pipeline/upgrade_request_record.bin"
echo


# Create proposal submission record  (the outer record that the threshold canister expects)
cat > canister_upgrade_pipeline/proposal_submission_record.did <<EOF
(
  "Canister upgrade",
  record {
    principal "aaaaa-aa";
    "install_code";
    blob "$INNER_ESCAPED"
  }
)
EOF
echo "Created canister_upgrade_pipeline/proposal_submission_record.did"
echo

echo "Proposal id will be returned below upon successful submission..."
# Submit canister upgrade proposal to the threshold canister
dfx canister call "$THRESHOLD_CANISTER" submit --argument-file canister_upgrade_pipeline/proposal_submission_record.did --network=$NETWORK

echo
echo "=== Done! ==="

