mod utils;

use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use serde::Deserialize;
use utils::structs::{
    Account, ApproveArgs, ApproveError, ArchiveOptions, CreatePoolParams, CustomError,
    FeatureFlags, LedgerArgument, Pool_Data,
};

use crate::utils::structs::InitArgs;
use std::fs;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";

pub fn get_user_principal() -> Principal {
    Principal::from_text("4jwha-xpj7p-sk2lp-bdo4u-cijhx-xskuu-qj34g-kqty4-n6jhy-ixgjd-aqe").unwrap()
}

#[test]
fn call_test_function() {
    let (pic, backend_canister, ckbtc_canister) = setup();
    test_rollback_on_invalid_data(&pic, backend_canister, ckbtc_canister,);
    // test_rollback_on_token_deposit_failure(&pic, backend_canister, ckbtc_canister);
    // test_rollback_on_lp_share_failure(&pic, backend_canister, ckbtc_canister);
    // test_rollback_on_add_liquidity_curr_failure(&pic, backend_canister, ckbtc_canister);
    // test_rollback_on_add_liquidity_failure(&pic, backend_canister, ckbtc_canister);
    // test_rollback_on_lock_handling_failure(&pic, backend_canister,ckbtc_canister);
    // test_rollback_on_store_pool_data_failure(&pic, backend_canister,ckbtc_canister);
}

fn setup() -> (PocketIc, Principal, Principal) {
    ic_cdk::println!("Setting up Pocket IC...");
    // std::env::set_var(
    //     "POCKET_IC_BIN",
    //     "/home/jyotirmay1789/valueswap/src/valueswap_backend/pocket-ic",
    // ); // Path of the pocket-ic binary

    let pic = PocketIc::new();
    ic_cdk::println!("Pocket IC setup complete.");

    let backend_canister = pic.create_canister();
    ic_cdk::println!("backend canister = {}",backend_canister.to_text());
    pic.add_cycles(backend_canister, 2_000_000_000_000_000); // 2T Cycles
    let backend_wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, backend_wasm, vec![], None);

    let ckbtc_canister = pic.create_canister();
    pic.add_cycles(ckbtc_canister, 2_000_000_000_000_000); // 2T Cycles
    let ckbtc_wasm = fs::read(CKBTC_WASM).expect("Wasm file not found, run 'dfx build'.");

    let args = InitArgs {
        token_symbol: String::from("CKBTC"),
        token_name: String::from("CKBTC"),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![(
            Account {
                owner: Principal::from_text(
                    "4jwha-xpj7p-sk2lp-bdo4u-cijhx-xskuu-qj34g-kqty4-n6jhy-ixgjd-aqe",
                )
                .unwrap(),
                subaccount: None,
            },
            Nat::from(10_000_000_000u128),
        )],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            max_transactions_per_response: None,
            trigger_threshold: 500,
            more_controller_ids: None,
            max_message_size_bytes: None,
            cycles_for_archive_creation: Some(1_000_000),
            node_max_memory_size_bytes: None,
            controller_id: Principal::anonymous(),
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    };

    let args_encoded =
        encode_args((LedgerArgument::Init(args),)).expect("Failed to encode arguments");

    pic.install_canister(ckbtc_canister, ckbtc_wasm, args_encoded, None);
    println!("CKBTC canister: {}", ckbtc_canister);

    (pic, backend_canister, ckbtc_canister)
}

fn icrc2_approve(pic: &PocketIc, backend_canister: Principal, ckbtc_canister: Principal) {
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(100_000_000u128),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: backend_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            get_user_principal(),
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(
                        allowance > Nat::from(0u64),
                        "Allowance should be greater than zero."
                    );
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }
}

