use asset_address::LP_LEDGER_ADDRESS;
use ic_cdk::api::call::call;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk_macros::update;

use crate::constants::*;

#[derive(CandidType, Deserialize)]
struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize)]
struct TransferArg {
    from_subaccount: Option<Vec<u8>>,
    to: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: String },
}


pub type BlockIndex = Nat;

#[derive(CandidType, Deserialize)]
enum TransferResult {
    Ok(BlockIndex),
    Err(TransferError),
}
// #[derive(CandidType, Deserialize, Debug)]
#[update]
pub async fn icrc1_transfer(user_principal: Principal, amount: Nat) -> Result<BlockIndex, String> {
    let canister_id = Principal::from_text(LP_LEDGER_ADDRESS).expect("Invalid ledger canister ID");
    // let amount_nat = Nat::from(amount * 100000000);
    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: user_principal,
            subaccount: None,
        },
        amount: amount,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    let (result,): (TransferResult,) = call(
        canister_id,
        "icrc1_transfer",
        (args,)
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e))?;

    match result {
        TransferResult::Ok(block_index) => Ok(block_index),
        TransferResult::Err(err) => Err(format!("Transfer failed: {:?}", err)),
    }
}


#[update]
pub async fn faucet(ledger_canister: Principal, user_principal: Principal, amount: Nat) -> Result<Nat, String> {
    // Define the parameters for the ICRC2 transfer call
    // let amount_as_u64 = amount * 100000000;
    // let amount_nat = Nat::from(amount_as_u64);
    let args = TransferArg {
        from_subaccount: None,          // Optionally specify a subaccount if needed
        to: Account {
            owner: user_principal,      // The recipient of the transfer
            subaccount: None,
        },
        amount: amount,         // The amount of tokens to transfer
        fee: None,                      // Specify a fee if required
        memo: None,                     // Optional memo for the transfer
        created_at_time: None,          // Optional timestamp
    };

    // Make the call to the ICRC2 token canister with the transfer arguments
    let (result,): (TransferResult,) = call(
        ledger_canister,               // The canister ID of the token ledger (ICRC2)
        "icrc1_transfer",          // The method to call
        (args,)                    // Transfer arguments
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e))?;

    // Check if the call was successful
    match result {
        TransferResult::Ok(balance) => Ok(balance),
        TransferResult::Err(err) => Err(format!("Transfer failed: {:?}", err)),
    }
}



#[update]
pub async fn faucet(ledger_canister: Principal, user_principal: Principal, amount: u64) -> Result<Nat, String> {
    // Define the parameters for the ICRC2 transfer call
    let amount_as_u64 = amount * 100000000;
    let amount_nat = Nat::from(amount_as_u64);
    let args = TransferArg {
        from_subaccount: None,          // Optionally specify a subaccount if needed
        to: Account {
            owner: user_principal,      // The recipient of the transfer
            subaccount: None,
        },
        amount: amount_nat.clone(),         // The amount of tokens to transfer
        fee: None,                      // Specify a fee if required
        memo: None,                     // Optional memo for the transfer
        created_at_time: None,          // Optional timestamp
    };

    // Make the call to the ICRC2 token canister with the transfer arguments
    let (result,): (TransferResult,) = call(
        ledger_canister,               // The canister ID of the token ledger (ICRC2)
        "icrc1_transfer",          // The method to call
        (args,)                    // Transfer arguments
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e))?;

    // Check if the call was successful
    match result {
        TransferResult::Ok(balance) => Ok(balance),
        TransferResult::Err(err) => Err(format!("Transfer failed: {:?}", err)),
    }
}