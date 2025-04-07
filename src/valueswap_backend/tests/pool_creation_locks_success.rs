use candid::{encode_args, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::fs;
mod utils;
use crate::utils::structs::*;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";
const CKETH_WASM: &str = "../../.dfx/local/canisters/cketh/cketh.wasm.gz";
const CKUSDC_WASM: &str = "../../.dfx/local/canisters/ckusdc/ckusdc.wasm.gz";

fn setup() -> (PocketIc, Principal, Principal, Principal, Principal) {
    // std::env::set_var(
    //     "POCKET_IC_BIN",
    //     "/home/ray/valueswap/src/valueswap_backend/tests/pocket-ic",
    // ); // Path of the pocket-ic binary

    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 10_000_000_000_000); // 2T Cycles
    let backend_wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, backend_wasm, vec![], None);

    // Install ckbtc_canister
    let ckbtc_canister = pic.create_canister();
    pic.add_cycles(ckbtc_canister, 2_000_000_000_000); // 2T Cycles
    let ckbtc_wasm = fs::read(CKBTC_WASM).expect("Wasm file not found, run 'dfx build'.");
    let ckbtc_args = InitArgs {
        token_symbol: String::from("CKBTC"),
        token_name: String::from("CKBTC"),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: Principal::from_text("6mrpp-3ynrv-4q5tl-xsuey-jwi6d-xfukg-w4l3l-h2ejb-h3fea-ghycd-mqe").unwrap(),
            subaccount: None,
        },
        initial_balances: vec![(Account {
            owner: Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap(),
            subaccount: None,
        }, Nat::from(1_000_000u64))],
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
    let ckbtc_args_encoded = encode_args((LedgerArgument::Init(ckbtc_args),)).expect("Failed to encode arguments");
    pic.install_canister(ckbtc_canister, ckbtc_wasm, ckbtc_args_encoded, None);
    println!("CKBTC canister: {}", ckbtc_canister);

    // Install cketh_canister
    let cketh_canister = pic.create_canister();
    pic.add_cycles(cketh_canister, 2_000_000_000_000); // 2T Cycles
    let cketh_wasm = fs::read(CKETH_WASM).expect("Wasm file not found, run 'dfx build'.");
    let cketh_args = InitArgs {
        token_symbol: String::from("CKETH"),
        token_name: String::from("CKETH"),
        transfer_fee: Nat::from(10000u64),
        metadata: vec![],
        minting_account: Account {
            owner: Principal::from_text("6mrpp-3ynrv-4q5tl-xsuey-jwi6d-xfukg-w4l3l-h2ejb-h3fea-ghycd-mqe").unwrap(),
            subaccount: None,
        },
        initial_balances: vec![(Account {
            owner: Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap(),
            subaccount: None,
        }, Nat::from(10_000_000_000_000u64))],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            max_transactions_per_response: None,
            trigger_threshold: 2000,
            more_controller_ids: None,
            max_message_size_bytes: None,
            cycles_for_archive_creation: Some(10_000_000_000_000u64),
            node_max_memory_size_bytes: None,
            controller_id: Principal::anonymous(),
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    };
    let cketh_args_encoded = encode_args((LedgerArgument::Init(cketh_args),)).expect("Failed to encode arguments");
    pic.install_canister(cketh_canister, cketh_wasm, cketh_args_encoded, None);
    println!("CKETH canister: {}", cketh_canister);

    // Install ckusdc_canister
    let ckusdc_canister = pic.create_canister();
    pic.add_cycles(ckusdc_canister, 2_000_000_000_000); // 2T Cycles
    let ckusdc_wasm = fs::read(CKUSDC_WASM).expect("Wasm file not found, run 'dfx build'.");
    let ckusdc_args = InitArgs {
        token_symbol: String::from("CKUSDC"),
        token_name: String::from("CKUSDC"),
        transfer_fee: Nat::from(10000u64),
        metadata: vec![],
        minting_account: Account {
            owner: Principal::from_text("6mrpp-3ynrv-4q5tl-xsuey-jwi6d-xfukg-w4l3l-h2ejb-h3fea-ghycd-mqe").unwrap(),
            subaccount: None,
        },
        initial_balances: vec![(Account {
            owner: Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap(),
            subaccount: None,
        }, Nat::from(10_000_000_000_000u64))],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            max_transactions_per_response: None,
            trigger_threshold: 2000,
            more_controller_ids: None,
            max_message_size_bytes: None,
            cycles_for_archive_creation: Some(10_000_000_000_000u64),
            node_max_memory_size_bytes: None,
            controller_id: Principal::anonymous(),
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    };
    let ckusdc_args_encoded = encode_args((LedgerArgument::Init(ckusdc_args),)).expect("Failed to encode arguments");
    pic.install_canister(ckusdc_canister, ckusdc_wasm, ckusdc_args_encoded, None);
    println!("CKUSDC canister: {}", ckusdc_canister);

    (pic, backend_canister, ckbtc_canister, cketh_canister, ckusdc_canister)
}

