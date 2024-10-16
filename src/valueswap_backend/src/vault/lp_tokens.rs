use candid::Principal;
use ic_cdk_macros::{query, update};
use std::cell::{Ref, RefCell};
use std::collections::{BTreeMap, HashMap};

use crate::api::deposit::deposit_tokens;
use crate::api::transfer::icrc1_transfer;
use crate::constants::asset_address::LP_LEDGER_ADDRESS;
// use crate::utils::maths::*;
use crate::utils::types::*;
use crate::with_state;

thread_local! {
    static TOTAL_LP_SUPPLY : RefCell<f64> = RefCell::new(0.0);
    static POOL_LP_SHARE : RefCell<HashMap<String , f64>> = RefCell::new(HashMap::new());
    static USERS_LP : RefCell<BTreeMap<Principal, f64>> = RefCell::new(BTreeMap::new());
}

// To map pool with their LP tokens
#[update]
pub fn increase_pool_lp_tokens(params: Pool_Data) -> HashMap<String, f64> {
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
async fn users_lp_share(user: Principal, params: Pool_Data) -> Result<() , String>{
    USERS_LP.with(|share| {
        let mut users_contribution = 0.0;
        for amount in params.pool_data {
            users_contribution += amount.value as f64 * amount.balance as f64;
        }
        let total_pool_value = get_total_lp() * 10.0;
        let total_lp_supply = get_total_lp();
        let mut borrowed_share = share.borrow_mut();
        let amount = (users_contribution / total_pool_value) * total_lp_supply;
        let amount_as_u64 = amount as u64;
        borrowed_share.insert(
            user,
            amount.clone(),
        );

        ic_cdk::spawn(async move {
            let transfer_result = icrc1_transfer(user, amount_as_u64).await;
            if let Err(e) = transfer_result {
                ic_cdk::trap(&format!("Transfer failed : {}" , e));
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
async fn burn_lp_tokens(pool_name : String , amount : f64){
    let user = ic_cdk::caller();
    let ledger_canister_id = Principal::from_text(LP_LEDGER_ADDRESS).expect("Invalid ledger canister id");
    let target_canister_id = ic_cdk::id();

    let result = deposit_tokens(amount as u64, ledger_canister_id, target_canister_id).await;
    if let Err(e) = result {
        ic_cdk::trap(&format!("Transfer failed : {}", e));
    }

    let canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        // Extract the principal if available
        pool_borrowed.get(&pool_name).map(|user_principal| user_principal.principal)
    });
    
    // calculate the amount of tokens to be transferred to the user

    decrease_pool_lp(pool_name ,amount);
    decrease_total_lp(amount);
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
fn decrease_total_lp(LP: f64){
    TOTAL_LP_SUPPLY.with(|total_lp|{
        let mut borrowed_lp = total_lp.borrow_mut();
        if(*borrowed_lp > 0.0){
          *borrowed_lp -=  LP;
        }
        else{
            ic_cdk::trap(&format!("Insufficient LP tokens"));
        }
    })
}


