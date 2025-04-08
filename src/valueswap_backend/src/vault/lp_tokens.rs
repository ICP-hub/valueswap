use candid::{Nat, Principal};
use ic_cdk::api::management_canister::main::{delete_canister, stop_canister};
use ic_cdk::call;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::api::deposit::deposit_tokens;
use crate::api::transfer::icrc1_transfer;
use crate::constants::asset_address::LP_LEDGER_ADDRESS;
use crate::utils::types::*;
use crate::vault::apy::*;
use crate::with_state;

thread_local! {
    static TOTAL_LP_SUPPLY : RefCell<Nat> = RefCell::new(Nat::from(0u128));
    static POOL_LP_SHARE : RefCell<BTreeMap<String , Nat>> = RefCell::new(BTreeMap::new());
    static USERS_LP : RefCell<BTreeMap<Principal, Nat>> = RefCell::new(BTreeMap::new());
    pub static USERS_POOL : RefCell<BTreeMap<Principal , Vec<String>>> = RefCell::new(BTreeMap::new());
    pub static USERS_POOL_LP: RefCell<BTreeMap<Principal, BTreeMap<String, Nat>>> = RefCell::new(BTreeMap::new());
}

#[update]
pub fn increase_pool_lp_tokens(params: Pool_Data) -> Result<(), CustomError> {
    params.validate()?;

    let mut pool_supply: Nat = Nat::from(1u128);

    POOL_LP_SHARE.with(|lp_share| {
        let mut borrowed_lp_share = lp_share.borrow_mut();

        // this is the accumulation of what the user want to supply.
        pool_supply = params
            .pool_data
            .iter()
            .try_fold(Nat::from(0u128), |acc, pool| {
                let value = pool.value.clone();
                let balance = pool.balance.clone();
                if value == Nat::from(0u128) || balance == Nat::from(0u128) {
                    return Err(CustomError::InvalidInput(
                        "Pool value and balance must be greater than zero.".to_string(),
                    ));
                }
                // acc == accumulate.
                Ok(acc + (value * balance))
            })
            .unwrap_or_else(|err| {
                ic_cdk::println!("Error calculating pool supply: {:?}", err);
                Nat::from(0u128)
            });

        if pool_supply == Nat::from(0u128) {
            ic_cdk::println!("Warning: Pool supply is zero.");
            return;
        }

        // join we are using the represent the name of ratio liquidity pool.
        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        // in this we are modify the existing number of tokens with the updated ones.
        borrowed_lp_share
            .entry(key.clone())
            .and_modify(|existing_supply| {
                *existing_supply += pool_supply.clone() / Nat::from(1000u128);
                ic_cdk::println!("Updated LP share for pool {}: {}", key, *existing_supply);
            })
            .or_insert_with(|| {
                let new_supply = pool_supply.clone() / Nat::from(1000u128);
                ic_cdk::println!("Inserted new LP share for pool {}: {}", key, new_supply);
                new_supply
            });
    });

    USERS_POOL_LP.with(|users_pool_lp| {
        let user = ic_cdk::caller();
        let mut user_pools = users_pool_lp.borrow_mut();
        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        let lp_tokens: Nat = pool_supply / Nat::from(1000u128);

        // Insert or update the user's LP tokens for the specified pool.
        user_pools
            .entry(user)
            .or_insert_with(BTreeMap::new)
            .entry(key.clone())
            .and_modify(|existing_lp| {
                *existing_lp += lp_tokens.clone();
                ic_cdk::println!(
                    "Updated user {} LP tokens for pool {}: {}",
                    user,
                    key,
                    *existing_lp
                );
            })
            .or_insert_with(|| {
                ic_cdk::println!(
                    "Inserted new LP tokens for user {} in pool {}: {}",
                    user,
                    key,
                    lp_tokens
                );
                lp_tokens
            });
    });

    users_pool(params.clone())?;
    total_lp_tokens();

    Ok(())
}

#[update]
pub fn users_pool(params: Pool_Data) -> Result<(), CustomError> {
    // Validate input params
    params.validate()?;

    let user = ic_cdk::caller();

    USERS_POOL.with(|pool| {
        let mut pool = pool.borrow_mut();

        let new_pool: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        pool.entry(user)
            .and_modify(|user_pools| {
                if !user_pools.contains(&new_pool) {
                    ic_cdk::println!("Adding new pool for user: {}", user);
                    user_pools.push(new_pool.clone());
                }
            })
            .or_insert_with(|| {
                ic_cdk::println!("Creating new pool entry for user: {}", user);
                vec![new_pool]
            });
    });

    Ok(())
}

