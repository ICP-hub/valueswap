use crate::with_state;
use candid::{ Nat, Principal};
use core::panic;
use ic_cdk_macros::*;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::sync::Mutex;
use core::cmp::max;
use once_cell::sync::Lazy;
use ic_cdk::{
    api,
    api::{
        call::{call_with_payment128, CallResult},
        canister_version,
        management_canister::main::CanisterInstallMode,
    },
    call,
};

use crate::api::balance::*;
use crate::api::deposit::deposit_tokens;
use crate::utils::maths::*;
use crate::utils::types::*;
use crate::vault::lp_tokens::*;

static LOCKS: Lazy<Mutex<HashMap<String, bool>>> = Lazy::new(|| Mutex::new(HashMap::new()));

thread_local! {
    pub static POOL_DATA: RefCell<BTreeMap<String, Vec<Pool_Data>>> = RefCell::new(BTreeMap::new());
}

fn prevent_anonymous() -> Result<(), String> {
    if api::caller() == Principal::anonymous() {
        Err("Anonymous access not allowed".to_string())
    } else {
        Ok(())
    }
}

//TODO 12 intercanister call can fail due to n number of reasons handle erros effectively with enums and custom erors
//TODO 13 Add validations for variables
//TODO 14 Add Test cases
//TODO 16 ADD limit of 30% for swapping
//TODO 15 ADD rate limiting for cycles  --PHASE 2
//TODO 17 Reguarly check cycles balance to preves DOS  ---PHASE 2

#[update(guard = prevent_anonymous)]
async fn create_pools(params: Pool_Data) -> Result<(), CustomError> {
    if params.pool_data.is_empty() {
        return Err(CustomError::PoolDataEmpty);
    }

    let principal_id = ic_cdk::caller();
    let pool_name = params
        .pool_data
        .iter()
        .map(|pool| pool.token_name.clone())
        .collect::<Vec<String>>()
        .join("");

    // Acquire lock for the pool
    {
        let mut locks = LOCKS
            .lock()
            .map_err(|_| CustomError::LockAcquisitionFailed)?;
        if locks.get(&pool_name).copied().unwrap_or(false) {
            return Err(CustomError::AnotherOperationInProgress(pool_name));
        }
        locks.insert(pool_name.clone(), true);
    }

    // Ensure lock is released after operation
    let release_lock = || {
        let mut locks = LOCKS.lock().unwrap_or_else(|e| {
            panic!("Failed to unlock LOCKS: {}", e);
        });
        locks.remove(&pool_name);
    };

    let result = async {
        let pool_canister_id = with_state(|pool| {
            let pool_borrowed = &pool.TOKEN_POOLS;
            pool_borrowed.get(&pool_name).clone()
        });

        if let Some(canister_id) = pool_canister_id {
            // Add liquidity and update pool details
            add_liquidity_curr(params.clone());
            add_liquidity(params.clone(), canister_id.principal.clone());

            increase_pool_lp_tokens(params.clone());
            users_pool(params.clone());
            store_pool_data(params.clone(), canister_id.principal)
                .await
                .map_err(|e| CustomError::UnableToStorePoolData(e));

            users_lp_share(principal_id.clone(), params.clone())
                .await
                .map_err(|e| CustomError::UnableToTransferLP(e));

            for amount in params.pool_data.iter() {
                deposit_tokens(
                    amount.balance.clone(),
                    amount.ledger_canister_id.clone(),
                    canister_id.principal,
                )
                .await
                .map_err(|_| CustomError::TokenDepositFailed)?;
            }
            Ok(())
        } else {
            // Create a new canister for the pool
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

                    store_pool_data(params.clone(), canister_id_record)
                        .await
                        .map_err(|e| CustomError::UnableToStorePoolData(e));

                    increase_pool_lp_tokens(params.clone());
                    users_pool(params.clone());

                    users_lp_share(principal_id.clone(), params.clone())
                        .await
                        .map_err(|e| CustomError::UnableToTransferLP(e));

                    for amount in params.pool_data.iter() {
                        deposit_tokens(
                            amount.balance.clone(),
                            amount.ledger_canister_id.clone(),
                            canister_id,
                        )
                        .await
                        .map_err(|_| CustomError::TokenDepositFailed)?;
                    }

                    Ok(())
                }
                Err(err_string) => Err(CustomError::CanisterCreationFailed(err_string)),
            }
        }
    }
    .await;

    // Release the lock
    release_lock();

    result
}

