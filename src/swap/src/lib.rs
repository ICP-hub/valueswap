use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use serde::de::value::Error;
use std::borrow::Borrow;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod utils;
mod api;

pub use utils::maths::*;
pub use utils::types::*;
pub use api::transfer::*;

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
    // pub static LP_SHARE : RefCell<BTreeMap<Principal , f64>> = RefCell::new(BTreeMap::new());
    pub static POOL_BALANCE : RefCell<BTreeMap<Principal , u64>> = RefCell::new(BTreeMap::new());
}

// #[update]
// fn increase_total_lp(params : Pool_Data)  {
//     TOTAL_LP.with(|lp|{
//         let mut total_lp = lp.borrow_mut().clone();
//         for amount in params.pool_data{
//             let temp = amount.balance as f64 * amount.value as f64;
//             total_lp += temp.borrow();
//         }
//         total_lp
//     });
// }

// #[query]
// fn get_total_lp() -> f64 {
//     TOTAL_LP.with(|lp|{ lp.borrow().clone()})
// }

#[update]
pub fn pool_balance(user_principal : Principal ,params : Pool_Data) {
    let balance : u64 = params
    .pool_data
    .iter()
    .map(|pool| pool.balance.clone())
    .sum();

POOL_BALANCE.with(|pool_balance|{
    let mut borrowed_pool_balance = pool_balance.borrow_mut();
    borrowed_pool_balance.entry(user_principal)
    .and_modify(|user_balance| *user_balance += balance)
    .or_insert(balance);
})
}

#[query]
pub fn get_pool_balance(user_principal : Principal) -> Option<u64> {
    POOL_BALANCE.with(|pool_balance|{
        let borrowed_pool_balance = pool_balance.borrow();
        if let Some(balance) = borrowed_pool_balance.get(&user_principal){
            Some(balance.clone())
        }else{
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
async fn burn_tokens(params : Pool_Data , user : Principal , user_share_ratio : f64) -> Result<(), String> {
    let total_token_balance = match get_pool_balance(user.clone()){
        Some(balance) => balance,
        None => 0
    };

for token in params.pool_data.iter(){
    let token_amount = token.weight as u64 * total_token_balance;
    let transfer_amount = token_amount * user_share_ratio as u64;
    let transfer_result = icrc1_transfer(token.ledger_canister_id , user , transfer_amount).await;

    if let Err(e) = transfer_result{
        ic_cdk::println!("Transfer failed {:}", e );
        return Err("Token transfer failed" . to_string());
    }
}
Ok(())
}

#[query]
async fn get_burned_tokens(params : Pool_Data , user : Principal , user_share_ratio : f64) -> Vec<f64> {
    let total_token_balance = match get_pool_balance(user.clone()){
        Some(balance) => balance,
        None => 0
    };

let mut result : Vec<f64> = vec![];
for token in params.pool_data.iter(){
    let token_amount = token.weight * total_token_balance as f64;
    
    let transfer_amount = user_share_ratio * token_amount;
    result.push(transfer_amount);
}
result
}


// fn pre_swap(params: SwapParams) -> Result<(), SwapError> {
//     let entered_token: u64 = if params.zero_for_one {
//         params.token1
//     } else {
//         params.token2
//     };

//     if entered_token <= 0 {
//         return Err(SwapError::InvalidAmount);
//     }
//     Ok(())
// }

// #[update]
// fn swap(params : SwapParams) -> Result<() , String>{
    

// }

#[update]
async fn swap( user_principal : Principal , params: SwapParams , amount : f64) -> Result<(), String> {
    // pool canister id
    // let token_canister_id = ic_cdk::api::id();

    // Convert f64 to u64
    let amount_as_u64 = amount as u64;

    // Convert u64 to Nat
    // let amount_nat = Nat::from(amount_as_u64);

// Example usage within your swap function
let transfer_result = icrc1_transfer(params.ledger_canister_id, user_principal, amount_as_u64.clone()).await;

if let Err(e) = transfer_result {
 ic_cdk::println!("Transfer failed: {:?}", e);
 return Err("Token transfer failed.".to_string());
}   

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

// fn pre_swap_for_all(params: SwapParams, operator: Principal) -> Result<Nat, SwapError> {
//     // let swap_result = match compute_swap(args.clone(), operator, false) {
//     //     Ok(result) => result,
//     //     Err(code) => return Err(SwapError::InternalError(format!("preswap {:?}", code))),
//     // };

//     let mut effective_amount = 0;
//     let mut swap_amount = 0;

//     // if params.zero_for_one && swap_result.amount1 < 0 {
//     //     swap_amount = Nat::from((-swap_result.amount1) as u64);
//     //     effective_amount = Nat::from(swap_result.amount0 as u64);
//     // }

// //     if !args.zero_for_one && swap_result.amount0 < 0 {
// //         swap_amount = Nat::from((-swap_result.amount0) as u64);
// //         effective_amount = Nat::from(swap_result.amount1 as u64);
// //     }

//     if swap_amount <= Nat::from(0) {
//         return Err(SwapError::InternalError(
//             "The amount of input token is too small.".to_string(),
//         ));
// } else if params.amount_in.parse::<i64>().unwrap_or(0) > effective_amount.to_u64().unwrap_or(0)
//         && effective_amount > Nat::from(0)
//     {
//         return Err(SwapError::InternalError(format!(
//             "The maximum amount of input tokens is {:?}",
//             effective_amount
// //         )));
//     } else {
//         return Ok(swap_amount);
//     }
// }

export_candid!();