# Proposal Management Workflow

**This folder contains a collection of bash utilities** to simplify the process of submitting and reviewing threshold proposals. Changes cannot be made to any CO.DELTA canister without team member consensus, which requires proposals to be submitted to the threshold canister. >50% of the team must accept the proposal for the change to take effect. ICP distributions (from the codelta_backend canister's default account to CO.DELTA team members) all requires consensus amongst the team.

**A generally useful command** to run is to review a summary of the most recent proposals - `dfx canister call 6g7za-ziaaa-aaaar-qaqja-cai getProposalSummaries --network=IC` (or `--network=local` if you're reviewing test proposals that were submitted on a local replica). Note that `6g7za-ziaaa-aaaar-qaqja-cai` is the threshold canister principal. Also note that this summary list won't display proposal payloads. You can drill into specific proposals with other commands. This process is massively simplified by utilising the following utility scripts.

**Refer to this readme for an explanation of how to**:
- [Check the codelta_backend canister ICP balance and optionally trigger a distribution](/utils/README.md#check-the-codelta_backend-canister-icp-balance-and-optionally-trigger-a-distribution)
- [Retrieve a submitted proposal, inspect the details, and vote to accept/reject](/utils/README.md#retrieve-a-submitted-proposal-inspect-the-details-and-vote-to-acceptreject)
- [Submit any arbitrary proposal](/utils/README.md#submit-any-arbitrary-proposal)
- [Submit a proposal to upgrade one of the three canisters](/utils/README.md#submit-a-proposal-to-upgrade-one-of-the-three-canisters)
- [Retrieve a canister upgrade proposal and compute the hash](/utils/README.md#retrieve-a-canister-upgrade-proposal-and-compute-the-hash)

### Check the codelta_backend canister ICP balance and optionally trigger a distribution

Use the `manage_icp.sh` script as follows. Be sure to first execute `chmod +x manage_icp.sh` to allow execution of the bash script.

- Execute `./manage_icp.sh local` for testing distribution with local canisters (see `test_environment_setup.sh` for some notes)
- Execute `./manage_icp.sh ic` to check the real balance of the production codelta_backend canister

```console
root@BuildMachine:/home/CO_DELTA/CODELTA/codelta/utils# ./manage_icp.sh local SubnetManagement $THRESHOLD_CANISTER_ID
Attempting to retrieve current balance in the codelta_backend SubnetManagement account ...

Please enter the passphrase for your identity: [hidden]
Decryption complete.
(variant { Ok = "1000.00000001 Token" })

Do you want to submit a proposal to trigger a distribution to CO.DELTA SubnetManagement team members? (y/n): 
y
Attempting to retrieve codelta_backend principal for specifying in the proposal...
bd3sg-teaaa-aaaaa-qaaba-cai

Submit proposal to threshold canister '6g7za-ziaaa-aaaar-qaqja-cai'
to call 'distribute_icp' on canister 'bd3sg-teaaa-aaaaa-qaaba-cai' on the local network?
(y/n): 
y
Proceeding...
About to encode METHOD_ARG (  
    variant {
      SubnetManagement
    }
  )
Encoded method args: vec {68; 73; 68; 76; 1; 107; 18; 213; 165; 229; 1; 127; 143; 217; 227; 10; 127; 184; 215; 206; 40; 127; 201; 251; 153; 134; 1; 127; 193; 205; 253; 202; 1; 127; 246; 161; 216; 209; 1; 127; 241; 185; 160; 129; 3; 127; 224; 223; 247; 189; 3; 127; 195; 181; 161; 182; 4; 127; 141; 214; 205; 146; 6; 127; 243; 251; 230; 203; 6; 127; 162; 168; 242; 189; 8; 127; 240; 176; 245; 236; 8; 127; 223; 237; 170; 173; 10; 127; 246; 183; 203; 177; 11; 127; 194; 218; 130; 222; 11; 127; 239; 205; 244; 194; 12; 127; 151; 247; 225; 195; 14; 127; 1; 0; 7}

Please enter the passphrase for your identity: [hidden]
Decryption complete.
(6 : nat)

=== Done! ===
```

As demonstrated above, you can follow the sequence of prompts to optionally submit a proposal to distribute that ICP evenly amongst CO.DELTA team members.

The output above indicates that the proposal was submitted and that the **proposal id is 6**.

### Retrieve a submitted proposal, inspect the details, and vote to accept/reject

If you've followed the steps above, the next thing you'll want to do is confirm that your proposal looks okay. Your team members will also need to carry out this step in order to vote on that proposal.

Use the `retrieve_proposal_canister_call.sh` script as follows. Be sure to first execute `chmod +x retrieve_proposal_canister_call.sh` to allow execution of the bash script.

- Execute `./retrieve_proposal_canister_call.sh local <proposal_id>` if your testing a local fake proposal
- Execute `./retrieve_proposal_canister_call.sh ic <proposal_id>` if your reviewing a live production proposal

e.g.

```console
root@BuildMachine:/home/CO_DELTA/CODELTA/codelta/utils# ./retrieve_proposal_canister_call.sh local 6
Attempting to retrieve proposal id 6 ...
Please enter the passphrase for your identity: [hidden]
Decryption complete.

( opt record { id = 6 : nat; memo = "Distribute ICP evenly between CO.DELTA team"; state = record { no = 0 : nat; yes = 1 : nat; result = null; active = true; votes = vec { record { 1_742_058_491 : nat; principal "zkkkd-i34qc-367ln-e2u7o-ezznu-dkfqh-gtfvz-cviph-6qa4v-evtfs-wqe"; }; }; }; payload = record { principal "bd3sg-teaaa-aaaaa-qaaba-cai"; "distribute_icp"; blob "\44\49\44\4c\01\6d\71\01\00\03\40\63\35\62\37\39\31\64\66\38\39\30\39\38\33\32\30\65\64\31\39\33\66\33\65\30\32\36\66\30\31\31\63\32\39\39\39\61\31\39\31\35\37\36\34\39\32\36\61\30\61\31\61\32\35\34\61\39\39\30\62\31\36\61\64\40\66\36\61\37\66\64\65\38\66\65\64\39\38\30\66\38\37\65\34\63\39\65\63\36\66\65\30\34\38\32\30\63\39\66\64\37\30\39\61\38\61\36\65\38\35\64\65\62\36\61\65\61\33\63\31\63\31\64\33\30\63\30\64\66\40\61\32\37\30\35\30\33\32\34\36\35\30\63\32\65\63\35\64\32\39\61\35\61\37\30\30\33\31\33\36\63\37\30\36\30\38\64\64\63\31\36\36\65\61\64\31\63\34\35\36\35\36\62\33\61\62\33\63\32\62\63\66\36\39"; }; }, )

Attempting to decode payload blob ...
Please enter the passphrase for your identity: [hidden]
Decryption complete.
(
  vec {
    "c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad";
    "f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df";
    "a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69";
  },
)

Proposal appears to be active. Type 'accept' or 'reject' and then press ENTER to vote, or type anything else to exit...
accept
Accepting the proposal...
Please enter the passphrase for your identity: [hidden]
Decryption complete.
()
```

The above demonstrates the prompts that you can go through, allowing you to easily decode and observed the ICP distribution argument (the account IDs that the ICP will be distributed to, i.e c5b791..., f6a7fd..., a27050...).

If all looks good you can type accept, press enter, and then your password (if your pem file is encrypted).

After voting you can run the script again and observe that the yes/no vote tally has changed. As soon as >= 50% of team members have voted yes, the proposal will execute and the ICP will be distributed.

You can check the codelta_backend logs to inspect details about the distribution, e.g. `dfx canister logs codelta_backend --network=local` (use `--network=ic` for inspecting real distributions rather that test distributions) ->

```
[3. 2025-03-15T14:35:45.87144692Z]: Call to distribute_icp(c5b791d..., f6a7fde..., a270503...) initiated by threshold consensus
[4. 2025-03-15T14:35:45.87144692Z]: Distributing canister's account balance (1000.00000000 Token) among 3 accounts; each share after transfer fee: 333.33323333 Token
[5. 2025-03-15T14:35:45.87144692Z]: Transfer of 333.33323333 Token to c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad succeeded in block index 6
[6. 2025-03-15T14:35:45.87144692Z]: Transfer of 333.33323333 Token to f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df succeeded in block index 7
[7. 2025-03-15T14:35:45.87144692Z]: Transfer of 333.33323333 Token to a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69 succeeded in block index 8
```

### Submit any arbitrary proposal

You can submit any arbitrary proposal, by specifying a canister, a method to call on that canister, and optionally an argument to pass to that method.

This is simplified by using the `submit_proposal_canister_call.sh` script. Be sure to first execute `chmod +x submit_proposal_canister_call.sh` to allow execution of the bash script.

Call `./submit_proposal_canister_call.sh` to retrieve usage information (the same approach can be used for other scripts). ->

```console
Usage: ./submit_proposal_canister_call.sh arg1_network arg2_proposalSummary arg3_targetCanister arg4_targetMethod arg5_methodArg
```

Note that the `manage_icp.sh` script mentioned above uses this script behind the scenes. Also note that the encoded argument is written to a file that you can inspect afterwards, if desired (`utils/canister_call_pipeline/arg.bin`). You can decode this using `didc decode --defs ../src/codelta_backend/codelta_backend.did --types '(Topic)'` (e.g. `cat canister_call_pipeline/arg.bin | didc decode --defs ../src/codelta_backend/codelta_backend.did --types '(Topic)'`). This may be useful if something went wrong and you'd like to inspect the encoded argument that was used.

### Submit a proposal to upgrade one of the three canisters

Whenever there's a need to update the codelta_backend or codelta_frontend canisters (or rarely the threshold canister), it will need to be done via a proposal, as no individual has control over these canisters.

After you've built and tested the changes, you need to provide the file path for the WASM, along with an optional upgrade argument.

This is simplified by using the `submit_proposal_canister_upgrade.sh` script. Be sure to first execute `chmod +x submit_proposal_canister_upgrade.sh` to allow execution of the bash script.

Call `./submit_proposal_canister_upgrade.sh` to retrieve usage information (the same approach can be used for other scripts). ->

```console
Usage: ./submit_proposal_canister_upgrade.sh arg1_network arg2_upgradeCanisterPrincipal arg3_wasmFilePath arg4_upgradeArg
```

Note that various files are generated in the `Utils/canister_upgrade_pipeline/` folder just in case you need to inspect the stages of encoding arguments, and/or converting the WASM bytes to a format that's suitable for passing into the proposal (a vec of backslash-escaped hex bytes).

**Important:** If there's ever a need to upgrade the threshold canister itself, the same approach can be used, however there are some precautions that should be taken. A botched upgrade to the threshold cansiter can result in a bricked system. Before upgrading the threshold canister (if this is ever actually needed) a second threshold canister should first be set up, with identical configuration and control priviledges. If the upgrade of one of these threshold canisters fails in a way that leaves the canister in a broken state, the healthy threshold canister can be used to rectify the situation. Setting up the second threshold canister will require several proposals to be submitted to the original threshold canister first (to give the second threshold canister the same level of control i.e. updating canister settings to add it as a controller. Refer to the main README in this repo for the exact settings necessary).

### Retrieve a canister upgrade proposal and compute the hash 

This script is particularly cool, and it massively simplifies the process of reviewing and verifying canister upgrade proposals. e.g.

```console
root@BuildMachine:/home/CO_DELTA/CODELTA/codelta/utils# ./retrieve_proposal_canister_upgrade.sh local 4

Attempting to retrieve canister arg for upgrade proposal id 4 ...
Please enter the passphrase for your identity: [hidden]
Decryption complete.
4449444c016d68010003011dcace99cee61c5c5bfe57dd67b24651a475d35cf841751cbb4620b55802011d7c80b7efada4d53ee2672da0d4581cd32d722aa1e7f401ca92b32cad02011d55caa35b16e2ef57b9ca9423919444ebe04d3024389290a52085910b02
(
  vec {
    principal "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae";
    principal "zkkkd-i34qc-367ln-e2u7o-ezznu-dkfqh-gtfvz-cviph-6qa4v-evtfs-wqe";
    principal "hfbtd-e2vzk-rvwfx-c55l3-tsuue-oizir-hl4bg-tajby-skikk-iefse-fqe";
  },
)

Attempting to retrieve canister WASM for upgrade proposal id 4 ...
Please enter the passphrase for your identity: [hidden]
Decryption complete.
Saved WASM to canister_upgrade_pipeline/module.wasm

Attempting to compute WASM Sha256...
ef860db565eed8e2322c60a9931300d05d20826adf1e00a96cb8a294cfe4ea85  canister_upgrade_pipeline/module.wasm

=== Done! ===

If the proposal is active you can accept or reject by executing 'dfx canister call 6g7za-ziaaa-aaaar-qaqja-cai accept 4 --network=local' or 'dfx canister call 6g7za-ziaaa-aaaar-qaqja-cai reject 4 --network=local'. You can confirm your vote was received by calling 'dfx canister call 6g7za-ziaaa-aaaar-qaqja-cai getProposal 4 --network=local' and observing the vote tally
```

The above demonstrates an upgrade proposal for the threshold cansiter. The argument is the list of signers (threshold voters). This is just shown as a demonstration (the signers can only actually be reset during initial install, or reinstall, not during an upgrade). The proper way to reset the signers list is to raise a proposal that calls the `setSigners` method on the threshold canister (see the [threshold repo](https://github.com/aodl/threshold/blob/main/threshold.mo)).
