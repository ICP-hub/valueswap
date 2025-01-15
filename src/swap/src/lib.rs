use candid::{ Nat, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod api;
mod utils;
pub use api::transfer::*;
pub use utils::maths::*;
pub use utils::types::*;
use crate::api::metadata::MetadataValue;


thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
    pub static POOL_BALANCE : RefCell<BTreeMap<Principal , Nat>> = RefCell::new(BTreeMap::new());
}



#[update]
pub fn pool_balance(user_principal: Principal, params: Pool_Data) {
    // Validate user principal
    if user_principal == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return;
    }

    // Validate pool data
    if params.pool_data.is_empty() {
        ic_cdk::println!("Error: Pool data is empty. Cannot compute balance.");
        return;
    }

    // Calculate the total balance
    let balance: Nat = params
        .pool_data
        .iter()
        .map(|pool| pool.balance.clone())
        .fold(Nat::from(0u128), |acc, x| acc + x);

    // Debug: Log the calculated balance
    ic_cdk::println!("Debug: Total calculated balance is {}.", balance);

    // Update the pool balance for the user
    POOL_BALANCE.with(|pool_balance| {
        let mut borrowed_pool_balance = pool_balance.borrow_mut();
        borrowed_pool_balance
            .entry(user_principal)
            .and_modify(|user_balance| *user_balance += balance.clone())
            .or_insert(balance);
    });
}


#[query]
pub fn get_pool_balance(user_principal: Principal) -> Option<Nat> {
    // Validate user principal
    if user_principal == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return None;
    }

    // Retrieve the pool balance
    POOL_BALANCE.with(|pool_balance| {
        let borrowed_pool_balance = pool_balance.borrow();
        if let Some(balance) = borrowed_pool_balance.get(&user_principal) {
            // Debug: Log the retrieved balance
            ic_cdk::println!(
                "Debug: Retrieved pool balance for user {}: {}.",
                user_principal,
                balance
            );
            Some(balance.clone())
        } else {
            // Debug: Log absence of balance
            ic_cdk::println!(
                "Debug: No pool balance found for user {}.",
                user_principal
            );
            None
        }
    })
}


// store user_id with pool data

#[update]
async fn store_pool_data(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    // Validate user principal
    if user_principal == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate pool data using the `validate` method
    if let Err(err) = params.validate() {
        ic_cdk::println!("Error: Invalid pool data - {:?}", err);
        return Err(format!("Invalid pool data: {:?}", err));
    }

    // Debug: Log storing operation
    ic_cdk::println!(
        "Debug: Storing pool data for user principal {} with {} pools.",
        user_principal,
        params.pool_data.len()
    );

    // Update the pool balance
    pool_balance(user_principal.clone(), params.clone());

    // Store the pool data
    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        pool_data_borrowed.entry(user_principal).or_default().push(liquidity);

        // Debug: Log successful storage
        ic_cdk::println!(
            "Debug: Successfully stored pool data for user principal {}.",
            user_principal
        );
    });

    Ok(())
}


// check if user exists and if it exists update pool data else add user entry with pool data

#[update]
async fn add_liquidity_to_pool(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    // Validate user principal
    if user_principal == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate pool data using the `validate` method
    if let Err(err) = params.validate() {
        ic_cdk::println!("Error: Invalid pool data - {:?}", err);
        return Err(format!("Invalid pool data: {:?}", err));
    }

    // Debug: Log adding liquidity operation
    let pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");
    ic_cdk::println!(
        "Debug: Adding liquidity to pool '{}' for user principal {}.",
        pool_name,
        user_principal
    );

    // Update the pool balance
    pool_balance(user_principal.clone(), params.clone());

    // Add the liquidity to the pool data
    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        // Check if user already exists
        if let Some(existing_pool_data) = pool_data_borrowed.get_mut(&user_principal) {
            ic_cdk::println!(
                "Debug: Found existing pool data for user {}. Appending new liquidity.",
                user_principal
            );
            existing_pool_data.push(liquidity);
        } else {
            ic_cdk::println!(
                "Debug: No existing pool data found for user {}. Creating new entry.",
                user_principal
            );
            pool_data_borrowed
                .entry(user_principal)
                .or_default()
                .push(liquidity);
        }
    });

    // Debug: Log successful operation
    ic_cdk::println!(
        "Debug: Successfully added liquidity to pool '{}' for user principal {}.",
        pool_name,
        user_principal
    );

    Ok(())
}



