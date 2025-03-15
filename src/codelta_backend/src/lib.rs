use candid::{ Principal, Nat };
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse
};
use ic_cdk_macros::update;
use icp_ledger::{
    AccountIdentifier, Memo, Tokens, TransferArgs, TransferError
};
use icrc_ledger_client_cdk::{ CdkRuntime, ICRC1Client };
use icrc_ledger_types::icrc1::account::Account;

const DEFAULT_FEE: Tokens = Tokens::from_e8s(10_000); // 0.0001 ICP

const MEMBER_PRINCIPALS: &[&str] = &[
    "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae", // aligatorr89
    "zkkkd-i34qc-367ln-e2u7o-ezznu-dkfqh-gtfvz-cviph-6qa4v-evtfs-wqe", // Lorimer
    "hfbtd-e2vzk-rvwfx-c55l3-tsuue-oizir-hl4bg-tajby-skikk-iefse-fqe", // MalithHatananchchige
];

const MEMBER_ACCOUNTS: &[&str] = &[
    "c5b791df89098320ed193f3e026f011c2999a1915764926a0a1a254a990b16ad", // aligatorr89
    "f6a7fde8fed980f87e4c9ec6fe04820c9fd709a8a6e85deb6aea3c1c1d30c0df", // Lorimer
    "a27050324650c2ec5d29a5a7003136c70608ddc166ead1c45656b3ab3c2bcf69", // MalithHatananchchige
];

// Threshold canister can only be operated by proposal and threshold consensus among members (>50%)
const THRESHOLD_PRINCIPAL: &str = "6g7za-ziaaa-aaaar-qaqja-cai";

const LEDGER_CANISTER_ID: &str = "ryjl3-tyaaa-aaaaa-aaaba-cai";

/// A helper function that checks if `ic_cdk::caller()` is allowed (see guards below)
fn contains_caller(allowed: &[&str]) -> Result<(), String> {
    let caller = ic_cdk::caller();
    // Convert the Principal to its string representation:
    let caller_str = caller.to_text();
    // Check if that string is in the allowed list:
    if allowed.contains(&caller_str.as_str()) {
        Ok(())
    } else {
        Err(format!(
            "Caller {} is not in the allowed list: {:?}",
            caller, allowed
        ))
    }
}

/// Guard for allowing only member calls
fn only_members_guard() -> Result<(), String> {
    contains_caller(&MEMBER_PRINCIPALS)
}

/// Guard for allowing only threshold calls
fn only_threshold_guard() -> Result<(), String> {
    contains_caller(&[THRESHOLD_PRINCIPAL])
}

#[update(guard = "only_members_guard")]
async fn check_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    let record = CanisterIdRecord { canister_id };
    let (status_response,): (CanisterStatusResponse,) = canister_status(record)
        .await
        .map_err(|(code, msg)| format!("Failed calling canister_status: code={:?}, msg={}", code, msg))?;
    Ok(status_response)
}

async fn _check_balance() -> Result<Tokens, String> {
    // Instantiate client for the ICP Ledger (ryjl3-tyaaa-aaaaa-aaaba-cai)
    let client = ICRC1Client { runtime: CdkRuntime, ledger_canister_id: Principal::from_text(LEDGER_CANISTER_ID).unwrap() };
    // Default canister account (no subaccount)
    let canister_account = Account { owner: ic_cdk::id(), subaccount: None };
    // Get the canister's current ICP balance
    let canister_balance_e8: Nat = match client.balance_of(canister_account.clone()).await {
        Ok(balance) => balance,
        Err((_code, err_msg)) => {
            let msg = format!("Failed to get canister balance: {}", err_msg);
            ic_cdk::println!("{}", msg);
            return Err(msg);
        }
    };
    Ok(Tokens::from_e8s(canister_balance_e8.0.try_into().unwrap()))
}

#[update(guard = "only_members_guard")]
async fn check_balance() -> Result<String, String> {
    Ok(_check_balance().await.unwrap().to_string())
}

#[update(guard = "only_threshold_guard")]
async fn distribute_icp(target_account_ids: Vec<String>) -> Result<(), String> {

	// Get short form of ids for logging
    let truncated_ids: Vec<String> = target_account_ids.iter()
        .map(|id| {
            if id.len() > 7 { format!("{}...", &id[..7]) } else { id.clone() }
        })
        .collect();

    ic_cdk::println!("Call to distribute_icp({}) initiated by threshold consensus", truncated_ids.join(", "));
	
    // Bail out if no target account IDs provided
    if target_account_ids.is_empty() {
        let msg = "No target account IDs provided.".to_string();
        ic_cdk::println!("{}", msg);
        return Err(msg);
    }
	
    // Verify each target ID is in the allowed list
    for id in &target_account_ids {
        if !MEMBER_ACCOUNTS.contains(&id.as_str()) {
            let msg = format!("Account ID {} is not in the list of allowed IDs.", id);
            ic_cdk::println!("{}", msg);
            return Err(msg);
        }
    }
	
    let icp_balance: Tokens = _check_balance().await?;

    // Calculate how much each account should receive
    let num_accounts = Nat::from(target_account_ids.len() as u64);
    // Perform the division and safely convert to u64
    let e8s_share: u64 = (icp_balance.get_e8s().clone() / num_accounts)
        .0.try_into()
        .map_err(|_| "Conversion error: Value exceeds u64 limit".to_string())?;

    // Bail out if the share is not above the transfer fee
    if e8s_share <= DEFAULT_FEE.get_e8s() {
        let msg = format!("Total of {} is not enough to distribute among {} accounts", icp_balance, target_account_ids.len());
        ic_cdk::println!("{}", msg);
        return Err(msg);
    }
    let icp_share_after_fee = Tokens::from_e8s(e8s_share - DEFAULT_FEE.get_e8s());

    ic_cdk::println!("Distributing canister's account balance ({}) among {} accounts; each share after transfer fee: {}", icp_balance, target_account_ids.len(), icp_share_after_fee);

    // Transfer each share
    for id in &target_account_ids {
		// Decode the hex string into an AccountIdentifier
		let account_id = match AccountIdentifier::from_hex(&id) {
			Ok(aid) => aid,
			Err(e) => {
				let msg = format!("Failed to parse hex AccountIdentifier {}: {:?}", id, e);
				ic_cdk::println!("{}", msg);
				return Err(msg);
			}
		};

		let args = TransferArgs {
			memo: Memo(9911110010110811697),// 'codelta', converted to ascii and integer values concatenated
			amount: icp_share_after_fee,
			fee: DEFAULT_FEE,
			from_subaccount: None,// canister's default subaccount
			to: account_id.to_address(),
			created_at_time: None,// ledger will use current time
		};

        // Call the ledger's transfer method.
        let (result,): (Result<u64, TransferError>,) = ic_cdk::call(
            Principal::from_text(LEDGER_CANISTER_ID)
                .map_err(|e| format!("Invalid ledger principal: {:?}", e))?,
            "transfer",
            (args,),
        )
        .await
        .map_err(|(code, msg)| format!("Transfer of {} to {} failed: code={:?}, msg={}", icp_share_after_fee, account_id, code, msg))?;


        match result {
            Ok(block_index) => {
                ic_cdk::println!("Transfer of {} to {} succeeded in block index {}", icp_share_after_fee, account_id, block_index);
            },
            Err(error) => {
                return Err(format!("Transfer of {} to {} failed: code={:?}", icp_share_after_fee, account_id, error));
            }
        }
    }

    Ok(())
}
