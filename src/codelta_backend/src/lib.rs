use candid::{Nat, Principal};
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse,
};
use ic_cdk_macros::update;
use ic_ledger_types::{
    account_balance, AccountBalanceArgs, AccountIdentifier, Memo, Subaccount, Tokens, TransferArgs,
    TransferError,
};

use crate::{member::MEMBERS, team::get_team_by_topic, topic::Topic};

pub mod member;
pub mod team;
pub mod topic;

const DEFAULT_FEE: Tokens = Tokens::from_e8s(10_000); // 0.0001 ICP

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
    let principals = MEMBERS.into_iter().map(|m| m.principal).collect::<Vec<_>>();
    contains_caller(&principals)
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

async fn _check_balance(sub_account: Subaccount) -> Result<Tokens, String> {
    // Instantiate client for the ICP Ledger (ryjl3-tyaaa-aaaaa-aaaba-cai)
    let ledger_canister_id = Principal::from_text(LEDGER_CANISTER_ID).unwrap();
    // Canister account args
    let account_balance_args = AccountBalanceArgs {
        account: AccountIdentifier::new(&ic_cdk::id(), &sub_account),
    };
    // Get the canister's current ICP balance
    match account_balance(ledger_canister_id, account_balance_args).await {
        Ok(balance) => Ok(balance),
        Err((_code, err_msg)) => {
            let msg = format!("Failed to get canister balance: {}", err_msg);
            ic_cdk::println!("{}", msg);
            return Err(msg);
        }
    }
}

#[update(guard = "only_members_guard")]
async fn check_balance(topic: Topic) -> Result<String, String> {
    let team = get_team_by_topic(topic);
    let team_sub_account = Subaccount(team.sub_account.clone());
    Ok(_check_balance(team_sub_account).await.unwrap().to_string())
}

#[update(guard = "only_members_guard")]
async fn check_subaccount_hex(topic: Topic) -> Result<String, String> {
    let team = get_team_by_topic(topic);
    let team_sub_account = Subaccount(team.sub_account.clone());
    let team_subaccount_hex = AccountIdentifier::new(&ic_cdk::id(), &team_sub_account).to_hex();
    Ok(team_subaccount_hex)
}

/// ## Arguments
/// * `topic` - Proposal topic which is unique per team
/// * `account_ids` - Optional (useful in case you want to exclude some team accounts - they might have been on vacation)
#[update(guard = "only_threshold_guard")]
async fn distribute_icp(topic: Topic, account_ids: Option<Vec<String>>) -> Result<(), String> {
    let team = get_team_by_topic(topic);
    let team_account_ids = team.members.iter().map(|m| m.account);

    // Pick all team members accounts and optionaly filter them by provided accounts
    let target_account_ids = team_account_ids
        .filter(|account_id| {
            if let Some(account_ids) = &account_ids {
                return account_ids.contains(&account_id.to_string());
            }
            return true;
        })
        .collect::<Vec<_>>();

    // Get short form of ids for logging
    let truncated_ids: Vec<String> = target_account_ids.iter()
        .map(|id| {
            if id.len() > 7 { format!("{}...", &id[..7]) } else { id.to_string() }
        })
        .collect();

    ic_cdk::println!("Call to distribute_icp({}) initiated by threshold consensus", truncated_ids.join(", "));

    // Bail out if no target account IDs provided
    if target_account_ids.is_empty() {
        let msg = "No target account IDs provided.".to_string();
        ic_cdk::println!("{}", msg);
        return Err(msg);
    }

    let team_sub_account = Subaccount(team.sub_account.clone());
    let icp_balance: Tokens = _check_balance(team_sub_account).await?;

    // Calculate how much each account should receive
    let num_accounts = Nat::from(target_account_ids.len() as u64);
    // Perform the division and safely convert to u64
    let e8s_share: u64 = (icp_balance.e8s().clone() / num_accounts)
        .0.try_into()
        .map_err(|_| "Conversion error: Value exceeds u64 limit".to_string())?;

    // Bail out if the share is not above the transfer fee
    if e8s_share <= DEFAULT_FEE.e8s() {
        let msg = format!("Total of {} is not enough to distribute among {} accounts", icp_balance, target_account_ids.len());
        ic_cdk::println!("{}", msg);
        return Err(msg);
    }
    let icp_share_after_fee = Tokens::from_e8s(e8s_share - DEFAULT_FEE.e8s());

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
			from_subaccount: Some(team_sub_account),// Team's subaccount
			to: account_id,
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
