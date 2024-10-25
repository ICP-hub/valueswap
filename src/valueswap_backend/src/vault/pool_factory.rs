use crate::{user_principal, with_state};
use candid::types::reserved;
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::*;
use serde::de::value;
use core::panic;
use std::cell::RefCell;
use std::{cmp, result};
use std::collections::{BTreeMap, HashMap};
// use std::simd::cmp;
use core::cmp::max;
// use serde::{Deserialize, Serialize};
use ic_cdk::{
    api,
    api::{
        call::{call_with_payment128, CallResult},
        canister_version,
        management_canister::main::{CanisterInstallMode, WasmModule},
    },
    call,
};

use crate::api::deposit::deposit_tokens;
use crate::types::state_handlers;
use crate::utils::maths::*;
use crate::utils::types::*;
use crate::vault::lp_tokens::*;

use ic_stable_structures::{writer::Writer, Memory as _, StableBTreeMap};

// use super::vault_pool::arrange_key;

thread_local! {
    // pub static TOKEN_POOLS: RefCell<HashMap<String, Principal>> = RefCell::new(HashMap::new());
    // pub static POOL_ID : RefCell<BTreeMap<String , String>> = RefCell::new(BTreeMap::new());
    pub static POOL_DATA: RefCell<BTreeMap<String, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
}

fn prevent_anonymous() -> Result<(), String> {
    if api::caller() == Principal::anonymous() {
        Err("Anonymous access not allowed".to_string())
    } else {
        Ok(())
    }
}

#[update(guard = prevent_anonymous)]
async fn create_pools(params: Pool_Data) -> Result<(), String> {
    let principal_id = api::caller();
    let pool_name = params
        .pool_data
        .iter() 
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    let pool_canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        if let Some(canister_id) = pool_borrowed.get(&pool_name) {
            return Some(canister_id);
        } else {
            None
        }
    });

    if let Some(canister_id) = pool_canister_id {
        add_liquidity_curr(params.clone());
        add_liquidity(params.clone(), canister_id.principal);
        increase_pool_lp_tokens(params.clone());
        users_pool(params.clone());
        users_lp_share(principal_id.clone(), params.clone()).await?;
        for amount in params.pool_data.iter() {
            // Deposit tokens to the newly created canister
            // ic_cdk::println!("canister_id.principal{:}",canister_id.principal);
            deposit_tokens(amount.balance.clone(), amount.ledger_canister_id.clone(), canister_id.principal).await?;
            // Deposit tokens when testing with static canister id
            // deposit_tokens(amount.balance.clone(), canister_id).await?;
        }
        Ok(())
    } else {
        match create().await {

            Ok(canister_id_record) => {
                let canister_id = canister_id_record;
                with_state(|pool| {
                    pool.TOKEN_POOLS.insert(
                        pool_name.clone(),
                        crate::user_principal {
                            principal: canister_id,
                        },
                    );
                });

                store_pool_data_curr(params.clone());
                store_pool_data(params.clone(), canister_id_record).await?;

                increase_pool_lp_tokens(params.clone());
                users_pool(params.clone());
                users_lp_share(principal_id.clone(), params.clone()).await?;
                
                for amount in params.pool_data.iter() {
                    // Deposit tokens to the newly created canister
                    deposit_tokens(amount.balance.clone() , amount.ledger_canister_id.clone() , canister_id ).await?;

                    // Deposit tokens when testing with static canister id
                    // deposit_tokens(amount.balance.clone(), canister_id).await?;
                }

                Ok(())
            }
            Err(( err_string)) => Err(format!("Error creating canister: {}", err_string)),
        }
    }
}

#[update]
// Create canister
async fn create_canister(arg: CreateCanisterArgument) -> CallResult<(CanisterIdRecord,)> {
    let extended_arg = CreateCanisterArgumentExtended {
        settings: arg.settings,
        sender_canister_version: Some(canister_version()),
    };
    let cycles: u128 = 200_000_000_000;

    call_with_payment128(
        Principal::management_canister(),
        "create_canister",
        (extended_arg,),
        cycles,
    )
    .await
}

async fn deposit_cycles(arg: CanisterIdRecord, cycles: u128) -> CallResult<()> {
    call_with_payment128(
        Principal::management_canister(),
        "deposit_cycles",
        (arg,),
        cycles,
    )
    .await
}

async fn install_code(arg: InstallCodeArgument) -> CallResult<()> {
    let wasm_module_sample: Vec<u8> =
        include_bytes!("../../../../.dfx/local/canisters/swap/swap.wasm").to_vec();

    let extended_arg = InstallCodeArgumentExtended {
        mode: arg.mode,
        canister_id: arg.canister_id,
        wasm_module: wasm_module_sample,
        arg: arg.arg,
        sender_canister_version: Some(canister_version()),
    };

    call_with_payment128(
        Principal::management_canister(),
        "install_code",
        (extended_arg,),
        0,
    )
    .await
}

