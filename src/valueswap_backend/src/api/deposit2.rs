use crate::constants::asset_address::*;
use candid::{Nat, Principal};
use ic_cdk::call;
use crate::utils::types::*;
use ic_cdk::api;
use ic_cdk_macros::update;

/// Enum representing supported asset types.
enum AssetType {
    CkBTC,
    CkETH,
    // Add more assets here as needed.
}

impl AssetType {
    /// Returns the ledger canister ID associated with the asset type.
    fn ledger_canister_id(&self) -> Result<Principal, String> {
        match self {
            AssetType::CkBTC => Principal::from_text(CKBTC_LEDGER_ADDRESS).map_err(|e| e.to_string()),
            AssetType::CkETH => Principal::from_text(CKETH_LEDGER_ADDRESS).map_err(|e| e.to_string()),
        }
    }
}

/// Represents the data for a single token in a pool.
struct PoolTokenData {
    token_name: String,
    ckbtc_balance: u64,
    cketh_balance: u64,
    // Add more asset balances here if needed.
}

/// Represents the overall pool data.
struct Pool_Data {
    pool_data: Vec<PoolTokenData>,
}

/// Facilitates the transfer of tokens from a specified ledger canister.
///
/// # Arguments
///
/// * `asset` - The type of asset to transfer.
/// * `from` - The principal initiating the transfer.
/// * `to` - The target canister principal receiving the tokens.
/// * `amount` - The amount to transfer.
///
/// # Returns
///
/// * `Ok(Nat)` - The new balance after the transfer.
/// * `Err(String)` - An error message if the transfer fails.
pub async fn transfer_from_ledger(
    asset: AssetType,
    from: Principal,
    to: Principal,
    amount: Nat,
) -> Result<Nat, String> {
    let ledger_canister_id = asset.ledger_canister_id()?;

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
        .map_err(|(code, msg)| format!("Ledger call failed with code {}: {}", code, msg))?;

    match result {
        TransferFromResult::Ok(balance) => Ok(balance),
        TransferFromResult::Err(err) => Err(format!("Transfer error: {:?}", err)),
    }
}

/// Handles the deposit of tokens from the caller to a specified target canister.
///
/// # Arguments
///
/// * `asset` - The type of asset to deposit.
/// * `amount` - The amount of tokens to deposit.
/// * `target_canister_id` - The Principal ID of the target canister where tokens will be deposited.
///
/// # Returns
///
/// * `Ok(Nat)` - The new balance after the transfer.
/// * `Err(String)` - An error message if the transfer fails.
#[ic_cdk_macros::update]
pub async fn deposit_tokens(
    asset: AssetType,
    amount: u64,
    target_canister_id: Principal,
) -> Result<Nat, String> {
    // Validate amount
    if amount == 0 {
        return Err("Deposit amount must be greater than zero.".to_string());
    }

    // Parse ledger canister ID (handled in transfer_from_ledger)
    let user_principal = ic_cdk::api::caller();

    ic_cdk::println!(
        "{} canister principal: {}",
        match asset {
            AssetType::CkBTC => "ckBTC",
            AssetType::CkETH => "ckETH",
        },
        asset
            .ledger_canister_id()
            .unwrap_or_else(|_| Principal::anonymous())
    );

    ic_cdk::println!("Target canister principal for deposit: {}", target_canister_id);

    let amount_nat = Nat::from(amount);

    // Perform the transfer
    transfer_from_ledger(asset, user_principal, target_canister_id, amount_nat).await
}

/// Deposits multiple assets to a target canister.
///
/// # Arguments
///
/// * `assets` - A vector of tuples containing asset types and amounts.
/// * `target_canister_id` - The Principal ID of the target canister.
///
/// # Returns
///
/// * `Ok(())` - If all deposits are successful.
/// * `Err(String)` - If any deposit fails.
async fn deposit_multiple_assets(
    assets: Vec<(AssetType, u64)>,
    target_canister_id: Principal,
) -> Result<(), String> {
    for (asset, amount) in assets {
        deposit_tokens(asset, amount, target_canister_id)
            .await
            .map_err(|e| format!("Deposit failed for {:?}: {}", asset, e))?;
    }
    Ok(())
}