fn test_rollback_on_invalid_data(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal
) {
    #[derive(Debug, Clone, CandidType, Deserialize)]
    struct CreatePoolParams {
        token_name: String,
        balance: Nat,
        weight: Nat,
        value: Nat,
        ledger_canister_id: Principal,
        image: String,
    }

    #[derive(Debug, Clone, CandidType, Deserialize)]
    struct PoolData {
        pool_data: Vec<CreatePoolParams>,
        swap_fee: Nat,
    }

    #[derive(Debug, Clone)]
    struct TestCase {
        pool_data: PoolData,
        expect_success: bool,
        expected_error_message: Option<String>,
    }

    let test_cases: Vec<TestCase> = vec![
        // ✅ Valid Pool Data Test Case
        TestCase {
            pool_data: PoolData {
                pool_data: vec![CreatePoolParams {
                    token_name: "FaultyToken".to_string(),
                    balance: Nat::from(600u128),
                    weight: Nat::from(10u128),
                    value: Nat::from(600u128),
                    ledger_canister_id: ckbtc_canister,
                    image: "invalid_image.png".to_string(),
                }],
                swap_fee: Nat::from(5u128),
            },
            expect_success: true,
            expected_error_message: None,
        },

        // ❌ Invalid Pool Data Test Case (should trigger rollback)
        // TestCase {
        //     pool_data: PoolData {
        //         pool_data: vec![CreatePoolParams {
        //             token_name: "bothTokens".to_string(),
        //             balance: Nat::from(100u128),
        //             weight: Nat::from(10u128),
        //             value: Nat::from(0u128), // ❌ Invalid: Zero value
        //             ledger_canister_id: ckbtc_canister,
        //             image: "invalid_image.png".to_string(),
        //         }],
        //         swap_fee: Nat::from(5u128),
        //     },
        //     expect_success: false,
        //     expected_error_message: Some("Invalid value: value cannot be zero.".to_string()),
        // },
    ];

    let hardcoded_principal = get_user_principal();

    // 🔄 Looping through test cases
    for (i, case) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n------------------------------------------------------------");
        ic_cdk::println!("🔵 IC Test Case {}: Executing Rollbacks Request", i + 1);
        ic_cdk::println!("------------------------------------------------------------\n");

        icrc2_approve(pic, backend_canister, ckbtc_canister);

        // Encode arguments
        let encoded_args = candid::encode_args((&case.pool_data,)).unwrap();

        // Make update call
        let response = pic
            .update_call(backend_canister, hardcoded_principal, "create_pools", encoded_args)
            .unwrap();

        // 🔍 Check response
        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                ic_cdk::println!("what is result = {:?}",result);
                
                if case.expect_success {
                    assert!(
                        result.is_ok(),
                        "❌ Test {} failed: Expected success, but got an error {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                    ic_cdk::println!("✅ Test {} passed! Pool created successfully.", i + 1);
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected rollback, but got success.",
                        i + 1
                    );
                    ic_cdk::println!(
                        "✅ Test {} passed! Rollback triggered as expected. Error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                }
            }
            WasmResult::Reject(message) => {
                if case.expect_success {
                    ic_cdk::println!("❌ Test {} failed: Unexpected rejection: {}", i + 1, message);
                } else {
                    ic_cdk::println!(
                        "✅ Test {} passed! Rollback triggered due to rejection: {}",
                        i + 1,
                        message
                    );
                }
            }
        }
    }
}


// fn test_rollback_on_invalid_data(
//     pic: &PocketIc,
//     backend_canister: Principal,
//     ckbtc_canister: Principal,
// ) {
//     #[derive(Debug, Clone)]
//     struct CreatePoolParams {
//         token_name: String,
//         balance: Nat,
//         weight: Nat,
//         value: Nat,
//         ledger_canister_id: Principal,
//         image: String,
//     }

//     #[derive(Debug, Clone)]
//     struct PoolData {
//         pool_data: Vec<CreatePoolParams>,
//         swap_fee: Nat,
//     }

//     #[derive(Debug, Clone)]
//     struct TestCase {
//         pool_data: PoolData,
//         expect_success: bool,
//         expected_error_message: Option<String>,
//     }

//     icrc2_approve(pic, backend_canister, ckbtc_canister);

//     let test_cases: Vec<TestCase> = vec![
//         // Valid Pool Data Test Case
//         TestCase {
//             pool_data: PoolData {
//                 pool_data: vec![CreatePoolParams {
//                     token_name: "ValidToken".to_string(),
//                     balance: Nat::from(500u128),
//                     weight: Nat::from(20u128),
//                     value: Nat::from(200u128),
//                     ledger_canister_id: ckbtc_canister,
//                     image: "valid_image.png".to_string(),
//                 }],
//                 swap_fee: Nat::from(5u128),
//             },
//             expect_success: true,
//             expected_error_message: None,
//         },
//     ];

//     for (i, case) in test_cases.iter().enumerate() {
//         ic_cdk::println!("\n------------------------------------------------------------");
//         ic_cdk::println!("🔵 IC Test Case {}: Executing Rollbacks Request", i + 1);
//         ic_cdk::println!("------------------------------------------------------------\n");

//     }

//     // let invalid_pool_data = Pool_Data {
//     //     pool_data: vec![CreatePoolParams {
//     //         token_name: "bothTokens".to_string(),
//     //         balance: Nat::from(100u128),
//     //         weight: Nat::from(10u128),
//     //         value: Nat::from(100u128), // Invalid: Zero values
//     //         ledger_canister_id: ckbtc_canister,
//     //         image: "invalid_image.png".to_string(),
//     //     }],
//     //     swap_fee: Nat::from(5u128),
//     // };

//     let encoded_args = candid::encode_args((invalid_pool_data,)).unwrap();

//     let response = pic
//         .update_call(
//             backend_canister,
//             hardcoded_principal,
//             "create_pools",
//             encoded_args,
//         )
//         .unwrap();

