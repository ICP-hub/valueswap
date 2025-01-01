use candid::{Nat, Principal};
use ic_cdk:: call;
use ic_cdk_macros::{query, update};
use std::cell:: RefCell;
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
pub fn increase_pool_lp_tokens(params: Pool_Data) {
    let mut pool_supply : Nat = Nat::from(1u128);
    POOL_LP_SHARE.with(|lp_share| {
        let mut borrowed_lp_share = lp_share.borrow_mut();

        // Calculate the pool's total value
        pool_supply = params
            .pool_data
            .iter()
            .map(|pool| pool.value.clone() * pool.balance.clone())
            .fold(Nat::from(0u128), |acc, x| acc + x);

        // Create a unique key for the pool
        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        // Update the pool LP share
        borrowed_lp_share
            .entry(key.clone())
            .and_modify(|existing_supply| {
                *existing_supply += pool_supply.clone() / Nat::from(10u128)
            })
            .or_insert(pool_supply.clone() / Nat::from(10u128));
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

        let lp_tokens : Nat = pool_supply / Nat::from(10u128);

        user_pools
            .entry(user)
            .or_insert_with(BTreeMap::new)
            .entry(key)
            .and_modify(|existing_lp| *existing_lp += lp_tokens.clone())
            .or_insert(lp_tokens);
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

#[query]
fn total_lp_tokens() {
    let mut total_supply: Nat = Nat::from(0u128);
    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, Nat> = share.borrow().clone();
        for (_key, value) in temp.iter() {
            total_supply = total_supply.clone() + value.clone();
        }
        total_supply = total_supply.clone() / Nat::from(1000u128);
    });

    TOTAL_LP_SUPPLY.with(|lp_supply| *lp_supply.borrow_mut() = total_supply);
}

#[query]
fn get_total_lp() -> Nat {
    TOTAL_LP_SUPPLY.with(|total_lp| total_lp.borrow().clone())
}

// Query to get LP tokens for a specific pool

#[query]
pub fn get_pool_lp_tokens(pool_name: String) -> Nat {
    POOL_LP_SHARE.with(|share| {
        let temp: BTreeMap<String, Nat> = share.borrow().clone();
        if let Some(key) = temp.get(&pool_name) {
            key.clone()
        } else {
            Nat::from(0u128)
        }
    })
}

#[query]
pub fn get_user_pools_with_lp(user: Principal) -> Option<BTreeMap<String, Nat>> {
    USERS_POOL_LP.with(|users_pool_lp| {
        let borrowed = users_pool_lp.borrow();
        borrowed.get(&user).cloned()
    })
}


#[update]
pub async fn users_lp_share( params: Pool_Data) -> Result<(), String> {
    let user = ic_cdk::caller();
    let mut users_contribution: Nat = Nat::from(1u128);
    USERS_LP.with(|share| {
        for amount in params.pool_data {
            users_contribution += amount.value * amount.balance;
        }
        let total_pool_value = get_total_lp() * Nat::from(1000u128);
        let total_lp_supply = get_total_lp();
        let mut borrowed_share = share.borrow_mut();
        let amount: Nat = (users_contribution / total_pool_value) * total_lp_supply;
        borrowed_share.insert(user, amount.clone());

        ic_cdk::spawn(async move {
            let mut attempts = 0;
            let max_retries = 2;
            // let mut success = false;

            while attempts < max_retries {
                let transfer_result = icrc1_transfer(user, amount.clone()).await;
                if transfer_result.is_ok() {
                    // success = true;
                    break; // Exit the loop if transfer is successful
                } else {
                    attempts += 1;
                    if attempts == max_retries {
                        log::error!("Transfer failed after {} attempts", attempts);
                    }
                }
            }
        });

        Ok(())
    })
}

#[query]
fn get_users_lp(user_id: Principal) -> Option<Nat> {
    USERS_LP.with(|lp| {
        let borrowed_lp = lp.borrow();
        borrowed_lp.get(&user_id).cloned()
    })
}


// TODO Send token amount to pool canister instead of user_share ratio
#[update]
async fn burn_lp_tokens(params: Pool_Data, pool_name: String, amount: Nat) -> Result<(), String> {
    let user = ic_cdk::caller();
    let ledger_canister_id =
        Principal::from_text(LP_LEDGER_ADDRESS).expect("Invalid ledger canister id");
    let target_canister_id = ic_cdk::id();

    let result = deposit_tokens(amount.clone(), ledger_canister_id, target_canister_id).await;
    if let Err(e) = result {
        ic_cdk::trap(&format!("Transfer failed : {}", e));
    }

    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
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
        borrowed_share
            .get(&pool_name)
            .cloned()
            .unwrap_or(Nat::from(0u128))
    });

    if pool_total_lp <= Nat::from(0u128) {
        ic_cdk::trap(&format!("No LP tokens in the pool: {}", pool_name));
    }

    let user_share_ratio = amount.clone().to_string().parse::<f64>().unwrap()
        / pool_total_lp.to_string().parse::<f64>().unwrap();

    let pool_value: Nat = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        if let Some(lp_value) = borrowed_pool_lp.get(&pool_name) {
            lp_value.clone() * Nat::from(1000u128)
        } else {
            Nat::from(0u128)
        }
    });

    if pool_value <= Nat::from(0u128) {
        ic_cdk::trap(&format!("No tokens in the pool: {}", pool_name));
    }
    
    let pool_value_f64 = pool_value.clone().to_string().parse::<f64>().unwrap();
    // let pool_value_f64 = pool_value.0.to_u64_digits().0.first().cloned().unwrap_or(0) as f64;

    let tokens_to_transfer = pool_value_f64 * user_share_ratio.clone();

    let result: Result<(), String> =
        call(canister_id, "burn_tokens", (params, user, tokens_to_transfer))
            .await
            .map_err(|e| format!("Failed to perform swap: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }

    decrease_pool_lp(pool_name.clone(), amount.clone());
    decrease_user_pool_lp(user , pool_name, amount.clone());
    decrease_total_lp(amount);
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

    // Retrieve the total LP share for the pool
    let pool_total_lp = POOL_LP_SHARE.with(|share| {
        let borrowed_share = share.borrow();
        borrowed_share
            .get(&pool_name)
            .cloned()
            .unwrap_or(Nat::from(0u128))
    });

    // Retrieve the canister ID for the pool
    let canister_id = with_state(|pool| {
        let pool_borrowed = &mut pool.token_pools;
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    });

    // Check if canister ID was found
    let canister_id = match canister_id {
        Some(id) => id,
        None => return Err(format!("No canister ID found for the pool")),
    };

    let pool_value: Nat = POOL_LP_SHARE.with(|pool_lp| {
        let borrowed_pool_lp = pool_lp.borrow();
        if let Some(lp_value) = borrowed_pool_lp.get(&pool_name) {
            lp_value.clone() * Nat::from(1000u128)
        } else {
            Nat::from(0u128)
        }
    });
    // Calculate the user share ratio
    let user_share_ratio = amount.clone().to_string().parse::<f64>().unwrap()
        / pool_total_lp.to_string().parse::<f64>().unwrap();

    let pool_value_f64 = pool_value.clone().to_string().parse::<f64>().unwrap();
    // let pool_value_f64 = pool_value.0.to_u64_digits().0.first().cloned().unwrap_or(0) as f64;

    let tokens_to_transfer = pool_value_f64 * user_share_ratio.clone();    


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
fn decrease_user_pool_lp(user: Principal, pool_name: String, amount: Nat) {
    USERS_POOL_LP.with(|users_pool_lp| {
        let mut borrowed = users_pool_lp.borrow_mut();
        if let Some(user_pools) = borrowed.get_mut(&user) {
            if let Some(current_lp) = user_pools.get_mut(&pool_name) {
                if *current_lp >= amount {
                    *current_lp -= amount;
                } else {
                    ic_cdk::trap(&format!("Insufficient LP tokens for user in pool: {}", pool_name));
                }
            } else {
                ic_cdk::trap(&format!("User not associated with pool: {}", pool_name));
            }
        } else {
            ic_cdk::trap("User not found");
        }
    });
}


#[update]
fn decrease_total_lp(lp: Nat) {
    TOTAL_LP_SUPPLY.with(|total_lp| {
        let mut borrowed_lp = total_lp.borrow_mut();
        if *borrowed_lp > Nat::from(0u128) {
            *borrowed_lp -= lp;
        } else {
            ic_cdk::trap(&format!("Insufficient LP tokens"));
        }
    })
}