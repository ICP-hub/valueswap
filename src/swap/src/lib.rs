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
    pub static LP_SHARE : RefCell<BTreeMap<Principal , f64>> = RefCell::new(BTreeMap::new());
    pub static TOTAL_LP : RefCell<f64> = RefCell::new(0.0);
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



// store user_id with pool data
#[update]
async fn store_pool_data(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    // let key = format!("{},{}", pool_name, params.swap_fee);
    let key = user_principal;
    // increase_total_lp(params.clone());

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
    ic_cdk::println!("just call the swap");
        // Convert f64 to u64
        let amount_as_u64 = amount as u64;

        // Convert u64 to Nat
        // let amount_nat = Nat::from(amount_as_u64);

    // Example usage within your swap function
    ic_cdk::println!("just call transfer");
    let transfer_result =
     icrc1_transfer(params.ledger_canister_id2.clone(), user_principal, amount_as_u64).await;

    if let Err(e) = transfer_result {
     ic_cdk::println!("Transfer failed: {:?}", e);
     return Err("Token transfer failed.".to_string());
    }   

    let token_amount_as_u64 = params.token_amount.clone();
    let token_amount_nat = Nat::from(token_amount_as_u64);

    // let transfer_result2 = icrc1_transfer(user_principal, token_canister_id, token_amount_nat);
    // if let Err(e) = transfer_result {
    //     ic_cdk::println!("Transfer failed : {:?}", e);
    //     return Err("Token transfer failed to pool canister".to_string());
    // }

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

#[query]
fn function() -> String{
    "hello".to_string()
}

export_candid!();