#[update]
// Create canister
async fn create_canister(arg: CreateCanisterArgument) -> CallResult<(CanisterIdRecord,)> {
    let extended_arg = CreateCanisterArgumentExtended {
        settings: arg.settings,
        sender_canister_version: Some(canister_version()),
    };
    let cycles: u128 = 500_000_000_000;

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
        match deposit_cycles(canister_id_record, 500_000_000_000).await {
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
        Err(_) => {
            panic!("Not able to install code");
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
        arg: vec![], // Optional: Arguments for the canister init method
    };

    let result: Result<(), (ic_cdk::api::call::RejectionCode, String)> = call(
        Principal::management_canister(),
        "install_code",
        (install_code_args,),
    )
    .await;

    match result {
        Ok(_) => Ok(()),
        Err((_, err_msg)) => Err(err_msg),
    }
}

// update to store all pool data

#[update]
async fn add_liquidity(params: Pool_Data, canister_id: Principal) -> Result<(), String> {
    let _pool_name = params
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
                if (existing_pool_data.swap_fee.clone() - params.swap_fee.clone())
                    < Nat::from(0u128)
                {
                    fee_matched = true;

                    for new_token in &params.pool_data {
                        if let Some(existing_token) = existing_pool_data
                            .pool_data
                            .iter_mut()
                            .find(|token| token.token_name == new_token.token_name)
                        {
                            existing_token.balance += new_token.balance.clone();
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

// #[update]
// async fn pre_compute_swap(params: SwapParams) -> (String, Nat) {

//     let required_pools = match search_swap_pool(params.clone()) {
//         Ok(pools) => pools,
//         Err(_) => {
//             ic_cdk::println!("No matching pools found.");
//             return ("No matching pools found.".to_string(), Nat::from(0u128));
//         }
//     };

//     let mut best_pool = None;
//     let mut max_output_amount:Nat = Nat::from(0u128);

//     // Move the POOL_DATA closure logic outside of the async block
//     let pool_data = POOL_DATA.with(|pool| pool.borrow().clone());

//     for pool_key in required_pools {
//         let pool_entries = match pool_data.get(&pool_key) {
//             Some(entries) => entries,
//             None => {
//                 ic_cdk::println!("Pool data {} not found in pool key.", pool_key);
//                 continue;
//             }
//         };

//         for data in pool_entries {
//             // Find the tokenA (input) and tokenB (output) from the pool data
//             let tokenA_data = data
//                 .pool_data
//                 .iter()
//                 .find(|p| p.token_name == params.token1_name);
//             let tokenB_data = data
//                 .pool_data
//                 .iter()
//                 .find(|p| p.token_name == params.token2_name);

//             let pool_name = data.pool_data.iter().map(|pool| pool.token_name.clone())
//                 .collect::<Vec<String>>()
//                 .join("");

//             if let (Some(tokenA), Some(tokenB)) = (tokenA_data, tokenB_data) {
//                 let w_i = tokenA.weight.clone();
//                 let w_o = tokenB.weight.clone();
//                 let amount_out = params.token_amount.clone();
//                 let fee = data.swap_fee.clone();

//                 // Fetch the pool canister ID asynchronously
//                 let pool_canister_id = with_state(|pool| {
//                     let borrowed_pool = pool.TOKEN_POOLS.borrow();
//                     borrowed_pool.get(&pool_name).map(|user_principal| user_principal.principal)
//                 });

//                 let pool_canister_id = match pool_canister_id {
//                     Some(id) => id,
//                     None => {
//                         ic_cdk::println!("Pool key {} not found in POOL_DATA.", pool_key);
//                         continue;
//                     },
//                 };
//                 // Fetch balances asynchronously
//                 let b_i = icrc_get_balance(tokenA.ledger_canister_id, pool_canister_id).await.unwrap();
//                 let b_o = icrc_get_balance(tokenB.ledger_canister_id, pool_canister_id).await.unwrap();

//                 // let b_i_f128 = convert_nat_to_u64(b_i).unwrap();
//                 // let b_o_f128 = convert_nat_to_u64(b_o).unwrap();

//                 //  let b_i = b_i_f128/10000000.0;
//                 //  let b_o = b_o_f128/10000000.0;

//                 ic_cdk::println!("The balance of First token is {}{}", b_i.clone(),b_o.clone());

//                 // Calculate the required input using the out_given_in formula
//                 let required_input = out_given_in(b_i, w_i , b_o, w_o, amount_out, fee);

//                 // Ensure the user has enough balance to provide the input
//                 if required_input >= max_output_amount {
//                     max_output_amount = max(required_input, max_output_amount);
//                     best_pool = Some(pool_key.clone());
//                 }
//             } else {
//                 ic_cdk::println!("Either tokenA or tokenB was not found in pool.");
//             }
//         }
//     }

//     match best_pool {
//         Some(pool) => (pool, max_output_amount),
//         None => ("No suitable pool found.".to_string(), Nat::from(0u128)),
//     }
// }

#[update]
async fn pre_compute_swap(params: SwapParams) -> (String, Nat) {
    // Search for required pools based on the provided parameters

    let required_pools = match search_swap_pool(params.clone()) {
        Ok(pools) => pools,
        Err(_) => {
            println!("No matching pools found.");
            return ("No matching pools found.".to_string(), Nat::from(0u128));
        }
    };

    let mut best_pool = None;
    let mut max_output_amount: Nat = Nat::from(0u128);

    let pool_data = POOL_DATA.with(|pool| pool.borrow().clone());



    // Collect async tasks for fetching balances
    let mut fetch_tasks = vec![];

    for pool_key in required_pools {
        if let Some(pool_entries) = pool_data.get(&pool_key) {
            for data in pool_entries {
                let tokenA_data = data
                    .pool_data
                    .iter()
                    .find(|p| p.token_name == params.token1_name);
                let tokenB_data = data
                    .pool_data
                    .iter()
                    .find(|p| p.token_name == params.token2_name);

                let pool_name = data
                    .pool_data
                    .iter()
                    .map(|pool| pool.token_name.clone())
                    .collect::<Vec<String>>()
                    .join("");

                if let (Some(tokenA), Some(tokenB)) = (tokenA_data, tokenB_data) {
                    let w_i = tokenA.weight.clone();
                    let w_o = tokenB.weight.clone();
                    let amount_out = params.token_amount.clone();
                    let fee = data.swap_fee.clone();

                    let pool_canister_id = with_state(|pool| {
                        let borrowed_pool = pool.TOKEN_POOLS.borrow();
                        borrowed_pool
                            .get(&pool_name)
                            .map(|user_principal| user_principal.principal)
                    });

                    if let Some(pool_canister_id) = pool_canister_id {
                        let pool_key_clone = pool_key.clone(); // Clone for async block
                        fetch_tasks.push(async move {
                            let b_i = icrc_get_balance(tokenA.ledger_canister_id, pool_canister_id)
                                .await
                                .unwrap_or_default();
                            let b_o = icrc_get_balance(tokenB.ledger_canister_id, pool_canister_id)
                                .await
                                .unwrap_or_default();
                            (pool_key_clone, b_i, b_o, w_i, w_o, amount_out, fee)
                        });
                    } else {
                        println!("Pool key {} not found in POOL_DATA.", pool_key);
                    }
                } else {
                    println!("Either tokenA or tokenB was not found in pool.");
                }
            }
        } else {
            println!("Pool data {} not found in pool key.", pool_key);
        }
    }

    // Wait for all balance-fetching tasks to complete
    let results = futures::future::join_all(fetch_tasks).await;

    for (pool_key, b_i, b_o, w_i, w_o, amount_out, fee) in results {
        // Calculate the required input using the out_given_in formula
        let required_input = out_given_in(b_i.clone(), w_i, b_o.clone(), w_o, amount_out, fee);

        println!(
            "Pool {}: b_i = {}, b_o = {}, required_input = {}",
            pool_key, b_i, b_o, required_input
        );

        // Update the best pool if the required input is better
        if required_input >= max_output_amount {
            max_output_amount = max(required_input, max_output_amount);
            best_pool = Some(pool_key);
        }
    }

    match best_pool {
        Some(pool) => (pool, max_output_amount),
        None => ("No suitable pool found.".to_string(), Nat::from(0u128)),
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
fn get_pool_canister_id(token1: String, token2: String) -> Option<Principal> {
    let pool_name = format!("{}{}", token1, token2);
    let canister_id = with_state(|pool| {
        let  pool_borrowed = &mut pool.TOKEN_POOLS;
        // Extract the principal if available
        pool_borrowed
            .get(&pool_name)
            .map(|user_principal| user_principal.principal)
    });
    canister_id
}


// TODO 18 assign unique id for each swap PHASE 2
// #[update]
// async fn compute_swap(params: SwapParams) -> Result<(), CustomError> {
//     let user = ic_cdk::caller().to_string();

//     {
//         let mut locks = LOCKS
//             .lock()
//             .map_err(|_| CustomError::LockAcquisitionFailed)?;
//         if locks.get(&user).copied().unwrap_or(false) {
//             return Err(CustomError::AnotherOperationInProgress(user));
//         }
//         locks.insert(user.clone(), true);
//     }

//     // Ensure lock is released after operation
//     let release_lock = || {
//         let mut locks = LOCKS.lock().unwrap_or_else(|e| {
//             panic!("Failed to unlock LOCKS: {}", e);
//         });
//         locks.remove(&user);
//     };

//     let (pool_name, _) = pre_compute_swap(params.clone()).await;
//     let (_, amount) = pre_compute_swap(params.clone()).await;

//     if pool_name == "No suitable pool found.".to_string()
//         || pool_name == "No matching pools found.".to_string()
//     {
//         return Err(CustomError::NoCanisterIDFound);
//     }

//     let canister_id = with_state(|pool| {
//         let pool_borrowed = &mut pool.TOKEN_POOLS;
//         // Extract the principal if available
//         pool_borrowed
//             .get(&pool_name)
//             .map(|user_principal| user_principal.principal)
//     });

//     let canister_id = match canister_id {
//         Some(id) => id,
//         None => return Err(CustomError::NoCanisterIDFound),
//     };

//     ic_cdk::println!("swap pool canister's canister_id {:}", canister_id.clone());

//     // let amount_as_u64 = amount as u64;
//     // deposit_tokens(amount_as_u64.clone(), ledger_canister_id, canister_id.clone());

//     // let user_principal_id = api::caller();
//     let token_amount = params.token_amount.clone();

//     let result = deposit_tokens(
//         token_amount.clone(),
//         params.ledger_canister_id1.clone(),
//         canister_id.clone(),
//     )
//     .await;

//     match result {
//         Ok(_) => {
//             println!("Token deposit successful, resuming process...");

//             let result: Result<(), String> = call(
//                 canister_id.clone(),
//                 "swap",
//                 (api::caller(), params.clone(), amount),
//             )
//             .await
//             .map_err(|e| format!("Failed to perform swap: {:?}", e));

//             if let Err(e) = result {
//                 return Err(CustomError::SwappingFailed(e));
//             }
//         }
//         Err(err) => {
//             println!(
//                 "Error during token deposit: {:?}. Initiating rollback...",
//                 err
//             );
//             // rollback code here

//             let _result: Result<(), String> = call(
//                 canister_id,
//                 "icrc1_transfer",
//                 (canister_id, api::caller(), token_amount),
//             )
//             .await
//             .map_err(|e| format!("Failed to perform rollback: {:?}", e));
//         }
//     }

//     // ic_cdk::println!("pool canister ka canister ID{:}", canister_id.clone());
//     // Proceed with the call using the extracted principal
//     // let result: Result<(), String> = call(
//     //     canister_id,
//     //     "swap",
//     //     (api::caller(),params.clone() , amount),
//     // )
//     // .await
//     // .map_err(|e| format!("Failed to perform swap: {:?}", e));

//     // if let Err(e) = result {
//     //     return Err(e);
//     // }
//     release_lock();

//     // deposit_tokens(params.token_amount.clone(), params.ledger_canister_id2.clone(), canister_id.clone()).await?;
//     Ok(())
// }

// if (data.swap_fee - params.swap_fee).abs() > f64::EPSILON {
//     continue;
// }

// if (data.swap_fee - params.swap_fee).abs() > f64::EPSILON {
//     continue;
// }





#[update]
async fn compute_swap(params: SwapParams) -> Result<(), CustomError> {
    let user = ic_cdk::caller().to_string();
    // let transaction_id = format!("{}_{}", user, uuid::Uuid::new_v4()); 

    {
        let mut locks = LOCKS
            .lock()
            .map_err(|_| CustomError::LockAcquisitionFailed)?;
        if locks.get(&user).copied().unwrap_or(false) {
            return Err(CustomError::AnotherOperationInProgress(user));
        }
        locks.insert(user.clone(), true);
    }

    let release_lock = || {
        let mut locks = LOCKS.lock().unwrap_or_else(|e| {
            panic!("Failed to unlock LOCKS: {}", e);
        });
        locks.remove(&user);
    };

    let result = async {
        let (pool_name, _) = pre_compute_swap(params.clone()).await;
        let (_, amount) = pre_compute_swap(params.clone()).await;

        if pool_name == "No suitable pool found.".to_string()
            || pool_name == "No matching pools found.".to_string()
        {
            return Err(CustomError::NoCanisterIDFound);
        }

        let canister_id = with_state(|pool| {
            pool.TOKEN_POOLS
                .get(&pool_name)
                .map(|user_principal| user_principal.principal)
        })
        .ok_or(CustomError::NoCanisterIDFound)?;

        ic_cdk::println!("Swap pool canister's ID: {}", canister_id);

        deposit_tokens(
            params.token_amount.clone(),
            params.ledger_canister_id1.clone(),
            canister_id.clone(),
        )
        .await
        .map_err(|_| CustomError::TokenDepositFailed)?;

        let swap_result: Result<(), String> = call(
            canister_id.clone(),
            "swap",
            (api::caller(), params.clone(), amount),
        )
        .await
        .map_err(|e| format!("Failed to perform swap: {:?}", e));

        if let Err(e) = swap_result {
            let rollback_result: Result<(), String> = call(
                canister_id,
                "icrc1_transfer",
                (canister_id, api::caller(), params.token_amount.clone()),
            )
            .await
            .map_err(|e| format!("Failed to perform rollback: {:?}", e));

            if rollback_result.is_err() {
                ic_cdk::println!("Rollback failed: {:?}", rollback_result);
            }

            return Err(CustomError::SwappingFailed(e));
        }

        Ok(())
    }
    .await;

    release_lock();

    result
}
