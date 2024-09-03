use crate::{user_principal, with_state};
use candid::{CandidType, Nat, Principal};
use ic_cdk::api::management_canister::bitcoin::BitcoinNetwork;
use ic_cdk_macros::*;
use serde::de::value;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
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

use crate::api::deposit::deposit_ckbtc;
use crate::types::state_handlers;
use crate::utils::types::*;

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
// #[update]
async fn create_pools(params: Pool_Data) -> Result<(), String> {
    let principal_id = api::caller();
    if principal_id == Principal::anonymous() {
        return Err("Anonymous principal not allowed to make calls".to_string());
    }

    let pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    // let pool_key = arrange_key(pool_name);

    // let pool_key = format!("{}{}", pool_name, params.swap_fee);

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
        add_liquidity(params.clone() , canister_id.principal);
        //TODO Map canister id with pool_key for adding liquidity
        Ok(())
    } else {
        match create_canister(CreateCanisterArgument { settings: None }).await {
            Ok((canister_id_record,)) => {
                let canister_id = canister_id_record.canister_id;
                with_state(|pool| {
                    // pool.    borrow_mut().insert(pool_key.clone(), canister_id);
                    &mut pool.TOKEN_POOLS.insert(
                        pool_name.clone(),
                        crate::user_principal {
                            principal: canister_id,
                        },
                    );
                });

                // POOL.with(|pool| {
                //     let mut pool_borrowed = pool.borrow_mut();
                //     let token_map = pool_borrowed.entry(pool_key).or_insert_with(HashMap::new);
                //     for (token, value) in params.token_names.iter().zip(params.balances.iter()) {
                //         token_map.insert(token.clone(), *value);
                //     }
                // });
                // store_pool_data(params , canister_id.principal);
                store_pool_data_curr(params.clone());
                store_pool_data(params.clone(), canister_id_record.canister_id).await?;

                for amount in params.pool_data.iter() {
                    deposit_ckbtc(amount.balance.clone()).await?;
                }

                // params.pool_data
                // .iter()
                // .map(|pool|pool.balance.clone())
                // .deposit_ckbtc().await?;

                Ok(())
            }
            Err((_, err_string)) => Err(format!("Error creating canister: {}", err_string)),
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
    let cycles: u128 = 100_000_000_000;

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
        include_bytes!("/home/nikhil27/valueswap/.dfx/local/canisters/swap/swap.wasm")
            .to_vec();
       
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
pub async fn create() -> Result<String, String> {
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
        match deposit_cycles(canister_id_record, 100_000_000).await {
            Ok(_) => Ok(()),
            Err((_, err_string)) => {
                ic_cdk::println!("Error in depositing cycles: {}", err_string);
                return Err(format!("Error: {}", err_string));
            }
        };

    let arg1 = InstallCodeArgument {
        mode: CanisterInstallMode::Install,
        canister_id,
        wasm_module: vec![], // Placeholder, should be the actual WASM module bytes if needed
        arg: Vec::new(),
    };

    let _install_code: Result<(), String> = match install_code(arg1).await {
        Ok(_) => Ok(()),
        Err((_, err_string)) => {
            ic_cdk::println!("Error in installing code: {}", err_string);
            return Err(format!("Error: {}", err_string));
        }
    };

    ic_cdk::println!("Canister ID: {:?}", canister_id.to_string());
    Ok(format!("Canister ID: {}", canister_id.to_string()))
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

    let result: Result<(), String> = call(
        canister_id, 
        "store_data_inpool",
        (api::caller(), params)
    )
        .await
        .map_err(|e| format!("Failed to add liquidity: {:?}", e));

    if let Err(e) = result {
        return Err(e);
    }
    Ok(())
}

// Adding liquidity to the specific pool
#[update]
async fn store_pool_data( params: Pool_Data , canister_id: Principal) -> Result<(), String> {
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

#[update]
fn store_pool_data_curr(params : Pool_Data)-> Result<(), String>  {
    let key = params
    .pool_data
    .iter()
    .map(|pool| pool.token_name.clone())
    .collect::<Vec<String>>()
    .join("");

    POOL_DATA.with(|pool|{
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
    POOL_DATA.with(|pool|{
        pool.borrow().clone()
    })
}

fn get_specific_pool_data(key : String) -> Result<Vec<Pool_Data>, String> {
    POOL_DATA.with(|pool|{
        let borrored_pool = pool.borrow();
        if let Some(pool_data) = borrored_pool.get(&key) {
            Ok(pool_data.clone())
        }else{
            Err("Pool not found".to_string())
        }
    })
}

#[update]
fn add_liquidity_curr(params : Pool_Data) -> Result<() , String> {
    let key = params
    .pool_data
    .iter()
    .map(|pool|pool.token_name.clone())
    .collect::<Vec<String>>()
    .join("");

    POOL_DATA.with(|pool|{
        let mut borrowed_pool = pool.borrow_mut();
        
        if let Some(existing_pool_data_vec) = borrowed_pool.get_mut(&key){

            for new_token in &params.pool_data{
                for existing_pool_data in existing_pool_data_vec.iter_mut() {
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
    });
    Ok(())
    
}


// #[update]
// fn search_swap_pool() -> Result<Vec<String> , String>{
      
// }


// #[query]
// fn pre_compute_swap(){

// }

// #[query]
// fn get_pool_data(pool_id: Principal) -> Result<Option<TokenData>, String> {
//     let data = POOL.with(|pool| {
//         pool.borrow().values().find(|&data| data.user_id == pool_id).cloned()
//     });
//     Ok(data)
// }

