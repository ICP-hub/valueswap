// use crate::api::deposit::{transfer_from_ckbtc , transfer_from_cketh};
use crate::constants::asset_address::*;
use candid::{Nat, Principal};
use ic_cdk::call;
use crate::utils::types::*;

// Function to handle deposits
// Function to handle deposits to dynamically created canisters
#[ic_cdk_macros::update]
pub async fn deposit_tokens(amount: u64, ledger_canister_id: Principal , target_canister_id: Principal) -> Result<Nat, String> {
    // let ledger_canister_id =
        // Principal::from_text(CKBTC_LEDGER_ADDRESS).map_err(|e| e.to_string())?;

    // ic_cdk::println!("ckbtc canister principal {}", ledger_canister_id);
    let user_principal = ic_cdk::api::caller();
    
    // Use the dynamically passed target canister principal
    let target_canister = target_canister_id;
    ic_cdk::println!("Target canister principal for deposit {}", target_canister);

    let amount_nat = Nat::from(amount);
    transfer_from(
        ledger_canister_id,
        user_principal,
        target_canister,
        amount_nat,
    )
    .await
}


// the function above is just an sample function, deposit function will use validation logic, reserve logic and other checks according to aave
pub async fn transfer_from(
    ledger_canister_id: Principal,
    from: Principal,
    to: Principal,
    amount: Nat,
) -> Result<Nat, String> {
    let args = TransferFromArgs {
        to: TransferAccount {
            owner: to,
            subaccount: None,
        },
        fee: None,
        spender_subaccount: None,
        from: TransferAccount {
            owner: from,
            subaccount: None,
        },
        memo: None,
        created_at_time: None,
        amount,
    };
    let (result,): (TransferFromResult,) = call(ledger_canister_id, "icrc2_transfer_from", (args,))
        .await
        .map_err(|e| e.1)?;

    match result {
        TransferFromResult::Ok(balance) => Ok(balance),
        TransferFromResult::Err(err) => Err(format!("{:?}", err)),
    }
}