#[update]
pub async fn create() -> Result<Principal, String> {
    let arg = CreateCanisterArgument { settings: None };

    let (canister_id_record,) = match create_canister(arg).await {
        Ok(id) => id,
        Err((_, err_string)) => {
            ic_cdk::println!("Error in creating canister: {}", err_string);
            return Err(format!("Error: {}", err_string));
        }
    };

    let canister_id = canister_id_record.canister_id;

    let _add_cycles: Result<(), String> =

        match deposit_cycles(canister_id_record, 200_000_000_000).await {
            Ok(_) => Ok(()),
            Err((_, err_string)) => {
                ic_cdk::println!("Error in depositing cycles: {}", err_string);
                return Err(format!("Error: {}", err_string));
            }
        };

    let arg1 = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: vec![],
        arg: Vec::new(),
    };

    let _install_code: Result<(), String> = match install_code(arg1).await {
        Ok(_) => Ok(()),
        Err((_, err_string)) => {
            // ic_cdk::println!("Error in installing code: {}", err_string);
            panic!("Not able to install code");
            // return Err(format!("Error: {}", err_string));
        }
    };

    ic_cdk::println!("Canister ID: {:?}", canister_id.to_string());
    Ok(canister_id)
}



#[update]
async fn install_wasm_on_new_canister(canister_id: Principal) -> Result<(), String> {
    let install_code_args = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id: canister_id,
        wasm_module: include_bytes!("../../../../.dfx/local/canisters/swap/swap.wasm").to_vec(),
        arg: vec![],  // Optional: Arguments for the canister init method
    };

    let result: Result<(), (ic_cdk::api::call::RejectionCode, String)> =
        call(Principal::management_canister(), "install_code", (install_code_args,)).await;

    match result {
        Ok(_) => Ok(()),
        Err((_, err_msg)) => Err(err_msg),
    }
}

// update to store all pool data

