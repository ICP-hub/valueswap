use candid::{encode_args, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::fs;
mod utils;
use crate::utils::structs::*;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";
const CKETH_WASM: &str = "../../.dfx/local/canisters/cketh/cketh.wasm.gz";

fn setup() -> (PocketIc, Principal, Principal, Principal) {
    std::env::set_var(
        "POCKET_IC_BIN",
        "/home/ray/valueswap/src/valueswap_backend/tests/pocket-ic",
    ); // Path of the pocket-ic binary

    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 2_000_000_000_000); // 2T Cycles
    let backend_wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, backend_wasm, vec![], None);

    let ckbtc_canister = pic.create_canister();
    pic.add_cycles(ckbtc_canister, 2_000_000_000_000); // 2T Cycles
    let ckbtc_wasm = fs::read(CKBTC_WASM).expect("Wasm file not found, run 'dfx build'.");

    let args = InitArgs {
        token_symbol: String::from("CKBTC"),
        token_name: String::from("CKBTC"),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: Principal::from_text(
                "6mrpp-3ynrv-4q5tl-xsuey-jwi6d-xfukg-w4l3l-h2ejb-h3fea-ghycd-mqe",
            )
            .unwrap(),
            subaccount: None,
        },
        initial_balances: vec![(
            Account {
                owner: Principal::from_text(
                    "xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe",
                )
                .unwrap(),
                subaccount: None,
            },
            Nat::from(1_000_000u64),
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

    // Create and setup the CKETH canister
    let cketh_canister = pic.create_canister();
    pic.add_cycles(cketh_canister, 2_000_000_000_000); // Assuming 2T Cycles as standard
    let cketh_wasm = fs::read(CKETH_WASM).expect("Wasm file not found, run 'dfx build'.");

    let cketh_args = InitArgs {
        token_symbol: String::from("ckETH"),
        token_name: String::from("ckETH"),
        transfer_fee: Nat::from(10000u64),
        metadata: vec![],
        minting_account: Account {
            owner: Principal::from_text(
                "6mrpp-3ynrv-4q5tl-xsuey-jwi6d-xfukg-w4l3l-h2ejb-h3fea-ghycd-mqe",
            )
            .unwrap(), // Replace with actual minter principal
            subaccount: None,
        },
        initial_balances: vec![(
            Account {
                owner: Principal::from_text(
                    "xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe",
                )
                .unwrap(), // Replace with actual principal
                subaccount: None,
            },
            Nat::from(10000000000000u64), // Adjust according to your specification
        )],
        archive_options: ArchiveOptions {
            num_blocks_to_archive: 1000,
            max_transactions_per_response: None,
            trigger_threshold: 2000,
            more_controller_ids: None,
            max_message_size_bytes: None,
            cycles_for_archive_creation: Some(10000000000000u64),
            node_max_memory_size_bytes: None,
            controller_id: Principal::anonymous(),
        },
        feature_flags: Some(FeatureFlags { icrc2: true }),
    };

    let cketh_args_encoded =
        encode_args((LedgerArgument::Init(cketh_args),)).expect("Failed to encode arguments");

    pic.install_canister(cketh_canister, cketh_wasm, cketh_args_encoded, None);
    println!("CKETH canister: {}", cketh_canister);

    (pic, backend_canister, ckbtc_canister, cketh_canister)
}

// FAILURE TEST CASE- 1
#[test]
fn test_create_pool_with_empty_token_name() {
    let (pic, backend_canister, ckbtc_canister, cketh_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // ICRC-2 Approve for ckbtc and cketh ledgers
    for canister in [&ckbtc_canister, &cketh_canister] {
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

        let response = pic.update_call(
            *canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        ).unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
                assert!(
                    result.is_ok(),
                    "Approval successful. Allowance set to: {:?}",
                    result.unwrap()
                );
            }
            WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
        }
    }

    // Create a pool with an empty token name
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "".to_string(), // Intentionally left empty to trigger validation failure
                balance: Nat::from(100000u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckbtc.png".to_string(),
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
            // Check if decoding fails which indicates an error was returned as expected
            let result = candid::decode_one::<()>(&data);
            assert!(
                result.is_err(),
                "Expected failure due to empty token name, but operation succeeded unexpectedly."
            );
            println!("Correctly failed to create pool due to empty token name.");
        },
        WasmResult::Reject(message) => {
            println!("Correctly rejected creation of pool due to empty token name: {}", message);
        }
    }
}

// FAILURE TEST CASE- 2

#[test]
fn test_create_pool_with_zero_weight() {
    let (pic, backend_canister, ckbtc_canister, cketh_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // ICRC-2 Approve for ckbtc and cketh ledgers
    for canister in [&ckbtc_canister, &cketh_canister] {
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

        let response = pic.update_call(
            *canister,
            hardcoded_principal,
            "icrc2_approve",
            encoded_args,
        ).unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
                assert!(
                    result.is_ok(),
                    "Approval successful. Allowance set to: {:?}",
                    result.unwrap()
                );
            }
            WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
        }
    }

    // Attempt to create a pool with zero weight
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(0u64), // Intentionally set to zero to trigger validation failure
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckbtc.png".to_string(),
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
            let result = candid::decode_one::<()>(&data);
            assert!(
                result.is_err(),
                "Expected failure due to zero weight, but operation succeeded unexpectedly."
            );
            println!("Correctly failed to create pool due to zero weight.");
        },
        WasmResult::Reject(message) => {
            println!("Correctly rejected creation of pool due to zero weight: {}", message);
        }
    }
}

// FAILURE TEST CASE- 3

#[test]
fn test_create_pool_with_incorrect_metadata() {
    let (pic, backend_canister, ckbtc_canister, _cketh_canister) = setup();

    // Assume setup() initializes ledgers correctly but we simulate incorrect metadata via direct call
    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();

    // icrc2_approve for ckusdc ledger with simulated incorrect metadata
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
    let response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

    assert!(matches!(response, WasmResult::Reject(_)), "Approval should fail due to incorrect metadata setup");

    // Create pool with a ledger canister that has incorrect metadata
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
    let response = pic.update_call(
        backend_canister,
        hardcoded_principal,
        "create_pools",
        encoded_args,
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(result.is_err(), "Pool creation should fail due to incorrect metadata");
        },
        WasmResult::Reject(message) => println!("Failed to create pools as expected: {}", message),
    }
}

// FAILURE TEST CASE- 4

#[test]
fn test_create_pool_with_zero_balance_token() {
    let (pic, backend_canister, ckbtc_canister, cketh_canister) = setup();

    // Approve tokens on ckbtc ledger for backend_canister
    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(1000000u64),  // Example large amount for sufficient approval
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: backend_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();
    let response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args.clone(),
    ).unwrap();

    assert!(matches!(response, WasmResult::Reply(_)), "Approval should succeed for valid token amounts");

    // Approve tokens on cketh ledger for backend_canister using same args but different canister
    let response = pic.update_call(
        cketh_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

    assert!(matches!(response, WasmResult::Reply(_)), "Approval should succeed for valid token amounts");

    // Attempt to create a pool where one token has zero balance
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),  // Valid balance
                weight: Nat::from(50u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(0u64),  // Zero balance
                weight: Nat::from(50u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image1.png".to_string(),
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
            assert!(
                result.is_err(),
                "Expected failure when trying to create a pool with a zero balance token, but operation succeeded"
            );
        },
        WasmResult::Reject(message) => println!("Failed to create pools as expected due to zero balance: {}", message),
    }
}
