// use crate::api::deposit::{transfer_from_ckbtc , transfer_from_cketh};
// use crate::constants::asset_address::*;
use crate::utils::types::*;
use candid::{Nat, Principal};
use ic_cdk::call;
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

// Function to handle deposits to dynamically created canisters
#[ic_cdk_macros::update]
pub async fn deposit_tokens(
    amount: Nat,
    ledger_canister_id: Principal,
    target_canister_id: Principal,
) -> Result<Nat, String> {
    if amount == Nat::from(0u32) {
        return Err("Deposit amount must be greater than zero.".to_string());
    }

    if ledger_canister_id == Principal::anonymous() {
        return Err("Invalid ledger canister ID: Cannot be anonymous.".to_string());
    }

    if target_canister_id == Principal::anonymous() {
        return Err("Invalid target canister ID: Cannot be anonymous.".to_string());
    }

    let user_principal = ic_cdk::api::caller();

    ic_cdk::println!(
        "Debug: Caller: {}, Amount: {}, Ledger Canister: {}, Target Canister: {}",
        user_principal,
        amount,
        ledger_canister_id,
        target_canister_id
    );

    match transfer_from(
        ledger_canister_id,
        user_principal,
        target_canister_id,
        amount,
    )
    .await
    {
        Ok(result) => {
            ic_cdk::println!("Debug: Transfer successful. Result: {}", result);
            Ok(result)
        }
        Err(err) => {
            ic_cdk::println!("Error: Transfer failed. Reason: {}", err);
            Err(format!("Failed to deposit tokens: {}", err))
        }
    }
}

pub async fn transfer_from(
    ledger_canister_id: Principal,
    from: Principal,
    to: Principal,
    amount: Nat,
) -> Result<Nat, String> {
    // Validate input amount
    if amount == Nat::from(0u32) {
        return Err("Transfer amount must be greater than zero.".to_string());
    }

    // Validate principals
    if ledger_canister_id == Principal::anonymous() {
        return Err("Invalid ledger canister ID: Cannot be anonymous.".to_string());
    }
    if from == Principal::anonymous() {
        return Err("Invalid sender principal: Cannot be anonymous.".to_string());
    }
    if to == Principal::anonymous() {
        return Err("Invalid recipient principal: Cannot be anonymous.".to_string());
    }

    // Debug: Log input arguments
    ic_cdk::println!(
        "Debug: Transfer from {} to {} of amount {} via ledger {}",
        from,
        to,
        amount,
        ledger_canister_id
    );

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

    // Debug: Log constructed arguments
    ic_cdk::println!("Debug: TransferFromArgs constructed: {:?}", args);

    let response = call(ledger_canister_id, "icrc2_transfer_from", (args,))
        .await
        .map_err(|e| format!("Call to icrc2_transfer_from failed: {:?}", e))?;

    let (result,): (TransferFromResult,) = response;

    // Debug: Log the result
    ic_cdk::println!("Debug: TransferFromResult: {:?}", result);

    match result {
        TransferFromResult::Ok(balance) => {
            ic_cdk::println!("Debug: Transfer successful. New balance: {}", balance);
            Ok(balance)
        }
        TransferFromResult::Err(err) => {
            ic_cdk::println!("Error: Transfer failed with error: {:?}", err);
            Err(format!("Transfer failed: {:?}", err))
        }
    }
}