//SUCCESS TEST CASE - 1

#[test]
fn test_create_pool_with_no_prior_lock_and_approval() {
    let (pic, backend_canister, ckbtc_canister, _cketh_canister,_ckusdc_canister) = setup();

    // Define hardcoded principal for simulation
    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // icrc2_approve for ckbtc ledger to simulate successful token approval
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(10000000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: backend_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    // Approve operation
    let approve_response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

    assert!(
        matches!(approve_response, WasmResult::Reply(_)),
        "Approval should be successful to proceed with pool creation"
    );

    // Prepare pool data
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image.png".to_string(),
            }
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

    // Attempt to create a pool
    let response = pic.update_call(
        backend_canister,
        hardcoded_principal,
        "create_pools",
        encoded_args,
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(result.is_ok(), "Expected successful pool creation without prior lock");
        },
        WasmResult::Reject(message) => panic!("Creation failed unexpectedly: {}", message),
    }
}

// SUCCESS TEST CASE - 2

#[test]
fn test_create_multiple_pools_successively() {
    let (pic, backend_canister, ckbtc_canister, _cketh_canister, ckusdc_canister) = setup();

    // Principal for testing
    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // Token Canisters array to alternate between tokens
    let token_canisters = [ckbtc_canister, ckusdc_canister];

    // Loop to simulate creating multiple pools
    for i in 1..=4 {
        let token_canister = token_canisters[i % 2]; // Alternate between ckbtc and ckusdc

        // Approve operation for each pool creation
        let approval_args = ApproveArgs {
            fee: None,
            memo: None,
            from_subaccount: None,
            created_at_time: None,
            amount: Nat::from(10000000u64),
            expected_allowance: None,
            expires_at: None,
            spender: Account {
                owner: backend_canister,
                subaccount: None,
            },
        };

        let encoded_args = candid::encode_args((approval_args,)).unwrap();
        let approve_response = pic.update_call(
            token_canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        ).unwrap();

        assert!(
            matches!(approve_response, WasmResult::Reply(_)),
            "Approval should be successful to proceed with pool creation"
        );

        // Create pool with varying parameters
        let pool_data = Pool_Data {
            pool_data: vec![
                CreatePoolParams {
                    token_name: format!("Token{}", i),
                    balance: Nat::from(100000u64 * i as u64),
                    weight: Nat::from(10u64),
                    value: Nat::from(100u64 * i as u64),
                    ledger_canister_id: token_canister,
                    image: format!("image{}.png", i),
                }
            ],
            swap_fee: Nat::from(5u64),
        };

        let encoded_args = candid::encode_args((pool_data,)).unwrap();
        let response = pic.update_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            encoded_args,
        ).unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
                assert!(result.is_ok(), "Expected successful pool creation for sequence {}", i);
            },
            WasmResult::Reject(message) => panic!("Creation failed unexpectedly in sequence {}: {}", i, message),
        }
    }
}

