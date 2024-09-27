use candid::{CandidType, Deserialize, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;
use ic_cdk::api::caller;
use std::fs;
use std::path::Path;

mod utils;
mod api;

pub use utils::maths::*;
pub use utils::types::*;
// Remove unused import
// pub use api::transfer::*;

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<PoolData>>> = RefCell::new(BTreeMap::new());
}

// store user_id with pool data
#[update]
async fn store_pool_data(user_principal: Principal, params: PoolData) -> Result<(), String> {
    let key = user_principal;

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        pool_data_borrowed.entry(key).or_default().push(liquidity);
    });

    Ok(())
}

// check if user exists and if it exists update pool data else add user entry with pool data
#[update]
async fn add_liquidity_to_pool(user_principal: Principal, params: PoolData) -> Result<(), String> {
    let _pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        if let Some(existing_pool_data) = pool_data_borrowed.get_mut(&user_principal) {
            existing_pool_data.push(liquidity)
        } else {
            pool_data_borrowed
                .entry(user_principal)
                .or_default()
                .push(liquidity);
        }
    });

    Ok(())
}

fn pre_swap(params: SwapParams) -> Result<(), SwapError> {
    let entered_token: u64 = params.token_amount;

    if entered_token <= 0 {
        return Err(SwapError::InvalidAmount);
    }
    Ok(())
}

#[update]
async fn swap(params: SwapParams) -> Result<(), String> {
    let user_principal = caller();

    // Fetch user pool data
    let pool_data = POOL_DATA.with(|pool_data| {
        pool_data.borrow().get(&user_principal).cloned()
    });

    if pool_data.is_none() {
        return Err("User has no pool data".to_string());
    }

    let mut user_pool_data = pool_data.unwrap();

    // Check if user has enough balance and liquidity
    let mut has_sufficient_balance = false;
    let mut has_sufficient_liquidity = false;

    for pool in &mut user_pool_data {
        for token in &mut pool.pool_data {
            if token.token_name == params.token1_name && token.balance >= params.token_amount {
                has_sufficient_balance = true;
                token.balance -= params.token_amount;
            }
            if token.token_name == params.token2_name {
                has_sufficient_liquidity = true;
                token.balance += params.token_amount;
            }
        }
    }

    if !has_sufficient_balance {
        return Err("Insufficient balance".to_string());
    }

    if !has_sufficient_liquidity {
        return Err("Insufficient liquidity".to_string());
    }

    // Update the pool data
    POOL_DATA.with(|pool_data| {
        pool_data.borrow_mut().insert(user_principal, user_pool_data);
    });

    Ok(()) 
}

// #[query]
// fn get_pool_data(params: Principal) -> Option<Vec<PoolData>> {
//     let user_principal = params;
//     let pool_data = POOL_DATA.with(|pool_data| {
//         pool_data.borrow().get(&user_principal).cloned()
//     });
    
//     pool_data
// }

#[query]
fn get_pool_data(params: Principal) -> Vec<PoolData> {
    let user_principal = params;
    POOL_DATA.with(|pool_data| {
        pool_data.borrow().get(&user_principal).cloned().unwrap_or_else(Vec::new)
    })
}

export_candid!();


