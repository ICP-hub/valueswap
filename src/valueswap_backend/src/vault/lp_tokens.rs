use candid::Principal;
use ic_cdk_macros::{query, update};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap};

// use crate::utils::maths::*;
use crate::utils::types::*;
// use crate::vault::vault_pool::VAULT;

thread_local! {
    static TOTAL_LP_SUPPLY : RefCell<f64> = RefCell::new(0.0);
    static POOL_LP_SHARE : RefCell<HashMap<String , f64>> = RefCell::new(HashMap::new());
    static USERS_LP : RefCell<BTreeMap<Principal, f64>> = RefCell::new(BTreeMap::new());
}

// To map pool with their LP tokens
#[update]
pub fn increase_lp_tokens(params: Pool_Data) -> HashMap<String, f64> {
    POOL_LP_SHARE.with(|lp_share| {
        let pool_supply: f64 = params
            .pool_data
            .iter()
            .map(|pool| pool.value.clone() as f64 * pool.balance.clone() as f64)
            .sum();

        // let key: String = params.token_names.join("");
        let key: String = params
            .pool_data
            .iter()
            .map(|pool| pool.token_name.clone())
            .collect::<Vec<String>>()
            .join("");

        lp_share.borrow_mut().insert(key, pool_supply);
    });

    POOL_LP_SHARE.with(|lp_share| lp_share.borrow().clone())
}

// To get all lp tokens
#[update]
fn total_lp_tokens() {
    let mut total_supply: f64 = 0.0;
    POOL_LP_SHARE.with(|share| {
        let temp: HashMap<String, f64> = share.borrow().clone();
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
        let temp: HashMap<String, f64> = share.borrow().clone();
        if let Some(key) = temp.get(&pool_name) {
            Some(*key)
        } else {
            None
        }
    })
}

#[update]
fn users_lp_share(user: Principal, params: Pool_Data) {
    USERS_LP.with(|share| {
        let mut users_contribution = 0.0;
        for amount in params.pool_data {
            users_contribution += amount.value as f64 * amount.balance as f64;
        }
        let total_pool_value = get_total_lp() * 10.0;
        let total_lp_supply = get_total_lp();
        let mut borrowed_share = share.borrow_mut();
        borrowed_share.insert(
            user,
            (users_contribution / total_pool_value) * total_lp_supply,
        );
    });
}

#[query]
fn get_users_lp(user_id: Principal) -> Option<f64> {
    USERS_LP.with(|lp| {
        let borrowed_lp = lp.borrow();
        borrowed_lp.get(&user_id).cloned()
    })
}

// #[update]
// fn burn_lp_tokens(){

// }
