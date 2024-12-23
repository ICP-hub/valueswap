use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::caller;
use ic_cdk_macros::*;
use serde::de::value::Error;
use std::borrow::Borrow;
use std::cell::Ref;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod api;
mod utils;

pub use api::transfer::*;
pub use utils::maths::*;
pub use utils::types::*;

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
    // pub static LP_SHARE : RefCell<BTreeMap<Principal , f64>> = RefCell::new(BTreeMap::new());
    pub static POOL_BALANCE : RefCell<BTreeMap<Principal , Nat>> = RefCell::new(BTreeMap::new());
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

// #[update]
// pub fn cmm_algorithm(){

// }


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


// TODO : make ledger calls with state checks for balance to prevent TOCTOU vulnerablities

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

// #[query]
// fn pre_compute_swap(params: SwapParams) -> (String, f64) {
//     // let required_pools = match search_swap_pool(params.clone()) {
//     //     Ok(pools) => pools,
//     //     Err(_) => {
//     //         ic_cdk::println!("No matching pools found.");
//     //         return ("No matching pools found.".to_string(), 0.0);
//     //     }
//     // };

//     // let mut best_pool = None;
//     // let mut max_output_amount = 0.0;

//     POOL_DATA.with(|pool_data| {
//         let pool_data = pool_data.borrow();

//         for pool_key in required_pools {
//             let pool_entries = match pool_data.get(&pool_key) {
//                 Some(entries) => entries,
//                 None => {
//                     ic_cdk::println!("Pool key {} not found in POOL_DATA.", pool_key);
//                     continue;
//                 }
//             };

//             for data in pool_entries {
//                 // Find the tokenA (input) and tokenB (output) from the pool data
//                 let tokenA_data = data
//                     .pool_data
//                     .iter()
//                     .find(|p| p.token_name == params.token1_name);
//                 let tokenB_data = data 
//                     .pool_data
//                     .iter()
//                     .find(|p| p.token_name == params.token2_name);

//                 // ic_cdk::println!(
//                 //     "Testing pool_key {} with tokenA_data: {:?}, tokenB_data: {:?}",
//                 //     pool_key,
//                 //     tokenA_data,
//                 //     tokenB_data
//                 // );

//                 if let (Some(tokenA), Some(tokenB)) = (tokenA_data, tokenB_data) {
//                     let b_i = tokenA.balance as f64;
//                     let w_i = tokenA.weight as f64;
//                     let b_o = tokenB.balance as f64;
//                     let w_o = tokenB.weight as f64;
//                     // ic_cdk::println!("Argument for swap {:?} , {:?} , {:?} , {:?}",b_i,w_i,b_o,w_o);

//                     let amount_out = params.token_amount as f64;
//                     let fee = data.swap_fee;
//                     // ic_cdk::println!("{:?}, {:?} ",amount_out , fee);

//                     // Calculate the required input using the in_given_out formula
//                     let required_input = out_given_in(b_i, w_i, b_o, w_o, amount_out, fee);
//                     // ic_cdk::println!("The required output is {:?}", required_input);
//                     // ic_cdk::println!("Required Input {:}", required_input);

//                     // Ensure the user has enough balance to provide the input
//                     if required_input >= max_output_amount {
//                         max_output_amount = f64::max(required_input, max_output_amount);

//                         best_pool = Some(pool_key.clone());
//                         // Check if the current pool gives a better output
//                         // if calculated_output > max_output_amount {
//                         //     max_output_amount = calculated_output;
//                         // }
//                     }
//                 } else {
//                     ic_cdk::println!("Either tokenA or tokenB was not found in pool.");
//                 }
//             }
//         }
//     });

//     match best_pool {
//         Some(pool) => (pool, max_output_amount),
//         None => ("No suitable pool found.".to_string(), 0.0),
//     }
// }


#[update]
async fn swap(user_principal: Principal, params: SwapParams, amount: Nat) -> Result<(), String> {
    // pool canister id
    // let token_canister_id = ic_cdk::api::id();

    // Convert f64 to u64
    // let amount_as_u64: u64 = amount as u64;

    // Convert u64 to Nat
    // let amount_nat = Nat::from(amount_as_u64);

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
    let mut has_sufficient_balance = false;
    let mut has_sufficient_liquidity = false;

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