#[update]
pub async fn lp_rollback(user: Principal, pool_data: Pool_Data) -> Result<(), String> {
    // Validate user principal
    if user == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate pool data using the `validate` method
    if let Err(err) = pool_data.validate() {
        ic_cdk::println!("Error: Invalid pool data - {:?}", err);
        return Err(format!("Invalid pool data: {:?}", err));
    }

    // Debug: Log rollback operation
    ic_cdk::println!(
        "Debug: Starting LP rollback for user principal {} with {} pools.",
        user,
        pool_data.pool_data.len()
    );

    // Perform rollback for each pool
    for amount in pool_data.pool_data.iter() {
        ic_cdk::println!(
            "Debug: Rolling back balance {} of token {} for user {}.",
            amount.balance,
            amount.token_name,
            user
        );

        let transfer_result =
            icrc1_transfer(amount.ledger_canister_id, user, amount.balance.clone()).await;

        if let Err(e) = transfer_result {
            let error_message = format!(
                "Rollback failed for user {} on token {}: {}",
                user, amount.token_name, e
            );
            ic_cdk::println!("Error: {}", error_message);
            ic_cdk::trap(&error_message);
        }
    }

    // Debug: Log successful completion
    ic_cdk::println!("Debug: LP rollback completed successfully for user principal {}.", user);

    Ok(())
}




// TODO : make ledger calls with state checks for balance to prevent TOCTOU vulnerablities

#[update]
async fn burn_tokens(
    params: Pool_Data,
    user: Principal,
    tokens_to_transfer: f64,
) -> Result<(), String> {
    // Validate user principal
    if user == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate pool data using the `validate` method
    if let Err(err) = params.validate() {
        ic_cdk::println!("Error: Invalid pool data - {:?}", err);
        return Err(format!("Invalid pool data: {:?}", err));
    }

    let tokens_to_transfer_u64 = tokens_to_transfer as u64;
    let tokens_to_transfer_nat = Nat::from(tokens_to_transfer_u64);

    // Debug: Log the burn operation start
    ic_cdk::println!(
        "Debug: Starting burn_tokens for user principal {} with share ratio {}.",
        user,
        tokens_to_transfer
    );


    // TODO Add check for balance if its greater than canister balance

    // Get total token balance for the user
    // let total_token_balance = match get_pool_balance(user.clone()) {
    //     Some(balance) => balance,
    //     None => {
    //         ic_cdk::println!(
    //             "Debug: No pool balance found for user principal {}. Defaulting to zero.",
    //             user
    //         );
    //         Nat::from(0u128)
    //     }
    // };

    // Debug: Log total token balance
    // ic_cdk::println!(
    //     "Debug: Total token balance for user principal {} is {}.",
    //     user,
    //     total_token_balance
    // );

    // Iterate over pool data to calculate and burn tokens
    for token in params.pool_data.iter() {
        // Calculate token amount to burn
        let mut token_amount = token.weight.clone() * tokens_to_transfer_nat.clone();
        // user_share_ratio = user_share_ratio * 10;
        // let user_share_ratio_u64 = user_share_ratio as u64;
        // let ratio = Nat::from(user_share_ratio_u64);
        // let mut transfer_amount = token_amount.clone() * ratio;
        // transfer_amount = transfer_amount / Nat::from(10u128);

        token_amount = token_amount * Nat::from(10u128);

        // Debug: Log transfer details
        ic_cdk::println!(
            "Debug: Burning {} of token {} for user principal {}.",
            token_amount,
            token.token_name,
            user
        );

        // Perform the token transfer
        let transfer_result = icrc1_transfer(token.ledger_canister_id, user, token_amount).await;

        if let Err(e) = transfer_result {
            let error_message = format!(
                "Error: Token transfer failed for token {} for user {}: {}",
                token.token_name, user, e
            );
            ic_cdk::println!("{}", error_message);
            return Err(error_message);
        }
    }

    // Debug: Log successful burn operation
    ic_cdk::println!("Debug: burn_tokens completed successfully for user principal {}.", user);

    Ok(())
}



