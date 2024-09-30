use candid::Principal;
use ic_cdk_macros::*;
use std::cell::RefCell;
use std::collections::BTreeMap;

mod utils;
mod api;

pub use utils::maths::*;
pub use utils::types::*;
pub use api::transfer::*;

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<Principal, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
}

// store user_id with pool data
#[update]
async fn store_pool_data(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    // let key = format!("{},{}", pool_name, params.swap_fee);
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
async fn add_liquidity_to_pool(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    let _pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

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

fn pre_swap(params: SwapParams) -> Result<(), SwapError> {
    let entered_token: u64 = if params.zero_for_one {
        params.token1
    } else {
        params.token2
    };

    if entered_token <= 0 {
        return Err(SwapError::InvalidAmount);
    }
    Ok(())
}

#[update]
fn swap(){

}

// fn compute_swap(params: SwapParams , ) -> Result<() , SwapError> {
    
// }

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
