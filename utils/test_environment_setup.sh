# THIS SCRIPT IS INTENDED AS A CONVENIENCE THAT INITIALISES REQUIRED CONTROLLER RELATIONSHIPS BETWEEN TEST CANISTERS
#
# PREREQUISITES:
# - you've already called `chmod +x test_environment_setup.sh` to allow this script to be executed using the following command `./test_environment_setup.sh`
# - you've already called `dfx start --clean --background` to start the test replica (you'll also want to call `dfx nns install` for the sake of testing methods that interact with the ledger). Tip, run these two commands after calling `tmux new -s replica`, then type `CTRL B + D` to detach from that session. This will run the replica from a separate session (useful given that it can produce noisy console outputs). You can always re-attach to that session to see what's going on by typing `tmux attach -t replica`.
# - you've then called `dfx deploy` in this repo (to create the local test versions of the frontend and backend canister)
# - you've then called the following from the threshold repo (a separate repo that defines the threshold canister). Note you have to specify at least two principals when initialising that canister (these are the principals with voting power for proposals). We specify the same canister id as production for convenience (there's not a lot that can go wrong if you accidentally submit a proposal to the production instance, `--network=ic`, given that any proposal will need to go through consensus before it can take any effect).
# dfx deploy threshold --argument='(vec {principal "'$(dfx identity get-principal)'"; principal "<...some other test principal... you need at least 2>"})' --specified-id 6g7za-ziaaa-aaaar-qaqja-cai

echo "retrieve principal id of local codelta_backend canister ..."
CODELTA_BACKEND=$(dfx canister id codelta_backend)
echo "retrieve principal id of local codelta_frontend canister ..."
CODELTA_FRONTEND=$(dfx canister id codelta_frontend)

echo "add local $CODELTA_BACKEND as controller of the local threshold canister ..."
dfx canister update-settings 6g7za-ziaaa-aaaar-qaqja-cai --add-controller $CODELTA_BACKEND
echo "add local threshold as controller of the local threshold canister ..."
dfx canister update-settings 6g7za-ziaaa-aaaar-qaqja-cai --add-controller 6g7za-ziaaa-aaaar-qaqja-cai
echo "add local $CODELTA_BACKEND as controller of the local $CODELTA_FRONTEND canister ..."
dfx canister update-settings $CODELTA_FRONTEND --add-controller $CODELTA_BACKEND
echo "add local threshold as controller of the local $CODELTA_FRONTEND canister ..."
dfx canister update-settings $CODELTA_FRONTEND --add-controller 6g7za-ziaaa-aaaar-qaqja-cai
echo "add local threshold as controller of the local $CODELTA_BACKEND canister ..."
dfx canister update-settings $CODELTA_BACKEND --add-controller 6g7za-ziaaa-aaaar-qaqja-cai

## Uncomment below if you'd like to remove the original controller of these local test canisters
## to more accurately reflect the production set up, but at the expense of some testing flexibility
# MY_PRINCIPAL=$(dfx identity get-principal)
# dfx canister update-settings 6g7za-ziaaa-aaaar-qaqja-cai --remove-controller $MY_PRINCIPAL
# dfx canister update-settings $CODELTA_FRONTEND --remove-controller $MY_PRINCIPAL
# dfx canister update-settings $CODELTA_BACKEND --remove-controller $MY_PRINCIPAL

echo "== Done! Let's get testing... =="