#[update]
pub fn remove_user_pool(params: Pool_Data) -> Result<(), CustomError> {
    // Validate input params
    params.validate()?;

    let user = ic_cdk::caller();

    USERS_POOL.with(|pool| {
        let mut pool = pool.borrow_mut();

        if let Some(user_pools) = pool.get_mut(&user) {
            let pool_to_remove: String = params
                .pool_data
                .iter()
                .map(|pool| pool.token_name.clone())
                .collect::<Vec<String>>()
                .join("");

            ic_cdk::println!("pool name remove user pool = {}",pool_to_remove);
            if let Some(index) = user_pools.iter().position(|p| p == &pool_to_remove) {
                ic_cdk::println!("Removing pool for user: {}", user);
                user_pools.remove(index);
            } else {
                ic_cdk::println!("Pool not found for user: {}", user);
            }

            // Remove entry if the user has no pools left
            if user_pools.is_empty() {
                ic_cdk::println!("No pools left, removing user entry: {}", user);
                pool.remove(&user);
            }
        } else {
            ic_cdk::println!("User has no pools to remove: {}", user);
        }
    });

    Ok(())
}


#[query]
pub fn get_users_pool(user: Principal) -> Result<Option<Vec<String>>, CustomError> {
    if user == Principal::anonymous() {
        return Err(CustomError::InvalidInput(
            "Anonymous principal is not allowed.".to_string(),
        ));
    }

    USERS_POOL.with(|pool| {
        let borrowed_pool = pool.borrow();
        match borrowed_pool.get(&user) {
            Some(pool_name) => {
                ic_cdk::println!("Found pool for user: {}", user);
                Ok(Some(pool_name.clone()))
            }
            None => {
                ic_cdk::println!("No pool found for user: {}", user);
                Ok(None)
            }
        }
    })
}

// To get all lp tokens

#[query]
fn total_lp_tokens() {
    let mut total_supply: Nat = Nat::from(0u128);

    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, Nat> = share.borrow().clone();

        if temp.is_empty() {
            ic_cdk::println!("Warning: POOL_LP_SHARE is empty.");
            return;
        }

        for (_key, value) in temp.iter() {
            total_supply = total_supply.clone() + value.clone();
        }

        if total_supply == Nat::from(0u128) {
            ic_cdk::println!("Warning: Total supply is zero after summing all shares.");
        }

        total_supply = total_supply.clone();
    });

    TOTAL_LP_SUPPLY.with(|lp_supply| {
        *lp_supply.borrow_mut() = total_supply.clone();
        ic_cdk::println!("Total LP supply updated: {}", total_supply);
    });
}

#[query]
fn get_total_lp() -> Nat {
    TOTAL_LP_SUPPLY.with(|total_lp| {
        let total = total_lp.borrow().clone();

        if total == Nat::from(0u128) {
            ic_cdk::println!("Warning: Total LP supply is zero.");
        }

        total
    })
}

// Query to get LP tokens for a specific pool
#[query]
pub fn get_pool_lp_tokens(pool_name: String) -> Nat {
    if pool_name.trim().is_empty() {
        ic_cdk::println!("Warning: Pool name is empty or whitespace.");
        return Nat::from(0u128);
    }

    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, Nat> = share.borrow().clone();

        match temp.get(&pool_name) {
            Some(key) => key.clone(),
            None => {
                ic_cdk::println!("Warning: No LP tokens found for pool '{}'.", pool_name);
                Nat::from(0u128)
            }
        }
    })
}

#[query]
pub fn get_user_pools_with_lp(user: Principal) -> Option<BTreeMap<String, Nat>> {
    if user == Principal::anonymous() {
        ic_cdk::println!("Warning: Anonymous principal is not allowed.");
        return None;
    }

    USERS_POOL_LP.with(|users_pool_lp| {
        let borrowed = users_pool_lp.borrow();
        match borrowed.get(&user) {
            Some(pools) => {
                ic_cdk::println!("Found pools for user: {}", user);
                Some(pools.clone())
            }
            None => {
                ic_cdk::println!("No pools found for user: {}", user);
                None
            }
        }
    })
}