#[query]
async fn get_burned_tokens(
    params: Pool_Data,
    user: Principal,
    tokens_to_transfer: f64,
) -> Result<Vec<Nat>, String> {
    // Validate user principal
    if user == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate pool data using the `validate` method
    if let Err(err) = params.validate() {
        ic_cdk::println!("Error: Invalid pool data - {:?}", err);
        return Err(format!("Invalid pool data: {:?}", err));
    }

    // Validate user_share_ratio
    // if user_share_ratio <= 0.0 {
    //     ic_cdk::println!("Error: Invalid user share ratio. Must be greater than zero.");
    //     return Err("User share ratio must be greater than zero.".to_string());
    // }
    
    let tokens_transfer_u64 = tokens_to_transfer as u64;
    let tokens_to_transfer_nat = Nat::from(tokens_transfer_u64);
        
    
    // Debug: Log the start of the function
    ic_cdk::println!(
        "Debug: Calculating burned tokens for user {} with share ratio {}.",
        user,
        tokens_to_transfer_nat
    );



    // Get the total token balance for the user
    // let total_token_balance = match get_pool_balance(user.clone()) {
    //     Some(balance) => {
    //         ic_cdk::println!(
    //             "Debug: Total token balance for user {} is {}.",
    //             user,
    //             balance
    //         );
    //         balance
    //     }
    //     None => {
    //         ic_cdk::println!(
    //             "Debug: No pool balance found for user {}. Defaulting to zero.",
    //             user
    //         );
    //         Nat::from(0u128)
    //     }
    // };

    let mut result: Vec<Nat> = vec![];

    // Iterate over each token in pool data
    for token in params.pool_data.iter() {
        // Calculate token amount and transfer amount
        let token_amount = token.weight.clone() * tokens_to_transfer_nat.clone();
        // let transfer_amount = token_amount
        //     .to_string()
        //     .parse::<f64>()
        //     .unwrap_or_default()
        //     / tokens_to_transfer_nat;

        // Debug: Log the calculated transfer amount
        ic_cdk::println!(
            "Debug: Calculated transfer amount for token {}: {}.",
            token.token_name,
            tokens_to_transfer_nat
        );

        result.push(token_amount.clone());
    }

    // Debug: Log the result
    ic_cdk::println!(
        "Debug: Burned token calculation completed for user {}. Result: {:?}",
        user,
        result
    );

    Ok(result)
}



#[update]
async fn swap(user_principal: Principal, params: SwapParams, amount: Nat) -> Result<(), String> {
    // Validate user principal
    if user_principal == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate SwapParams (you can extend this validation based on your use case)
    if params.token1_name.trim().is_empty() || params.token2_name.trim().is_empty() {
        ic_cdk::println!("Error: Token names cannot be empty.");
        return Err("Invalid SwapParams: Token names cannot be empty.".to_string());
    }

    if amount == Nat::from(0u32) {
        ic_cdk::println!("Error: Transfer amount must be greater than zero.");
        return Err("Transfer amount must be greater than zero.".to_string());
    }

    // Debug: Log the swap operation start
    ic_cdk::println!(
        "Debug: Starting swap for user principal {} with amount {}.",
        user_principal,
        amount
    );

    // Perform the token transfer
    let transfer_result = icrc1_transfer(params.ledger_canister_id2, user_principal, amount.clone()).await;

    if let Err(e) = transfer_result {
        let error_message = format!("Token transfer failed for user {}: {:?}", user_principal, e);
        ic_cdk::println!("Error: {}", error_message);
        return Err(error_message);
    }

    // Debug: Log successful transfer
    ic_cdk::println!(
        "Debug: Token transfer completed successfully for user principal {}.",
        user_principal
    );

    // Fetch and clone the pool data
    let mut pool_data = POOL_DATA.with(|pool_data| pool_data.borrow_mut().clone());

    // Validate pool data
    if pool_data.is_empty() {
        ic_cdk::println!("Error: No pool data found for swap operation.");
        return Err("No pool data found.".to_string());
    }

    // Debug: Log pool data processing
    ic_cdk::println!("Debug: Processing pool data for swap operation.");

    // Initialize distributed amount and check flags
    let length = pool_data.len();
    let distribute_amount1 = params.token_amount / length;
    let distribute_amount2 = amount / length;
    let mut has_sufficient_balance = false;
    let mut has_sufficient_liquidity = false;

    // Iterate over pool data and perform the swap
    for (user, pools) in pool_data.iter_mut() {
        for pool in pools {
            for token in &mut pool.pool_data {
                if token.token_name == params.token1_name {
                    // Ensure sufficient balance
                    if token.balance < distribute_amount1.clone() {
                        return Err(format!(
                            "User {:?} has insufficient balance for token {}",
                            user, token.token_name
                        ));
                    }

                    // Subtract distributed amount from token
                    token.balance -= distribute_amount1.clone();
                    has_sufficient_balance = true;
                }

                if token.token_name == params.token2_name {
                    token.balance += distribute_amount2.clone();
                    has_sufficient_liquidity = true;
                }
            }
        }
    }

    // Check balance and liquidity flags
    if !has_sufficient_balance {
        ic_cdk::println!("Error: Insufficient balance for swap operation.");
        return Err("Insufficient balance.".to_string());
    }

    if !has_sufficient_liquidity {
        ic_cdk::println!("Error: Insufficient liquidity for swap operation.");
        return Err("Insufficient liquidity.".to_string());
    }

    // Debug: Log successful validation
    ic_cdk::println!("Debug: Sufficient balance and liquidity verified.");

    // Update the pool data
    POOL_DATA.with(|pool_data_cell| {
        let mut pool_data_mut = pool_data_cell.borrow_mut();
        *pool_data_mut = pool_data;
    });

    // Debug: Log successful swap completion
    ic_cdk::println!(
        "Debug: Swap operation completed successfully for user principal {}.",
        user_principal
    );

    Ok(())
}



export_candid!();
