// use crate::api::deposit::{transfer_from_ckbtc , transfer_from_cketh};
// use crate::constants::asset_address::*;
use crate::utils::types::*;
use candid::{Nat, Principal};
use ic_cdk:: call;
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};



// Function to approve allowance
// #[update]
// #[ic_cdk_macros::update]
// pub async fn approve_allowance(
//     ledger_canister_id: Principal,
//     spender_canister_id: Principal,
//     amount: u64,
// ) -> Result<Nat, String> {
//     let amount_nat = Nat::from(amount * 100000000); // Convert to smallest unit (e.g., wei for ckETH)
//     let args = (
//         spender_canister_id,
//         amount_nat,
//     );

//     let (result,): (Result<Nat, String>,) =
//         call(ledger_canister_id, "icrc2_approve", args).await.map_err(|e| e.1)?;

//     result.map_err(|err| format!("Failed to approve allowance: {:?}", err))
// }

// Function to handle deposits
// Function to handle deposits to dynamically created canisters
#[ic_cdk_macros::update]
pub async fn deposit_tokens(amount: Nat, ledger_canister_id: Principal , target_canister_id: Principal) -> Result<Nat, String> {

    let user_principal = ic_cdk::api::caller();

    // Use the dynamically passed target canister principal
    let target_canister = target_canister_id;
    ic_cdk::println!("Target canister principal for deposit {}", target_canister);

//     ic_cdk::println!("amount{:}", amount.clone());
    // let amount_nat = Nat::from(amount * 100000000);
//     ic_cdk::println!("amount_nat{:}", amount_nat.clone());
    transfer_from(
        ledger_canister_id,
        user_principal,
        target_canister,
        amount,
    )
    .await
}

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



// to get exchange rates
#[ic_cdk::update]
pub async fn get_exchange_rate(
    base_asset_symbol: String, 
    quote_asset_symbol: String
) -> Result<(f64, u64), String> {

    let args = GetExchangeRateRequest {
        timestamp: None,
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: quote_asset_symbol.clone(),
        },
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: base_asset_symbol.clone(),
        },
    };

    let res: Result<(GetExchangeRateResult,), (ic_cdk::api::call::RejectionCode, String)> = 
        ic_cdk::api::call::call_with_payment128(
            Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC).unwrap(), 
            "get_exchange_rate", 
            (args,), 
            1_000_000_000
        ).await;

    match res {
        Ok(res_value) => {
            match res_value.0 {
                GetExchangeRateResult::Ok(v) => {
                    let quote = v.rate;
                    let pow = 10usize.pow(v.metadata.decimals);
                    let res = quote as f64 / pow as f64;
                    let time = ic_cdk::api::time();
                    Ok((res, time))
                },
                GetExchangeRateResult::Err(e) => {
                    Err(format!("ERROR :: {:?}", e))
                }
            }
        },
        Err(error) => {
            Err(format!("Could not get {}/{} Rate - {:?} - {}", base_asset_symbol, quote_asset_symbol, error.0, error.1))
        },
    }
}