//     match response {
//         WasmResult::Reply(data) => {
//             let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
//             assert!(
//                 result.is_err(),
//                 "Expected failure to trigger rollback, but got success."
//             );
//             println!(
//                 "Rollback triggered as expected. Error: {:?}",
//                 result.unwrap_err()
//             );
//         }
//         WasmResult::Reject(message) => {
//             println!("Rollback triggered due to rejection: {}", message);
//         }
//     }

//     let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

//     let pools_response = pic
//         .query_call(
//             backend_canister,
//             hardcoded_principal,
//             "get_users_pool",
//             query_args,
//         )
//         .unwrap();

//     match pools_response {
//         WasmResult::Reply(data) => {
//             let pools: Result<Option<Vec<String>>, CustomError> =
//                 candid::decode_one(&data).unwrap();
//             assert!(
//                 pools.as_ref().unwrap_or(&None).is_none(),
//                 "Expected no pools after rollback, but found: {:?}",
//                 pools
//             );
//             println!("Rollback verification successful: No pools found.");
//         }
//         WasmResult::Reject(message) => {
//             panic!("Failed to query user's pools: {}", message);
//         }
//     }
// }

// this one is correct.TokenDepositFailed -- here canister_id is not valid or found will be a better error.
// no need to check roll back in this as id is wrong.
// #[test]
fn test_rollback_on_token_deposit_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    // ✅ Step 1: Approve tokens
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(1000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(
                        allowance > Nat::from(0u64),
                        "Allowance should be greater than zero."
                    );
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    // ✅ Step 2: Simulate Token Deposit Failure
    // Using an invalid ledger_canister_id to trigger failure
    let invalid_ledger_canister_id = Principal::from_text("aaaaa-aa").unwrap(); // Invalid principal

    let faulty_pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "FaultyToken".to_string(),
            balance: Nat::from(100u64),
            weight: Nat::from(10u64),
            value: Nat::from(100u64),
            ledger_canister_id: invalid_ledger_canister_id, // 🚩 Invalid ID to trigger failure
            image: "invalid_image.png".to_string(),
        }],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((faulty_pool_data,)).unwrap();

    // ✅ Step 3: Attempt to Create Pool (Expected to Fail & Trigger Rollback)
    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure due to invalid ledger_canister_id, but got success."
            );
            println!(
                "Rollback triggered as expected. Error: {:?}",
                result.unwrap_err()
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered due to rejection: {}", message);
        }
    }

    // ✅ Step 4: Verify Rollback Using get_users_pool
    let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

    let pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            query_args,
        )
        .unwrap();

    match pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, CustomError> =
                candid::decode_one(&data).unwrap();
            assert!(
                pools.as_ref().unwrap_or(&None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
            println!("Rollback verification successful: No pools found.");
        }
        WasmResult::Reject(message) => {
            panic!("Failed to query user's pools: {}", message);
        }
    }
}

// test cases passed.
// #[test]
fn test_rollback_on_lp_share_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    // ✅ Step 1: Approve tokens
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(1000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(
                        allowance > Nat::from(0u64),
                        "Allowance should be greater than zero."
                    );
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    // ✅ Step 2: Simulate LP Share Calculation Failure
    let faulty_pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "FaultyToken".to_string(),
            balance: Nat::from(100u64),
            weight: Nat::from(10u64),
            value: Nat::from(100u64),
            ledger_canister_id: ckbtc_canister,
            image: "invalid_image.png".to_string(),
        }],
        swap_fee: Nat::from(u128::MAX), // 🚩 Invalid swap fee to cause LP share failure
    };
    ic_cdk::println!("Faulty pool swap fee: {:?}", faulty_pool_data.swap_fee);

    let encoded_args = candid::encode_args((faulty_pool_data.clone(),)).unwrap(); // Clone added
    ic_cdk::println!("ckbtc canister id: {:?}", ckbtc_canister.to_text());

    // ✅ Step 3: Attempt to Create Pool (Expected to Fail & Trigger Rollback)
    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure due to LP share calculation error, but got success."
            );
            println!(
                "Rollback triggered as expected. Error: {:?}",
                result.unwrap_err()
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered due to rejection: {}", message);
        }
    }

    // ✅ Step 4: Verify Rollback Using get_users_pool
    let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

    let pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            query_args,
        )
        .unwrap();

    match pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, CustomError> =
                candid::decode_one(&data).unwrap();
            assert!(
                pools.as_ref().unwrap_or(&None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
            println!("Rollback verification successful: No pools found.");
        }
        WasmResult::Reject(message) => {
            panic!("Failed to query user's pools: {}", message);
        }
    }
}

