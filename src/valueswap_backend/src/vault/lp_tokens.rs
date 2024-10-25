use candid::Principal;
use ic_cdk::api::call;
use ic_cdk::{api, call};
use ic_cdk_macros::{query, update};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap};

use crate::api::deposit::deposit_tokens;
use crate::api::transfer::icrc1_transfer;
use crate::constants::asset_address::LP_LEDGER_ADDRESS;
use crate::utils::types::*;
use crate::with_state;

thread_local! {
    static TOTAL_LP_SUPPLY : RefCell<f64> = RefCell::new(0.0);
    static POOL_LP_SHARE : RefCell<BTreeMap<String , f64>> = RefCell::new(BTreeMap::new());
    static USERS_LP : RefCell<BTreeMap<Principal, f64>> = RefCell::new(BTreeMap::new());
    static USERS_POOL : RefCell<BTreeMap<Principal , Vec<String>>> = RefCell::new(BTreeMap::new());
}

// To map pool with their LP tokens

#[update]
pub fn increase_pool_lp_tokens(params: Pool_Data) {
    POOL_LP_SHARE.with(|lp_share| {
        let mut borrowed_lp_share = lp_share.borrow_mut();

        // Calculate the pool's total value (sum of value * balance for each token in the pool)
        let pool_supply: f64 = params
            .pool_data
            .iter()
            .map(|pool| pool.value as f64 * pool.balance as f64)
            .sum();

        // Create a unique key for the pool using the token names (concatenate token names)
        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        // If the pool already exists, add the new pool supply; otherwise, insert a new entry
        borrowed_lp_share
            .entry(key)
            .and_modify(|existing_supply| *existing_supply += pool_supply / 10.0)
            .or_insert(pool_supply / 10.0);
    });
    users_pool(params.clone());
    total_lp_tokens();
}

#[update]
pub fn users_pool(params: Pool_Data) {
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
                // If the user exists, only push the new pool if it doesn't already exist
                if !user_pools.contains(&new_pool) {
                    user_pools.push(new_pool.clone());
                }
            })
            .or_insert_with(|| vec![new_pool]);
    });
}

#[query]
fn get_users_pool(user: Principal) -> Option<Vec<String>> {
    USERS_POOL.with(|pool| {
        let borrowed_pool = pool.borrow();
        if let Some(pool_name) = borrowed_pool.get(&user) {
            Some(pool_name.clone())
        } else {
            None
        }
    })
}

// To get all lp tokens

#[update]
fn total_lp_tokens() {
    let mut total_supply: f64 = 0.0;
    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, f64> = share.borrow().clone();
        for (_key, value) in temp.iter() {
            total_supply += value;
        }
        total_supply = total_supply / 10.0;
    });

    TOTAL_LP_SUPPLY.with(|lp_supply| *lp_supply.borrow_mut() = total_supply);
}

#[query]
fn get_total_lp() -> f64 {
    TOTAL_LP_SUPPLY.with(|total_lp| total_lp.borrow().clone())
}

// Query to get LP tokens for a specific pool

#[query]
fn get_lp_tokens(pool_name: String) -> Option<f64> {
    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, f64> = share.borrow().clone();
        if let Some(key) = temp.get(&pool_name) {
            Some(*key)
        } else {
            None
        }
    })
}

#[update]
pub async fn users_lp_share(user: Principal, params: Pool_Data) -> Result<(), String> {
    let mut users_contribution :f64 = 1.0;
    USERS_LP.with(|share| {
        for amount in params.pool_data {
            users_contribution += amount.value as f64 * amount.balance as f64;
        }
        let total_pool_value = get_total_lp() * 10.0;
        let total_lp_supply = get_total_lp();
        let mut borrowed_share = share.borrow_mut();
        let amount : f64 = (users_contribution / total_pool_value) * total_lp_supply;
        let amount_as_u64 = amount as u64;
        borrowed_share.insert(user, amount.clone());

        ic_cdk::spawn(async move {
            let transfer_result = icrc1_transfer(user, amount_as_u64).await;
            if let Err(e) = transfer_result {
                ic_cdk::trap(&format!("Transfer failed : {}", e));
            }
        });

        Ok(())
    })
}