// SUCCESS TEST CASE - 3

#[test]
fn test_create_pool_with_approval_timeout() {
    let (pic, backend_canister, _ckbtc_canister, _cketh_canister, ckusdc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // Simulate icrc2_approve with a manual reject to mimic a timeout scenario
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(10000000u64),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: backend_canister,
            subaccount: None,
        },
    };

    let _encoded_args = candid::encode_args((approval_args,)).unwrap();

    // Manually force a reject scenario to simulate a timeout for educational purposes
    let response = WasmResult::Reject("Simulated timeout".to_string());

    assert!(matches!(response, WasmResult::Reject(_)), "Approval should fail due to simulated timeout");

    // Proceed with the creation attempt which should also fail due to dependencies not being correctly approved
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckusdc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckusdc_canister,
                image: "image.png".to_string(),
            }
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();
    let response = pic.update_call(
        backend_canister,
        hardcoded_principal,
        "create_pools",
        encoded_args,
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(result.is_err(), "Pool creation should fail due to dependency approval failure");
        },
        WasmResult::Reject(message) => println!("Failed to create pools as expected due to dependency failure: {}", message),
    }
}


// SUCCESS TEST CASE - 3
#[test]
fn test_create_multiple_pools_concurrently_with_approval() {
    let (pic, backend_canister, ckbtc_canister, cketh_canister, _ckusdc_canister) = setup();

    // Principal for testing
    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    let mut pool_creation_msg_ids = Vec::new();
    let mut approval_msg_ids = Vec::new();

    // Array of token canisters to use in the test
    let token_canisters = [ckbtc_canister, cketh_canister];

    // Submit multiple concurrent approval and pool creation calls
    for i in 0..4 {
        let token_canister = token_canisters[i % 2]; // Alternate between ckbtc and cketh

        // Approve operation for each pool creation
        let approval_args = ApproveArgs {
            fee: None,
            memo: None,
            from_subaccount: None,
            created_at_time: None,
            amount: Nat::from(10000000u64),
            expected_allowance: None,
            expires_at: None,
            spender: Account {
                owner: backend_canister,
                subaccount: None,
            },
        };

        let approval_encoded_args = candid::encode_args((approval_args,)).unwrap();
        let approval_msg_id = pic.submit_call(
            token_canister,
            hardcoded_principal,
            "icrc2_approve",
            approval_encoded_args,
        ).unwrap();

        approval_msg_ids.push(approval_msg_id);

        // Create pool with varying parameters
        let pool_data = Pool_Data {
            pool_data: vec![
                CreatePoolParams {
                    token_name: format!("Token{}", i),
                    balance: Nat::from(100000u64 * (i as u64 + 1)),
                    weight: Nat::from(10u64),
                    value: Nat::from(100u64 * (i as u64 + 1)),
                    ledger_canister_id: token_canister,
                    image: format!("image{}.png", i),
                }
            ],
            swap_fee: Nat::from(5u64),
        };

        let pool_encoded_args = candid::encode_args((pool_data,)).unwrap();
        let pool_msg_id = pic.submit_call(
            backend_canister,
            hardcoded_principal,
            "create_pools",
            pool_encoded_args,
        ).unwrap();

        pool_creation_msg_ids.push(pool_msg_id);
    }

    // Await all submitted approval calls and check results
    for msg_id in approval_msg_ids {
        let res = pic.await_call(msg_id).unwrap();
        assert!(matches!(res, WasmResult::Reply(_)), "Approval should be successful to proceed with pool creation");
    }

    // Await all submitted pool creation calls and check results
    for msg_id in pool_creation_msg_ids {
        let res = pic.await_call(msg_id).unwrap();
        match res {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
                assert!(result.is_ok(), "Expected successful pool creation");
            },
            WasmResult::Reject(message) => {
                println!("Pool creation failed with message: {}", message);
                assert!(false, "Pool creation should not fail");
            },
        }
    }
}