// working correct.
// #[test]
fn test_rollback_on_add_liquidity_curr_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(1000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(
                        allowance > Nat::from(0u64),
                        "Allowance should be greater than zero."
                    );
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    let pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "FaultyToken".to_string(),
            balance: Nat::from(100u64),
            weight: Nat::from(10u64),
            value: Nat::from(100u64),
            ledger_canister_id: ckbtc_canister,
            image: "image.png".to_string(),
        }],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = encode_args((pool_data.clone(),)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure, got success: {:?}",
                result
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered as expected. Message: {}", message)
        }
    }

    let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

    let get_pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            query_args,
        )
        .unwrap();

    match get_pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, String> = decode_one(&data).unwrap();
            assert!(
                // ic_cdk::println!("i think this one is failing")
                pools.clone().unwrap_or(None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
        }
        WasmResult::Reject(message) => panic!("Failed to get user pools: {}", message),
    }
}

// TODO: rollbacks are fixed, but i need to have a function from the harshit to rollback the created pool as well.
// #[test]
fn test_rollback_on_add_liquidity_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(100_000_000u128),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    let pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "FaultyToken".to_string(),
            balance: Nat::from(600u128),
            weight: Nat::from(10u128),
            value: Nat::from(600u128),
            ledger_canister_id: ckbtc_canister,
            image: "image.png".to_string(),
        }],
        swap_fee: Nat::from(5u128),
    };

    let encoded_args = encode_args((pool_data.clone(),)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure, got success: {:?}",
                result
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered as expected. Message: {}", message)
        }
    }

    let get_pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            encode_args((hardcoded_principal,)).unwrap(),
        )
        .unwrap();

    match get_pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, String> = decode_one(&data).unwrap();
            assert!(
                pools.clone().unwrap_or(None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
        }
        WasmResult::Reject(message) => panic!("Failed to get user pools: {}", message),
    }
}

// TODO: rollbacks are fixed, but i need to have a function from the harshit to rollback the created pool as well.
// #[test]
fn test_rollback_on_store_pool_data_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(100_000_000u128),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    let pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "FaultyToken".to_string(),
            balance: Nat::from(100u128),
            weight: Nat::from(10u128),
            value: Nat::from(100u128),
            ledger_canister_id: ckbtc_canister,
            image: "image.png".to_string(),
        }],
        swap_fee: Nat::from(5u128),
    };

    let encoded_args = encode_args((pool_data.clone(),)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure, got success: {:?}",
                result
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered as expected. Message: {}", message)
        }
    }

    let get_pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            encode_args((hardcoded_principal,)).unwrap(),
        )
        .unwrap();

    match get_pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, String> = decode_one(&data).unwrap();
            assert!(
                pools.clone().unwrap_or(None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
        }
        WasmResult::Reject(message) => panic!("Failed to get user pools: {}", message),
    }
}

// #[test]
fn test_rollback_on_lock_handling_failure(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
) {
    // let (pic, backend_canister, ckbtc_canister) = setup();
    ic_cdk::println!("Backend canister: {:?}", backend_canister.to_text());
    ic_cdk::println!("CKBTC canister: {:?}", ckbtc_canister.to_text());

    let hardcoded_principal = get_user_principal();
    let spender_canister = backend_canister;

    // Step 1: Approve Token Spending
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(1000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    // Step 2: Trigger Pool Creation with Intentional Failure (Simulating Lock Failure)
    let pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "LockedPool".to_string(),
            balance: Nat::from(100u64),
            weight: Nat::from(10u64),
            value: Nat::from(100u64),
            ledger_canister_id: ckbtc_canister,
            image: "image.png".to_string(),
        }],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = encode_args((pool_data.clone(),)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args.clone(),
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                result.is_err(),
                "Expected failure, got success: {:?}",
                result
            );
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered as expected. Message: {}", message)
        }
    }

    // Step 3: Ensure Lock Was Released by Trying to Create the Same Pool Again
    let response_retry = pic
        .update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args.clone(),
        )
        .unwrap();

    match response_retry {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                !matches!(result, Err(CustomError::AnotherOperationInProgress(_))),
                "Lock was not released! Retrying pool creation still results in 'AnotherOperationInProgress'."
            );
        }
        WasmResult::Reject(message) => {
            println!("Retried creation failed as expected. Message: {}", message)
        }
    }

    // Step 4: Ensure No Pool Was Stored for the User
    let get_pools_response = pic
        .query_call(
            backend_canister,
            hardcoded_principal,
            "get_users_pool",
            encode_args((hardcoded_principal,)).unwrap(),
        )
        .unwrap();

    match get_pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, CustomError> = decode_one(&data).unwrap();
            assert!(
                pools.clone().unwrap_or(None).is_none(),
                "Expected no pools after rollback, but found: {:?}",
                pools
            );
        }
        WasmResult::Reject(message) => panic!("Failed to get user pools: {}", message),
    }
}