#[update]
pub async fn users_lp_share(params: Pool_Data) -> Result<(), String> {
    let user = ic_cdk::caller();
    ic_cdk::println!("Starting LP share calculation for user: {}", user);

    params
        .validate()
        .map_err(|e| format!("Invalid pool data: {:?}", e))?;
    ic_cdk::println!("Pool data validated for user: {}", user);

    let mut users_contribution: Nat = Nat::from(1u128);
    ic_cdk::println!("Initial user contribution set to: {}", users_contribution);

    let total_pool_value = get_total_lp() * Nat::from(1000u128);
    let total_lp_supply = get_total_lp();

    if total_pool_value == Nat::from(0u128) || total_lp_supply == Nat::from(0u128) {
        return Err("Total pool value or total LP supply is zero.".to_string());
    }

    let amount = USERS_LP.with(|share| {
        let mut borrowed_share = share.borrow_mut();
        let mut users_contribution = Nat::from(0u128); // Assuming this was declared somewhere outside originally
    
        for amount in &params.pool_data {
            if amount.value == Nat::from(0u128) || amount.balance == Nat::from(0u128) {
                ic_cdk::println!("Skipping zero value or balance: value = {}, balance = {}", amount.value, amount.balance);
                continue;
            }
            let contribution = amount.value.clone() * amount.balance.clone();
            ic_cdk::println!(
                "Adding contribution: value = {}, balance = {}, contribution = {}",
                amount.value, amount.balance, contribution
            );
            users_contribution += contribution;
        }
    
        ic_cdk::println!("Total user contribution: {}", users_contribution);
        ic_cdk::println!("Total pool value: {}", total_pool_value);
        ic_cdk::println!("Total LP supply: {}", total_lp_supply);
    
        let lp_amount: Nat = (users_contribution.clone() / total_pool_value.clone()) * total_lp_supply.clone();
        ic_cdk::println!("Calculated LP amount to be minted for user {}: {}", user, lp_amount);
    
        borrowed_share.insert(user, lp_amount.clone());
    
        Some(lp_amount) // Return LP amount for later use
    });
    

    // If LP amount calculation failed, return an error
    let amount = amount.ok_or_else(|| {
        let err_msg = "Failed to calculate LP amount.".to_string();
        ic_cdk::println!("{}", err_msg);
        err_msg
    })?;


    ic_cdk::println!("Calculated LP token amount to assign to user: {}", amount);
    let mut attempts = 0;
    let max_retries = 2;

    while attempts < max_retries {
        match icrc1_transfer(user, amount.clone()).await {
            Ok(_) => {
                ic_cdk::println!("Transfer successful for user: {}", user);
                return Ok(());
            }
            Err(err) => {
                attempts += 1;
                ic_cdk::println!(
                    "Transfer attempt {}/{} failed for user: {}. Error: {:?}",
                    attempts,
                    max_retries,
                    user,
                    err
                );
                if attempts == max_retries {
                    let cycles = ic_cdk::api::canister_balance();
                    ic_cdk::println!("Current cycle balance: {}", cycles);

                    ic_cdk::println!("LP token transfer failed after {} retries.", attempts);
                    return Err("LP token transfer failed after 2 retries.".to_string());
                }
            }
        }
    }

    Err("Unexpected error occurred.".to_string())
}
// }

// #[update]
// pub async fn users_lp_share(params: Pool_Data) -> Result<(), String> {
//     let user = ic_cdk::caller();
//     ic_cdk::println!("Starting LP share calculation for user: {}", user);

//     params
//         .validate()
//         .map_err(|e| format!("Invalid pool data: {:?}", e))?;
//     ic_cdk::println!("Pool data validated for user: {}", user);

//     let mut users_contribution: Nat = Nat::from(1u128);
//     ic_cdk::println!("Initial user contribution set to: {}", users_contribution);

//     // Calculate user's contribution based on pool data
//     USERS_LP.with(|share| {
//         let mut borrowed_share = share.borrow_mut();

//         for amount in &params.pool_data {
//             if amount.value == Nat::from(0u128) || amount.balance == Nat::from(0u128) {
//                 log::warn!(
//                     "Skipping pool entry with zero value or balance for user: {}. Value: {:?}, Balance: {:?}",
//                     user,
//                     amount.value,
//                     amount.balance
//                 );
//                 continue;
//             }
//             users_contribution += amount.value.clone() * amount.balance.clone();
//             ic_cdk::println!(
//                 "User contribution updated: {} (added value: {:?} * balance: {:?})",
//                 users_contribution,
//                 amount.value,
//                 amount.balance
//             );
//         }

