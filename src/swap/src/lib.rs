use ic_cdk_macros::*;
use std::collections::BTreeMap;
use candid::{CandidType, Deserialize, Principal};
use std::cell::RefCell;

#[derive(CandidType, Deserialize, Clone)]
pub struct CreatePoolParams {
    pub token_name: String,
    pub balance: u64,
    pub weight: f64,
    pub value: u64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Pool_Data {
    pub pool_data: Vec<CreatePoolParams>,
    pub swap_fee: f64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct PoolLiquidity {
    pub user_principal: Principal,
    pub data: Pool_Data,
}

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<String, Vec<PoolLiquidity>>> = RefCell::new(BTreeMap::new());
}

// check if user exists and if it exists update pool data else add user entry with pool data
#[update]
async fn add_liquidity_to_pool(user_principal: Principal, params: Pool_Data) -> Result<(), String> {
    let pool_name = params.pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    let key = format!("{},{}", pool_name, params.swap_fee);

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = PoolLiquidity {
            user_principal,
            data: params,
        };

        pool_data_borrowed.entry(key).or_default().push(liquidity);
    });

    Ok(())
}

// store user_id with pool data
#[update]
async fn store_pool_data(params: Pool_Data, canister_id: Principal) -> Result<(), String> {
    let pool_name = params.pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    let key = format!("{},{}", pool_name, params.swap_fee);

    POOL_DATA.with(|pool_data| {
        let mut pool_data_borrowed = pool_data.borrow_mut();
        let liquidity = PoolLiquidity {
            user_principal: canister_id,
            data: params,
        };

        pool_data_borrowed.insert(key, vec![liquidity]);
    });

    Ok(())
}
