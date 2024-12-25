// use candid::{Nat, Principal};
// use ic_cdk::call;
// use crate::utils::types::*;

// #[ic_cdk_macros::update]
// pub async fn transfer_tokens_to_user(amount: u64, target_canister_id: Principal) -> Result<Nat, String> {
//     let ledger_canister_id =
//         Principal::from_text("jhbhbiuhbiuhiubhiuy").map_err(|e| e.to_string())?;

//     ic_cdk::println!("ckbtc canister principal {}", ledger_canister_id);
//     let user_principal = ic_cdk::api::caller();
    
//     // Use the dynamically passed target canister principal
//     let platform_principal = target_canister_id;
//     ic_cdk::println!("Target canister principal for deposit {}", platform_principal);

//     let amount_nat = Nat::from(amount);
//     transfer(
//         ledger_canister_id,
//         user_principal,
//         platform_principal,
//         amount_nat,
//     )
//     .await
// }

// pub async fn transfer(
//     ledger_canister_id: Principal,
//     from: Principal,
//     to: Principal,
//     amount: Nat,
// ) -> Result<Nat, String> {
//     let args = TransferFromArgs {
//         to: TransferAccount {
//             owner: to,
//             subaccount: None,
//         },
//         fee: None,
//         spender_subaccount: None,
//         from: TransferAccount {
//             owner: from,
//             subaccount: None,
//         },
//         memo: None,
//         created_at_time: None,
//         amount,
//     };
//     let (result,): (TransferFromResult,) = call(ledger_canister_id, "icrc2_transfer_from", (args,))
//         .await
//         .map_err(|e| e.1)?;

//     match result {
//         TransferFromResult::Ok(balance) => Ok(balance),
//         TransferFromResult::Err(err) => Err(format!("{:?}", err)),
//     }
// }