// // to get exchange rates
#[ic_cdk::update]
pub async fn get_exchange_rate(
    base_asset: String,
    quote_asset: String,
) -> Result<(f64, u64), String> {
    // Validate inputs
    if base_asset.trim().is_empty() {
        return Err("Base asset symbol cannot be empty.".to_string());
    }
    if quote_asset.trim().is_empty() {
        return Err("Quote asset symbol cannot be empty.".to_string());
    }

    // Construct arguments
    let args = GetExchangeRateRequest {
        timestamp: None,
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: quote_asset.clone(),
        },
        base_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: base_asset.clone(),
        },
    };

    // Debug: Log the constructed arguments
    ic_cdk::println!("Debug: Requesting exchange rate with args: {:?}", args);

    // Make the inter-canister call
    let canister_id = match Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC) {
        Ok(id) => id,
        Err(_) => {
            return Err("Invalid CANISTER_ID_XRC in constants.".to_string());
        }
    };

    let res: Result<(GetExchangeRateResult,), (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::api::call::call_with_payment128(
            canister_id,
            "get_exchange_rate",
            (args,),
            10_000_000_000,
        )
        .await;

    // Process the response
    match res {
        Ok((result,)) => match result {
            GetExchangeRateResult::Ok(v) => {
                // Calculate the exchange rate
                let quote = v.rate;
                let pow = 10usize.pow(v.metadata.decimals);
                let exchange_rate = quote as f64 / pow as f64;
                let timestamp = ic_cdk::api::time();

                // Debug: Log the successful exchange rate
                ic_cdk::println!(
                    "Debug: Exchange rate for {}/{}: {} at timestamp {}",
                    base_asset,
                    quote_asset,
                    exchange_rate,
                    timestamp
                );

                Ok((exchange_rate, timestamp))
            }
            GetExchangeRateResult::Err(e) => {
                // Debug: Log the error response
                ic_cdk::println!("Error: Received error from XRC canister: {:?}", e);
                Err(format!("Exchange rate error: {:?}", e))
            }
        },
        Err((rejection_code, message)) => {
            // Debug: Log the rejection details
            ic_cdk::println!(
                "Error: Failed to call XRC canister. Rejection code: {:?}, Message: {}",
                rejection_code,
                message
            );
            Err(format!(
                "Could not get {}/{} rate. Rejection code: {:?}, Message: {}",
                base_asset, quote_asset, rejection_code, message
            ))
        }
    }
}

#[ic_cdk::update]
pub async fn get_exchange_rates1() -> Result<(f64, u64), String> {
    // Construct the request for the exchange rate
    let args = GetExchangeRateRequest {
        timestamp: None, // No specific timestamp, fetch latest rate
        quote_asset: Asset {
            class: AssetClass::Cryptocurrency,
            symbol: "ICP".to_string(),
        },
        base_asset: Asset {
            class: AssetClass::FiatCurrency,
            symbol: "USD".to_string(),
        },
    };

    // Debug: Log the constructed arguments
    ic_cdk::println!("Debug: Requesting exchange rate with args: {:?}", args);

    // Parse CANISTER_ID_XRC and handle invalid ID
    let canister_id = match Principal::from_text(crate::constants::asset_address::CANISTER_ID_XRC) {
        Ok(id) => id,
        Err(_) => {
            ic_cdk::println!("Error: Invalid CANISTER_ID_XRC in constants.");
            return Err("Invalid CANISTER_ID_XRC in constants.".to_string());
        }
    };

    // Perform the inter-canister call with payment
    let res: Result<(GetExchangeRateResult,), (ic_cdk::api::call::RejectionCode, String)> =
        ic_cdk::api::call::call_with_payment128(
            canister_id,
            "get_exchange_rate",
            (args,),
            10_000_000_000,
        )
        .await;

    // Process the response
    match res {
        // Handle successful response
        Ok((result,)) => match result {
            GetExchangeRateResult::Ok(v) => {
                // Calculate the exchange rate as f64
                let quote = v.rate;
                let pow = 10usize.pow(v.metadata.decimals);
                let rate = quote as f64 / pow as f64;

                // Capture the current time
                let time = ic_cdk::api::time();

                // Debug: Log the successful rate
                ic_cdk::println!("Debug: USD/ICP rate: {}, Timestamp: {}", rate, time);

                Ok((rate, time))
            }
            // Handle domain-specific errors from the XRC
            GetExchangeRateResult::Err(e) => {
                ic_cdk::println!("Error: Received error from XRC: {:?}", e);
                Err(format!("Error from XRC: {:?}", e))
            }
        },
        // Handle call rejection or failure
        Err((rejection_code, message)) => {
            ic_cdk::println!(
                "Error: Failed to call XRC canister. Rejection code: {:?}, Message: {}",
                rejection_code,
                message
            );
            Err(format!(
                "Could not get USD/ICP Rate. Rejection code: {:?}, Message: {}",
                rejection_code, message
            ))
        }
    }
}
