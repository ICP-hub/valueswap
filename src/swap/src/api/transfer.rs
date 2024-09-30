use ic_cdk::api::call::call;
use candid::{Nat, Principal};
use ic_cdk_macros::update;

#[update]
pub async fn icrc1_transfer(canister_id: Principal, user_principal: Principal, amount: Nat) -> Result<(), String> {
    // Define the parameters for the ICRC1 transfer call
    let args = (
        user_principal,  // The recipient of the transfer
        amount.clone(),          // The amount of tokens to transfer
    );

    // Make the call to the ICRC1 token canister
    let result: Result<(Nat,), String> = call(
        canister_id,     // The canister ID of the token ledger (ICRC1)
        "icrc1_transfer", // The method to call
        (user_principal, amount), // The transfer arguments
    )
    .await
    .map_err(|e| format!("Transfer failed: {:?}", e));

    // Check if the call was successful
    match result {
        Ok(_) => Ok(()),
        Err(e) => Err(e),
    }
}