#[update]
async fn add_liquidity(params: Pool_Data, canister_id: Principal) -> Result<(), String> {
    let pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    // let key = format!("{},{}", pool_name, params.swap_fee);

    let result: Result<(), String> =
        call(canister_id, "store_data_inpool", (api::caller(), params))
            .await
            .map_err(|e| format!("Failed to add liquidity: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }
    Ok(())
}

// Adding liquidity to the specific pool


#[update]
fn store_pool_data_curr(params: Pool_Data) -> Result<(), String> {
    let key = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    POOL_DATA.with(|pool| {
        let mut borrowed_pool = pool.borrow_mut();
        let liquidity = params.clone();
        borrowed_pool
            .entry(key)
            .or_insert_with(Vec::new)
            .push(params.clone());
    });
    Ok(())
}

#[query]
fn get_pool_data() -> BTreeMap<String, Vec<Pool_Data>> {
    POOL_DATA.with(|pool| pool.borrow().clone())
}

#[query]
fn get_specific_pool_data(key: String) -> Result<Vec<Pool_Data>, String> {
    POOL_DATA.with(|pool| {
        let borrored_pool = pool.borrow();
        if let Some(pool_data) = borrored_pool.get(&key) {
            Ok(pool_data.clone())
        } else {
            Err("Pool not found".to_string())
        }
    })
}

#[update]
fn add_liquidity_curr(params: Pool_Data) -> Result<(), String> {
    let key = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    POOL_DATA.with(|pool| {
        let mut borrowed_pool = pool.borrow_mut();

        if let Some(existing_pool_data_vec) = borrowed_pool.get_mut(&key) {
            let mut fee_matched = false;

            for existing_pool_data in existing_pool_data_vec.iter_mut() {
                if (existing_pool_data.swap_fee - params.swap_fee).abs() < f64::EPSILON {
                    fee_matched = true;

                    for new_token in &params.pool_data {
                        if let Some(existing_token) = existing_pool_data
                            .pool_data
                            .iter_mut()
                            .find(|token| token.token_name == new_token.token_name)
                        {
                            existing_token.balance += new_token.balance;
                        }
                    }
                }
            }

            if !fee_matched {
                existing_pool_data_vec.push(params);
            }
        } else {
            borrowed_pool.insert(key, vec![params]);
        }
    });

    Ok(())
}

// take swap elements in the vector
#[update]
fn search_swap_pool(params: SwapParams) -> Result<Vec<String>, String> {
    let mut search_tokens = Vec::new();
    search_tokens.push(params.token1_name);
    search_tokens.push(params.token2_name);

    POOL_DATA.with(|pool| {
        let borrowed_pool = pool.borrow();
        let mut matching_keys = Vec::new();

        for key in borrowed_pool.keys() {
            // Check if all search tokens are present in the key
            if search_tokens.iter().all(|token| key.contains(token)) {
                matching_keys.push(key.clone());
            }
        }

        if !matching_keys.is_empty() {
            Ok(matching_keys)
        } else {
            Err("No matching pools found.".to_string())
        }
    })
}

#[query]
fn pre_compute_swap(params: SwapParams) -> (String, f64) {
    let required_pools = match search_swap_pool(params.clone()) {
        Ok(pools) => pools,
        Err(_) => {
            ic_cdk::println!("No matching pools found.");
            return ("No matching pools found.".to_string(), 0.0);
        }
    };

    let mut best_pool = None;
    let mut max_output_amount = 0.0;

    POOL_DATA.with(|pool_data| {
        let pool_data = pool_data.borrow();

        for pool_key in required_pools {
            let pool_entries = match pool_data.get(&pool_key) {
                Some(entries) => entries,
                None => {
                    ic_cdk::println!("Pool key {} not found in POOL_DATA.", pool_key);
                    continue;
                }
            };

            for data in pool_entries {
                // Find the tokenA (input) and tokenB (output) from the pool data
                let tokenA_data = data
                    .pool_data
                    .iter()
                    .find(|p| p.token_name == params.token1_name);
                let tokenB_data = data 
                    .pool_data
                    .iter()
                    .find(|p| p.token_name == params.token2_name);

                ic_cdk::println!(
                    "Testing pool_key {} with tokenA_data: {:?}, tokenB_data: {:?}",
                    pool_key,
                    tokenA_data,
                    tokenB_data
                );

                if let (Some(tokenA), Some(tokenB)) = (tokenA_data, tokenB_data) {
                    let b_i = tokenA.balance as f64;
                    let w_i = tokenA.weight as f64;
                    let b_o = tokenB.balance as f64;
                    let w_o = tokenB.weight as f64;

                    let amount_out = params.token_amount as f64;
                    let fee = data.swap_fee;

                    // Calculate the required input using the in_given_out formula
                    let required_input = in_given_out(b_i, w_i, b_o, w_o, amount_out, fee);
                    // ic_cdk::println!("The required output is {:?}", required_input);

                    // Ensure the user has enough balance to provide the input
                    if required_input >= max_output_amount {
                        max_output_amount = f64::max(required_input, max_output_amount);

                        best_pool = Some(pool_key.clone());
                        // Check if the current pool gives a better output
                        // if calculated_output > max_output_amount {
                        //     max_output_amount = calculated_output;
                        // }
                    }
                } else {
                    ic_cdk::println!("Either tokenA or tokenB was not found in pool.");
                }
            }
        }
    });

    match best_pool {
        Some(pool) => (pool, max_output_amount),
        None => ("No suitable pool found.".to_string(), 0.0),
    }
}

// Adding liquidity to the specific pool
#[update]
async fn store_pool_data(params: Pool_Data, canister_id: Principal) -> Result<(), String> {
    // Call the canister's add_liquidity function with the provided data
    let result: Result<(), String> = call(
        canister_id,
        "add_liquidity_to_pool",
        (api::caller(), params),
    )
    .await
    .map_err(|e| format!("Failed to store token data: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }

    Ok(())
}

#[query]
fn get_pool_canister_id(token1 : String , token2 : String) -> Option<Principal>{
    let mut pool_name =format!("{}{}",token1,token2);
    let canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        // Extract the principal if available
        pool_borrowed.get(&pool_name).map(|user_principal| user_principal.principal)
    });
    canister_id
}

#[update]
async fn compute_swap(params: SwapParams) -> Result<(), String> {
    let (pool_name, _) = pre_compute_swap(params.clone());
    let (_ , amount) = pre_compute_swap(params.clone());

    if pool_name == "No suitable pool found.".to_string()
        || pool_name == "No matching pools found.".to_string()
    {
        return Err(pool_name);
    }

    let canister_id = with_state(|pool| {
        let mut pool_borrowed = &mut pool.TOKEN_POOLS;
        // Extract the principal if available
        pool_borrowed.get(&pool_name).map(|user_principal| user_principal.principal)
    });

    let canister_id = match canister_id {
        Some(id) => id,
        None => return Err("No canister ID found for the pool".to_string()),
    };
  
    ic_cdk::println!("swap pool canister's canister_id {:}",canister_id.clone());

    // let amount_as_u64 = amount as u64;
    // deposit_tokens(amount_as_u64.clone(), ledger_canister_id, canister_id.clone());

    // let user_principal_id = api::caller();

    deposit_tokens(params.token_amount.clone(), params.ledger_canister_id.clone(), canister_id.clone()).await?;

    // ic_cdk::println!("pool canister ka canister ID{:}", canister_id.clone());
    // Proceed with the call using the extracted principal
    let result: Result<(), String> = call(
        canister_id,
        "swap",
        (api::caller(),params.clone() , amount),
    )
    .await
    .map_err(|e| format!("Failed to perform swap: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }

    // deposit_tokens(params.token_amount.clone(), params.ledger_canister_id2.clone(), canister_id.clone()).await?;
    Ok(())
}

// if (data.swap_fee - params.swap_fee).abs() > f64::EPSILON {
//     continue;
// }

// if (data.swap_fee - params.swap_fee).abs() > f64::EPSILON {
//     continue;
// }