//         // Retrieve total pool value and total LP supply
//         let total_pool_value = get_total_lp() * Nat::from(1000u128);
//         let total_lp_supply = get_total_lp();

//         ic_cdk::println!(
//             "Total pool value: {}, Total LP supply: {}",
//             total_pool_value,
//             total_lp_supply
//         );

//         // Validation: Ensure the total pool value and total LP supply are not zero
//         if total_pool_value == Nat::from(0u128) || total_lp_supply == Nat::from(0u128) {
//             log::error!(
//                 "Total pool value or total LP supply is zero. Total pool value: {}, Total LP supply: {}",
//                 total_pool_value,
//                 total_lp_supply
//             );
//             return Err("Total pool value or total LP supply is zero.".to_string());
//         }

//         // Calculate the amount of LP tokens to assign to the user
//         let amount: Nat = (users_contribution / total_pool_value) * total_lp_supply;
//         ic_cdk::println!("Calculated LP token amount to assign to user: {}", amount);
//         borrowed_share.insert(user, amount.clone());

//         // Spawn an async task to transfer LP tokens with retry mechanism
//         ic_cdk::spawn(async move {
//             ic_cdk::println!("Starting transfer of LP tokens for user: {}", user);
//             let mut attempts = 0;
//             let max_retries = 2;

//             while attempts < max_retries {
//                 ic_cdk::println!("while function");
//                 let transfer_result = icrc1_transfer(user, amount.clone()).await;
//                 if transfer_result.is_ok() {
//                     ic_cdk::println!("Transfer successful for user: {}", user);
//                     break;
//                 } else {
//                     attempts += 1;
//                     ic_cdk::println!(
//                         "Transfer attempt {}/{} failed for user: {}. Error: {:?}",
//                         attempts,
//                         max_retries,
//                         user,
//                         transfer_result
//                     );
//                     log::warn!(
//                         "Transfer attempt {}/{} failed for user: {}. Retrying...",
//                         attempts,
//                         max_retries,
//                         user
//                     );

//                     if attempts == max_retries {
//                         // TODO: inter canister call.
//                         ic_cdk::println!(
//                             "Max retries reached for user: {}. Transfer failed.",
//                             user
//                         );
//                         log::error!(
//                             "Transfer failed after {} attempts for user: {}",
//                             attempts,
//                             user
//                         );
//                     }
//                 }
//             }
//         });

//         Ok(())
//     })?;

//     ic_cdk::println!("Completed LP share calculation for user: {}", user);
//     Ok(())
// }

#[query]
fn get_users_lp(user_id: Principal) -> Option<Nat> {
    if user_id == Principal::anonymous() {
        ic_cdk::println!("Warning: Anonymous principal is not allowed.");
        return None;
    }

    USERS_LP.with(|lp| {
        let borrowed_lp = lp.borrow();
        match borrowed_lp.get(&user_id) {
            Some(amount) => {
                ic_cdk::println!("Found LP tokens for user: {}", user_id);
                Some(amount.clone())
            }
            None => {
                ic_cdk::println!("No LP tokens found for user: {}", user_id);
                None
            }
        }
    })
}

