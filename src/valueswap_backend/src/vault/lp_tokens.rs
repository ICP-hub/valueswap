use candid::{Nat, Principal};
use ic_cdk::call;
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
use std::collections::BTreeMap;

use crate::api::deposit::deposit_tokens;
use crate::api::transfer::icrc1_transfer;
use crate::constants::asset_address::LP_LEDGER_ADDRESS;
use crate::utils::types::*;
use crate::with_state;

thread_local! {
    static TOTAL_LP_SUPPLY : RefCell<Nat> = RefCell::new(Nat::from(0u128));
    static POOL_LP_SHARE : RefCell<BTreeMap<String , Nat>> = RefCell::new(BTreeMap::new());
    static USERS_LP : RefCell<BTreeMap<Principal, Nat>> = RefCell::new(BTreeMap::new());
    static USERS_POOL : RefCell<BTreeMap<Principal , Vec<String>>> = RefCell::new(BTreeMap::new());
    static USERS_POOL_LP: RefCell<BTreeMap<Principal, BTreeMap<String, Nat>>> = RefCell::new(BTreeMap::new());
}

#[update]
pub fn increase_pool_lp_tokens(params: Pool_Data) -> Result<(), CustomError> {
    params.validate()?;

    let mut pool_supply: Nat = Nat::from(1u128);

    POOL_LP_SHARE.with(|lp_share| {
        let mut borrowed_lp_share = lp_share.borrow_mut();

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

        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        borrowed_lp_share
            .entry(key.clone())
            .and_modify(|existing_supply| {
                *existing_supply += pool_supply.clone() / Nat::from(1000u128);
                ic_cdk::println!(
                    "Updated LP share for pool {}: {}",
                    key,
                    *existing_supply
                );
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


#[query]
fn get_users_pool(user: Principal) -> Result<Option<Vec<String>>, CustomError> {
    if user == Principal::anonymous() {
        return Err(CustomError::InvalidInput("Anonymous principal is not allowed.".to_string()));
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

        total_supply = total_supply.clone() / Nat::from(1000u128);
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

    params.validate().map_err(|e| format!("Invalid pool data: {:?}", e))?;

    let mut users_contribution: Nat = Nat::from(1u128);

    // Calculate user's contribution based on pool data
    USERS_LP.with(|share| {
        let mut borrowed_share = share.borrow_mut();

        for amount in &params.pool_data {
            if amount.value == Nat::from(0u128) || amount.balance == Nat::from(0u128) {
                log::warn!(
                    "Skipping pool entry with zero value or balance for user: {}",
                    user
                );
                continue;
            }
            users_contribution += amount.value.clone() * amount.balance.clone();
        }

        // Retrieve total pool value and total LP supply
        let total_pool_value = get_total_lp() * Nat::from(1000u128);
        let total_lp_supply = get_total_lp();

        // Validation: Ensure the total pool value and total LP supply are not zero
        if total_pool_value == Nat::from(0u128) || total_lp_supply == Nat::from(0u128) {
            log::error!("Total pool value or total LP supply is zero. Cannot calculate LP share.");
            return Err("Total pool value or total LP supply is zero.".to_string());
        }

        // Calculate the amount of LP tokens to assign to the user
        let amount: Nat = (users_contribution / total_pool_value) * total_lp_supply;
        borrowed_share.insert(user, amount.clone());

        // Spawn an async task to transfer LP tokens with retry mechanism
        ic_cdk::spawn(async move {
            let mut attempts = 0;
            let max_retries = 2;

            while attempts < max_retries {
                let transfer_result = icrc1_transfer(user, amount.clone()).await;
                if transfer_result.is_ok() {
                    log::info!("Transfer successful for user: {}", user);
                    break;
                } else {
                    attempts += 1;
                    log::warn!(
                        "Transfer attempt {}/{} failed for user: {}",
                        attempts,
                        max_retries,
                        user
                    );

                    if attempts == max_retries {
                        log::error!("Transfer failed after {} attempts for user: {}", attempts, user);
                    }
                }
            }
        });

        Ok(())
    })?;

    Ok(())
}




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
async fn burn_lp_tokens(params: Pool_Data, pool_name: String, amount: Nat) -> Result<(), String> {
    params.validate().map_err(|e| format!("Invalid pool data: {:?}", e))?;

    let user = ic_cdk::caller();

    if pool_name.trim().is_empty() {
        return Err("Pool name cannot be empty.".to_string());
    }

    if amount <= Nat::from(0u128) {
        return Err("Amount to burn must be greater than zero.".to_string());
    }

    let ledger_canister_id =
        Principal::from_text(LP_LEDGER_ADDRESS).expect("Invalid ledger canister ID");
    let target_canister_id = ic_cdk::id();

    let result = deposit_tokens(amount.clone(), ledger_canister_id, target_canister_id).await;
    if let Err(e) = result {
        return Err(format!("Transfer failed: {}", e));
    }

    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
        pool_borrowed.get(&pool_name).map(|user_principal| user_principal.principal)
    });

    let canister_id = match canister_id {
        Some(id) => id,
        None => {
            ic_cdk::println!("No canister ID found for pool: {}", pool_name);
            return Err("No canister ID found for the pool.".to_string());
        }
    };

    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share.get(&pool_name).cloned().unwrap_or(Nat::from(0u128))
    });

    if pool_total_lp <= Nat::from(0u128) {
        return Err(format!("No LP tokens in the pool: {}", pool_name));
    }

    let user_share_ratio = match amount.clone().to_string().parse::<f64>() {
        Ok(amount_f64) => match pool_total_lp.to_string().parse::<f64>() {
            Ok(total_f64) => amount_f64 / total_f64,
            Err(_) => return Err("Failed to parse pool_total_lp to f64.".to_string()),
        },
        Err(_) => return Err("Failed to parse amount to f64.".to_string()),
    };

    let pool_value: Nat = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        if let Some(lp_value) = borrowed_pool_lp.get(&pool_name) {
            lp_value.clone() * Nat::from(1000u128)
        } else {
            Nat::from(0u128)
        }
    });

    if pool_value <= Nat::from(0u128) {
        return Err(format!("No tokens in the pool: {}", pool_name));
    }

    let pool_value_f64 = match pool_value.to_string().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return Err("Failed to parse pool_value to f64.".to_string()),
    };

    let tokens_to_transfer = pool_value_f64 * user_share_ratio;

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

    decrease_pool_lp(pool_name.clone(), amount.clone());
    decrease_user_pool_lp(user, pool_name, amount.clone());
    decrease_total_lp(amount);

    ic_cdk::println!("Successfully burned LP tokens for user: {}", user);

    Ok(())
}



