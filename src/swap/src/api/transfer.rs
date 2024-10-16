
use ic_cdk::api::call::call;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk_macros::update;

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
    fee: Option<u64>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize)]
enum TransferResult {
    Ok(Nat),
    Err(String),
}


#[update]
pub async fn icrc1_transfer(canister_id: Principal, user_principal: Principal, amount: u64) -> Result<Nat, String> {
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
        canister_id,               // The canister ID of the token ledger (ICRC2)
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