// TODO Send token amount to pool canister instead of user_share ratio
#[update]
async fn burn_lp_tokens(
    params: Pool_Data,
    pool_name: String,
    amount: Nat,
    ledger_canister_id: Principal,
) -> Result<(), String> {
    params
        .validate()
        .map_err(|e| format!("Invalid pool data: {:?}", e))?;

    let base_scaling = Nat::from(10u128.pow(18)); // 10^18 for base calculations
    let weight_scaling = Nat::from(100u128); // Scale for percentages

    let user = ic_cdk::caller();

    if pool_name.trim().is_empty() {
        return Err("Pool name cannot be empty.".to_string());
    }

    if amount <= Nat::from(0u128) {
        return Err("Amount to burn must be greater than zero.".to_string());
    }

    let target_canister_id = ic_cdk::id();

    // Transfer tokens to the canister
    let result = deposit_tokens(amount.clone(), ledger_canister_id, target_canister_id).await;
    if let Err(e) = result {
        return Err(format!("Transfer failed: {}", e));
    }

    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    })
    .ok_or_else(|| format!("No canister ID found for the pool: {}", pool_name))?;

    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share
            .get(&pool_name)
            .cloned()
            .unwrap_or(Nat::from(0u128))
    });

    if pool_total_lp <= Nat::from(0u128) {
        return Err(format!("No LP tokens in the pool: {}", pool_name));
    }

    // Calculate user share ratio with proper scaling
    let user_share_ratio = (amount.clone() * base_scaling.clone()) / pool_total_lp.clone();

    let pool_value: Nat = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        borrowed_pool_lp
            .get(&pool_name)
            .map(|lp_value| lp_value.clone() * weight_scaling.clone())
            .unwrap_or(Nat::from(0u128))
    });

    if pool_value <= Nat::from(0u128) {
        return Err(format!("No tokens in the pool: {}", pool_name));
    }

    // Calculate tokens to transfer with proper scaling
    let tokens_to_transfer = (pool_value * user_share_ratio) / base_scaling.clone();

    let result: Result<(), String> = call(
        canister_id,
        "burn_tokens",
        (params, user, tokens_to_transfer),
    )
    .await
    .map_err(|e| format!("Failed to perform swap: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }

    // Update pool state
    decrease_pool_lp(pool_name.clone(), amount.clone());
    decrease_user_pool_lp(user, pool_name, amount.clone());
    decrease_total_lp(amount);

    ic_cdk::println!("Successfully burned LP tokens for user: {}", user);
    Ok(())
}

#[update]
#[candid::candid_method(update)]
async fn get_user_share_ratio(
    params: Pool_Data,
    pool_name: String,
    amount: Nat,
) -> Result<Vec<Nat>, String> {
    let user = ic_cdk::caller();

    if user == Principal::anonymous() {
        ic_cdk::println!("Error: Invalid user principal: Cannot be anonymous.");
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }
    ic_cdk::println!(
        "Input Params: {:?}, Pool Name: {}, Amount: {}",
        params,
        pool_name,
        amount
    );

    // Basic input validation
    if pool_name.trim().is_empty() {
        return Err("Pool name cannot be empty.".to_string());
    }

    if amount <= Nat::from(0u128) {
        return Err("Amount must be greater than zero.".to_string());
    }

    params
        .validate()
        .map_err(|e| format!("Invalid pool data: {:?}", e))?;

    // Get the total LP tokens for this pool
    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        let val = borrowed_share.get(&pool_name).cloned();
        ic_cdk::println!("pool_total_lp: {:?}", val);
        val.unwrap_or(Nat::from(0u128))
    });

    if pool_total_lp <= Nat::from(0u128) {
        return Err(format!("No LP tokens found for the pool: {}", pool_name));
    }

    // Get the canister ID for the pool
    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
        let val = pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal);
        ic_cdk::println!("canister_id: {:?}", val);
        val
    });

    let canister_id = match canister_id {
        Some(id) => id,
        None => return Err(format!("No canister ID found for the pool: {}", pool_name)),
    };

    let base_scaling = Nat::from(10u128.pow(18));

    let user_share_ratio = (amount.clone() * base_scaling.clone()) / pool_total_lp.clone();
    ic_cdk::println!("user_share_ratio: {:?}", user_share_ratio);

    let pool_data = params.pool_data.clone();

    let scaling_multiplier = Nat::from(1000u128);
    let pool_value = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();

        let val = borrowed_pool_lp
            .get(&pool_name)
            .map(|lp_value| lp_value.clone() * base_scaling.clone() * scaling_multiplier);

        ic_cdk::println!("pool_value: {:?}", val);
        val.unwrap_or(Nat::from(0u128))
    });

    if pool_value <= Nat::from(0u128) {
        return Err(format!("No tokens in the pool: {}", pool_name));
    }

    // Calculate tokens to transfer with proper scaling
    // This ensures the value sent to get_burned_tokens is sufficiently large
    let tokens_to_transfer = (pool_value.clone() * user_share_ratio.clone()) / base_scaling;
    ic_cdk::println!("tokens_to_transfer: {:?}", tokens_to_transfer);

    ic_cdk::println!(
        "DEBUG: amount = {}, pool_total_lp = {}",
        amount,
        pool_total_lp
    );
    ic_cdk::println!("DEBUG: user_share_ratio = {}", user_share_ratio);

    if tokens_to_transfer == Nat::from(0u128) {
        ic_cdk::println!("WARNING: tokens_to_transfer calculated as zero. Check scaling factors.");
    }

    let (tokens_vec,): (BurnedTokensResponse,) = call(
        canister_id,
        "get_burned_tokens",
        (params, user, tokens_to_transfer),
    )
    .await
    .map_err(|e| e.1)?;

    match tokens_vec {
        BurnedTokensResponse::Ok(balance) => {
            ic_cdk::println!("balance = {:?}", balance);
            Ok(balance)
        }
        BurnedTokensResponse::Err(err) => Err(format!("{:?}", err)),
    }
    // ic_cdk::println!("get_burned_tokens result: {:?}", response);
    // tokens_vec
    // result.map(|(response,)| response)
}

