use candid::{ Nat, Principal};
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod api;
mod utils;

pub use api::transfer::*;
pub use utils::maths::*;
pub use utils::types::*;

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
    pub static POOL_BALANCE : RefCell<BTreeMap<Principal , Nat>> = RefCell::new(BTreeMap::new());
}



#[update]
pub fn pool_balance(user_principal: Principal, params: Pool_Data) {
    let balance: Nat = params
        .pool_data
        .iter()
        .map(|pool| pool.balance.clone())
        .fold(Nat::from(0u128), |acc, x| acc + x);
        // .sum();

    POOL_BALANCE.with(|pool_balance| {
        let mut borrowed_pool_balance = pool_balance.borrow_mut();
        borrowed_pool_balance
            .entry(user_principal)
            .and_modify(|user_balance| *user_balance += balance.clone())
            .or_insert(balance);
    })
}

#[query]
pub fn get_pool_balance(user_principal: Principal) -> Option<Nat> {
    POOL_BALANCE.with(|pool_balance| {
        let borrowed_pool_balance = pool_balance.borrow();
        if let Some(balance) = borrowed_pool_balance.get(&user_principal) {
            Some(balance.clone())
        } else {
            None
        }
    })
}

// store user_id with pool data

#[update]
async fn store_pool_data(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    // let key = format!("{},{}", pool_name, params.swap_fee);
    let key = user_principal;
    // increase_total_lp(params.clone());
    pool_balance(user_principal.clone(), params.clone());

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        pool_data_borrowed.entry(key).or_default().push(liquidity);
    });

    Ok(())
}

// check if user exists and if it exists update pool data else add user entry with pool data

#[update]
async fn add_liquidity_to_pool(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    let _pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    // increase_total_lp(params.clone());
    pool_balance(user_principal.clone(), params.clone());

    // let key = format!("{},{}", pool_name, params.swap_fee);

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = params.clone();

        // check if user exists
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


#[update]
pub async fn lp_rollback(user: Principal, pool_data: Pool_Data) -> Result<(), String> {
    for amount in pool_data.pool_data.iter() {
        let transfer_result = icrc1_transfer(amount.ledger_canister_id , user, amount.balance.clone()).await;
        if let Err(e) = transfer_result {
            ic_cdk::trap(&format!(
                "Rollback failed for user {} on token {}: {}",
                user, amount.token_name, e
            ));
        }
    }

    Ok(())
}



// TODO : make ledger calls with state checks for balance to prevent TOCTOU vulnerablities
// TODO : make use of user_share_ratio

#[update]
async fn burn_tokens(
    params: Pool_Data,
    user: Principal,
    user_share_ratio: f64,
) -> Result<(), String> {
    let total_token_balance = match get_pool_balance(user.clone()) {
        Some(balance) => balance,
        None => Nat::from(0u128),
    };

    for token in params.pool_data.iter() {
        let token_amount = token.weight.clone() * total_token_balance.clone();
        // let transfer_amount = token_amount * user_share_ratio as u64;
        let transfer_result = icrc1_transfer(token.ledger_canister_id, user, token_amount).await;

        if let Err(e) = transfer_result {
            ic_cdk::println!("Transfer failed {:}", e);
            return Err("Token transfer failed".to_string());
        }
    }
    Ok(())
}


#[query]
async fn get_burned_tokens(params: Pool_Data, user: Principal, user_share_ratio: f64) -> Vec<f64> {
    let total_token_balance = match get_pool_balance(user.clone()) {
        Some(balance) => balance,
        None => Nat::from(0u128),
    };

    let mut result: Vec<f64> = vec![];
    for token in params.pool_data.iter() {
        let token_amount = token.weight.clone() * total_token_balance.clone() ;
        let transfer_amount = token_amount.to_string().parse::<f64>().unwrap_or_default() / user_share_ratio.to_string().parse::<f64>().unwrap_or_default();

        // let transfer_amount = user_share_ratio * token_amount;
        result.push(transfer_amount);
    }
    result
}


#[update]
async fn swap(user_principal: Principal, params: SwapParams, amount: Nat) -> Result<(), String> {

    // Example usage within your swap function
    let transfer_result = icrc1_transfer(
        params.ledger_canister_id2,
        user_principal,
        amount.clone(),
    )
    .await;

    if let Err(e) = transfer_result {
        ic_cdk::println!("Transfer failed: {:?}", e);
        return Err("Token transfer failed.".to_string());
    }

    // Fetch user pool data
    let mut pool_data = POOL_DATA.with(|pool_data| pool_data.borrow_mut().clone());

    // if pool_data.is_none() {
    //     return Err("User has no pool data".to_string());
    // }

    // let mut user_pool_data = pool_data.unwrap();

    // Check if user has enough balance and liquidity
    let has_sufficient_balance = false;
    let has_sufficient_liquidity = false;

    let length = pool_data.len();

    let distribute_amount1 = params.token_amount / length;
    let distribute_amount2 = amount / length ;

    for (user, pools) in pool_data.iter_mut() {
        for pool in pools {
            for token in &mut pool.pool_data {
                if token.token_name == params.token1_name {
                    // Ensure sufficient balance
                    if token.balance < distribute_amount1.clone() {
                        return Err(format!(
                            "User {:?} has insufficient balance for token {}",
                             user,  token.token_name
                        ));
                    }
                    // Subtract distributed amount from token
                    token.balance -= distribute_amount1.clone() ;
                }

                if token.token_name == params.token2_name {
                    token.balance += distribute_amount2.clone();
                }
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
    POOL_DATA.with(|pool_data_cell| {
        let mut pool_data_mut = pool_data_cell.borrow_mut();
        *pool_data_mut = pool_data;
    });

    Ok(())
}


export_candid!();