#[query]
fn get_users_lp(user_id: Principal) -> Option<f64> {
    USERS_LP.with(|lp| {
        let borrowed_lp = lp.borrow();
        borrowed_lp.get(&user_id).cloned()
    })
}

#[update]
async fn burn_lp_tokens(params: Pool_Data, pool_name: String, amount: f64) -> Result<(), String> {
    let user = ic_cdk::caller();
    let ledger_canister_id =
        Principal::from_text(LP_LEDGER_ADDRESS).expect("Invalid ledger canister id");
    let target_canister_id = ic_cdk::id();

    let result = deposit_tokens(amount as u64, ledger_canister_id, target_canister_id).await;
    if let Err(e) = result {
        ic_cdk::trap(&format!("Transfer failed : {}", e));
    }

    let canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        // Extract the principal if available
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    });

    let canister_id = match canister_id {
        Some(id) => id,
        None => ic_cdk::trap(&format!("No canister ID found for the pool")),
    };

    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share.get(&pool_name).cloned().unwrap_or(0.0)
    });

    if pool_total_lp <= 0.0 {
        ic_cdk::trap(&format!("No LP tokens in the pool: {}", pool_name));
    }

    let user_share_ratio = amount / pool_total_lp;

    let pool_value: f64 = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        if let Some(&lp_value) = borrowed_pool_lp.get(&pool_name) {
            lp_value * 10.0
        } else {
            0.0
        }
    });

    if pool_value <= 0.0 {
        ic_cdk::trap(&format!("No tokens in the pool: {}", pool_name));
    }

    let tokens_to_transfer = pool_value * user_share_ratio;

    let result: Result<(), String> =
        call(canister_id, "burn_tokens", (params, user, user_share_ratio))
            .await
            .map_err(|e| format!("Failed to perform swap: {:?}", e));

    // if let Err(e) = result {
    //     return Err(e);
    // }

    decrease_pool_lp(pool_name, amount);
    decrease_total_lp(amount);
    Ok(())
}

#[update]
async fn get_user_share_ratio(
    params: Pool_Data,
    pool_name: String,
    amount: f64,
) -> Result<Vec<f64>, String> {
    let user = ic_cdk::caller();

    // Retrieve the total LP share for the pool
    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share.get(&pool_name).cloned().unwrap_or(0.0)
    });

    // Retrieve the canister ID for the pool
    let canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    });

    // Check if canister ID was found
    let canister_id = match canister_id {
        Some(id) => id,
        None => return Err(format!("No canister ID found for the pool")),
    };

    // Calculate the user share ratio
    let user_share_ratio = pool_total_lp / amount;

    // Call the `get_burned_tokens` function on the canister
    let result: Result<(Vec<f64>,), String> = call(
        canister_id,
        "get_burned_tokens",
        (params, user, user_share_ratio),
    )
    .await
    .map_err(|e| format!("Failed to get token data: {:?}", e));

    // Return the result from the call
    result.map(|(burned_tokens_vec,)| burned_tokens_vec)
}




#[update]
fn decrease_pool_lp(pool_name: String, amount: f64) {
    POOL_LP_SHARE.with(|pool| {
        let mut pool_lp_share = pool.borrow_mut();
        if let Some(current_lp) = pool_lp_share.get_mut(&pool_name) {
            // Ensure the LP amount does not go negative
            if *current_lp >= amount {
                *current_lp -= amount;
            } else {
                ic_cdk::trap(&format!("Insufficient LP tokens in pool: {}", pool_name));
            }
        } else {
            ic_cdk::trap(&format!("Pool not found: {}", pool_name));
        }
    });
}

#[update]
fn decrease_total_lp(LP: f64) {
    TOTAL_LP_SUPPLY.with(|total_lp| {
        let mut borrowed_lp = total_lp.borrow_mut();
        if (*borrowed_lp > 0.0) {
            *borrowed_lp -= LP;
        } else {
            ic_cdk::trap(&format!("Insufficient LP tokens"));
        }
    })
}
