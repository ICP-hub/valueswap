// use crate::api::deposit::{transfer_from_ckbtc , transfer_from_cketh};
use crate::constants::asset_address::*;
use crate::utils::types::*;
use candid::{Nat, Principal};
use ic_cdk::{api::call::call_with_payment, call};
use ic_xrc_types::{Asset, AssetClass, GetExchangeRateRequest, GetExchangeRateResult};

// Function to handle deposits
// Function to handle deposits to dynamically created canisters
#[ic_cdk_macros::update]
pub async fn deposit_tokens(amount: u64, ledger_canister_id: Principal , target_canister_id: Principal) -> Result<Nat, String> {

    let user_principal = ic_cdk::api::caller();

    // Use the dynamically passed target canister principal
    let target_canister = target_canister_id;
    ic_cdk::println!("Target canister principal for deposit {}", target_canister);
    ic_cdk::println!("amount{:}", amount.clone());
    let amount_nat = Nat::from(amount * 100000000);
    ic_cdk::println!("amount_nat{:}", amount_nat.clone());
    transfer_from(
        ledger_canister_id,
        user_principal,
        target_canister,
        amount_nat,
    )
    .await
}


// to get exchange rates
#[ic_cdk::update]
pub async fn get_exchange_rates() -> Result<(f64, u64), String>  {
    // let x = GetExchangeRateRequest {

    // }

    let args = GetExchangeRateRequest {
        timestamp: None,
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "USDT".to_string(),
        },
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "ICP".to_string(),
        },
    };

    let res: Result<(GetExchangeRateResult,), (ic_cdk::api::call::RejectionCode, String)> 
    = ic_cdk::api::call::call_with_payment128(Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC).unwrap(), "get_exchange_rate", (args, ), 1_000_000_000).await;


    match res {
        Ok(res_value) => {
            match res_value.0 {
                GetExchangeRateResult::Ok(v) => {

                    let quote = v.rate;
                    let pow = 10usize.pow(v.metadata.decimals);
                    let res = quote as f64 / pow as f64;
                    let time = ic_cdk::api::time();
                    return Ok((res,time));
                    //return Ok((price, time));
                },
                GetExchangeRateResult::Err(e) => {
                    return Err(format!("ERROR :: {:?}", e));
                }
            }
        },
        Err(error) => {
      
            return Err(format!("Could not get USD/ICP Rate - {:?} - {}", error.0, error.1));
        },
    }
    // let result = call_with_payment(
    //     Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC).unwrap(),
    //     "get_exchange_rate",
    //     (payload,),
    //     1000000000,
    // )
    // .await.unwrap().0;

    // match result {
    //     GetExchangeRateResult::Ok(exchange_rate) => {
    //         Ok(exchange_rate.rate as f64 / 10u64.pow(exchange_rate.metadata.decimals as u32) as f64)
    //     },
    //     GetExchangeRateResult::Err(_) => {
    //         Err("failed ".to_string())
    //     }
    // }
// x

    // Ok("()".to_string())

//     let res_xrc =
//     call(Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC).unwrap(), "get_exchange_rate", (payload,)).await;
// ic_cdk::println!("{:?}", res_xrc);
// match res_xrc {
//     Ok((xr,)) => Ok(xr),
//     Err((code, msg)) => Err(format!(
//         "Error while calling XRC canister ({}): {:?}",
//     )),
// }
}

// Static caniste id for testing purpose only
// #[ic_cdk_macros::update]
// pub async fn deposit_ckbtc(amount: u64) -> Result<Nat, String> {
//     let ledger_canister_id =
//         Principal::from_text(CKBTC_LEDGER_ADDRESS).map_err(|e| e.to_string())?;

//     ic_cdk::println!("ckbtc canister principal {}", ledger_canister_id);
//     let user_principal = ic_cdk::api::caller();
//     let platform_principal =
//         Principal::from_text("bkyz2-fmaaa-aaaaa-qaaaq-cai").map_err(|e| e.to_string())?;
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
