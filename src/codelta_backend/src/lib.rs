use candid::Principal;
use ic_cdk::api::management_canister::main::{
    canister_status, CanisterIdRecord, CanisterStatusResponse
};
use ic_cdk_macros::update;
use ic_cdk::caller;

const ALLOWED_CALLERS: &[&str] = &[
    "koiza-s6kz2-m45zq-4lrn7-4v65m-6zemu-neoxj-vz6cb-ouolw-rrawv-mae", // aligatorr89
    "zkkkd-i34qc-367ln-e2u7o-ezznu-dkfqh-gtfvz-cviph-6qa4v-evtfs-wqe", // Lorimer
    "hfbtd-e2vzk-rvwfx-c55l3-tsuue-oizir-hl4bg-tajby-skikk-iefse-fqe"  // MalithHatananchchige
];

fn only_allowed_callers() -> Result<(), String> {
    let current_caller = caller();
    for &allowed_str in ALLOWED_CALLERS {
        if let Ok(allowed_principal) = Principal::from_text(allowed_str) {
            if allowed_principal == current_caller {
                return Ok(());
            }
        }
    }
    Err(format!(
        "Caller {} is not in the allowed callers list: {:?}",
        current_caller,
        ALLOWED_CALLERS
    ))
}

#[update(guard = "only_allowed_callers")]
async fn check_status(canister_id: Principal) -> Result<CanisterStatusResponse, String> {
    let record = CanisterIdRecord { canister_id };
    let (status_response,): (CanisterStatusResponse,) = canister_status(record)
        .await
        .map_err(|(code, msg)| format!("Failed calling canister_status: code={:?}, msg={}", code, msg))?;
    Ok(status_response)
}