// TODO Send token amount to pool canister instead of user_share ratio
#[update]
async fn get_user_share_ratio(
    params: Pool_Data,
    pool_name: String,
    amount: Nat,
) -> Result<Vec<f64>, String> {
    let user = ic_cdk::caller();

    // Validate input: pool_name
    if pool_name.trim().is_empty() {
        return Err("Pool name cannot be empty.".to_string());
    }

    // Validate input: amount
    if amount <= Nat::from(0u128) {
        return Err("Amount must be greater than zero.".to_string());
    }

    // Validate params
    params.validate().map_err(|e| format!("Invalid pool data: {:?}", e))?;

    // Retrieve the total LP share for the pool
    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share
            .get(&pool_name)
            .cloned()
            .unwrap_or(Nat::from(0u128))
    });

    if pool_total_lp <= Nat::from(0u128) {
        return Err(format!("No LP tokens found for the pool: {}", pool_name));
    }

    // Retrieve the canister ID for the pool
    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    });

    let canister_id = match canister_id {
        Some(id) => id,
        None => return Err(format!("No canister ID found for the pool: {}", pool_name)),
    };

    // Retrieve the pool value
    let pool_value: Nat = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        borrowed_pool_lp
            .get(&pool_name)
            .map(|lp_value| lp_value.clone() * Nat::from(1000u128))
            .unwrap_or(Nat::from(0u128))
    });

    if pool_value <= Nat::from(0u128) {
        return Err(format!("No tokens in the pool: {}", pool_name));
    }

    // Calculate the user share ratio
    let user_share_ratio = match amount.clone().to_string().parse::<f64>() {
        Ok(amount_f64) => match pool_total_lp.to_string().parse::<f64>() {
            Ok(total_f64) => amount_f64 / total_f64,
            Err(_) => return Err("Failed to parse pool_total_lp to f64.".to_string()),
        },
        Err(_) => return Err("Failed to parse amount to f64.".to_string()),
    };

    let pool_value_f64 = match pool_value.to_string().parse::<f64>() {
        Ok(value) => value,
        Err(_) => return Err("Failed to parse pool_value to f64.".to_string()),
    };

    let tokens_to_transfer = pool_value_f64 * user_share_ratio;

    // Call the `get_burned_tokens` function on the canister
    let result: Result<(Vec<f64>,), String> = call(
        canister_id,
        "get_burned_tokens",
        (params, user, tokens_to_transfer),
    )
    .await
    .map_err(|e| format!("Failed to get token data: {:?}", e));

    // Return the result from the call
    result.map(|(burned_tokens_vec,)| burned_tokens_vec)
}


#[update]
fn decrease_pool_lp(pool_name: String, amount: Nat) {
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
fn decrease_user_pool_lp(user: Principal, pool_name: String, amount: Nat) {
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
fn decrease_total_lp(lp: Nat) {
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