#[update]
pub fn decrease_pool_lp(pool_name: String, amount: Nat) {
    if pool_name.trim().is_empty() {
        ic_cdk::trap("Pool name cannot be empty.");
    }

    if amount <= Nat::from(0u128) {
        ic_cdk::trap("Amount to decrease must be greater than zero.");
    }

    POOL_LP_SHARE.with(|pool| {
        let mut pool_lp_share = pool.borrow_mut();

        match pool_lp_share.get_mut(&pool_name) {
            Some(current_lp) => {
                // Ensure the LP amount does not go negative
                if *current_lp >= amount {
                    *current_lp -= amount;
                    ic_cdk::println!(
                        "Decreased LP tokens in pool '{}': new balance = {}",
                        pool_name,
                        *current_lp
                    );
                } else {
                    ic_cdk::trap(&format!(
                        "Insufficient LP tokens in pool '{}'. Available: {}, Required: {}",
                        pool_name, *current_lp, amount
                    ));
                }
            }
            None => ic_cdk::trap(&format!("Pool not found: {}", pool_name)),
        }
    });
}

#[update]
pub fn decrease_user_pool_lp(user: Principal, pool_name: String, amount: Nat) {
    if pool_name.trim().is_empty() {
        ic_cdk::trap("Pool name cannot be empty.");
    }

    if amount <= Nat::from(0u128) {
        ic_cdk::trap("Amount to decrease must be greater than zero.");
    }

    if user == Principal::anonymous() {
        ic_cdk::trap("Anonymous users are not allowed.");
    }

    USERS_POOL_LP.with(|users_pool_lp| {
        let mut borrowed = users_pool_lp.borrow_mut();

        match borrowed.get_mut(&user) {
            Some(user_pools) => {
                match user_pools.get_mut(&pool_name) {
                    Some(current_lp) => {
                        if *current_lp >= amount {
                            *current_lp -= amount;
                            ic_cdk::println!(
                                "Decreased LP tokens for user '{}' in pool '{}': new balance = {}",
                                user, pool_name, *current_lp
                            );
                        } else {
                            ic_cdk::trap(&format!(
                                "Insufficient LP tokens for user '{}' in pool '{}'. Available: {}, Required: {}",
                                user, pool_name, *current_lp, amount
                            ));
                        }
                    }
                    None => ic_cdk::trap(&format!(
                        "User '{}' is not associated with pool '{}'",
                        user, pool_name
                    )),
                }
            }
            None => ic_cdk::trap(&format!("User '{}' not found", user)),
        }
    });
}

#[update]
pub fn decrease_total_lp(lp: Nat) {
    // Validation: LP amount must be greater than zero
    if lp <= Nat::from(0u128) {
        ic_cdk::trap("Amount to decrease must be greater than zero.");
    }

    TOTAL_LP_SUPPLY.with(|total_lp| {
        let mut borrowed_lp = total_lp.borrow_mut();

        if *borrowed_lp >= lp {
            *borrowed_lp -= lp;
        } else {
            ic_cdk::trap("Insufficient total LP tokens to decrease.");
        }
    });
}

#[update]
pub async fn remove_canister(canister_id: String) -> Result<(), String> {
    ic_cdk::println!("inside remove_canister");
    let canister_principal =
        Principal::from_text(canister_id.clone()).map_err(|_| "Invalid canister ID".to_string())?;

    // Stop the canister before deletion
    stop_canister(ic_cdk::api::management_canister::main::CanisterIdRecord {
        canister_id: canister_principal,
    })
    .await
    .map_err(|e| format!("Failed to stop canister: {:?}", e))?;

    // Delete the canister
    delete_canister(ic_cdk::api::management_canister::main::CanisterIdRecord {
        canister_id: canister_principal,
    })
    .await
    .map_err(|e| format!("Failed to delete canister: {:?}", e))?;

    ic_cdk::println!("Canister removed successfully.");

    Ok(())
}
