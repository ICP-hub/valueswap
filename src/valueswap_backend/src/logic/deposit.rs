// use crate::api::transfer::{transfer_from_ckbtc , transfer_from_cketh};
// use crate::constants::asset_address::*;
// use candid::{Nat, Principal};
// use crate::utils::types::*;

// // Function to handle deposits
// #[ic_cdk_macros::update]
// async fn deposit_ckbtc(amount: u64) -> Result<Nat, String> {
//     let ledger_canister_id =
//         Principal::from_text(CKBTC_LEDGER_ADDRESS).map_err(|e| e.to_string())?;

//     ic_cdk::println!("ckbtc canister principal {}", ledger_canister_id);
//     let user_principal = ic_cdk::api::caller();
//     let platform_principal =
//         Principal::from_text("bw4dl-smaaa-aaaaa-qaacq-cai").map_err(|e| e.to_string())?;
//     ic_cdk::println!("platform canister principal {}", platform_principal);

//     let amount_nat = Nat::from(amount);
//     transfer_from_ckbtc(
//         ledger_canister_id,
//         user_principal,
//         platform_principal,
//         amount_nat,
//     )
//     .await
// }

// // the function above is just an sample function, deposit function will use validation logic, reserve logic and other checks according to aave
