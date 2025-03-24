use candid::{encode_args, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::fs;
mod utils;
use crate::utils::structs::*;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";
const CKETH_WASM: &str = "../../.dfx/local/canisters/cketh/cketh.wasm.gz";
const CKUSDC_WASM: &str = "../../.dfx/local/canisters/ckusdc/ckusdc.wasm.gz";
const LP_LEDGER_WASM: &str ="../../.dfx/local/canisters/LP_ledger_canister/LP_ledger_canister.wasm.gz";

fn setup() -> (
    PocketIc,
    Principal,
    Principal,
    Principal,
    Principal,
    Principal,
) {
    std::env::set_var(
        "POCKET_IC_BIN",
        "/home/ray/valueswap/src/valueswap_backend/tests/pocket-ic",
    ); // Path of the pocket-ic binary

    let pic = PocketIc::new();

    let backend_canister = pic.create_canister();
    pic.add_cycles(backend_canister, 2_000_000_000_000); // 2T Cycles
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
    let ckbtc_args_encoded =
        encode_args((LedgerArgument::Init(ckbtc_args),)).expect("Failed to encode arguments");
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
            Nat::from(10_000_000_000_000u64),
        )],
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
    let cketh_args_encoded =
        encode_args((LedgerArgument::Init(cketh_args),)).expect("Failed to encode arguments");
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
            Nat::from(10_000_000_000_000u64),
        )],
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
    let ckusdc_args_encoded =
        encode_args((LedgerArgument::Init(ckusdc_args),)).expect("Failed to encode arguments");
    pic.install_canister(ckusdc_canister, ckusdc_wasm, ckusdc_args_encoded, None);
    println!("CKUSDC canister: {}", ckusdc_canister);

    let lp_ledger_canister = pic.create_canister();
    pic.add_cycles(lp_ledger_canister, 2_000_000_000_000);
    let lp_ledger_wasm = fs::read(LP_LEDGER_WASM).expect("Wasm file not found, run 'dfx build'.");

    // Define the initialization arguments for the LP Ledger canister
    let lp_ledger_args = InitArgs {
        token_symbol: String::from("LP_Token"),
        token_name: String::from("LP_Token"),
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
            Nat::from(10_000_000u64),
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

    let lp_args_encoded =
        encode_args((LedgerArgument::Init(lp_ledger_args),)).expect("Failed to encode arguments");

    // Install the LP Ledger canister
    pic.install_canister(lp_ledger_canister, lp_ledger_wasm, lp_args_encoded, None);
    println!("LP Ledger canister: {}", lp_ledger_canister);

    (
        pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        ckusdc_canister,
        lp_ledger_canister,
    )
}

//SUCCESS TEST CASE - 1

#[test]
fn test_create_pools_with_varied_weights() {
    let (
        pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        _ckusdc_canister,
        _lp_ledger_canister,
    ) = setup();

    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();

    // Approve tokens for ckbtc and cketh ledgers
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

        let response = pic
            .update_call(
                *canister,
                hardcoded_principal,
                "icrc2_approve",
                encoded_args,
            )
            .unwrap();

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

    // Create pools with varied weights
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(80u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckbtc.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(20u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image_cketh.png".to_string(),
            },
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

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
                result.is_ok(),
                "Expected successful pool creation, got {:?}",
                result
            );
        }
        WasmResult::Reject(message) => panic!("Failed to create pools: {}", message),
    }
}

//SUCCESS TEST CASE - 2
#[test]
fn test_create_pool_with_three_tokens_30_40_30() {
    let (
        pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        ckusdc_canister,
        _lp_ledger_canister,
    ) = setup();

    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();

    // ICRC-2 Approve for each ledger canister (ckbtc, cketh, and third_canister)
    for canister in [&ckbtc_canister, &cketh_canister, &ckusdc_canister] {
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

        let response = pic
            .update_call(
                *canister,
                hardcoded_principal,
                "icrc2_approve",
                encoded_args,
            )
            .unwrap();

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

    // Create a pool with three tokens having 30%, 40%, and 30% weights respectively
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(30u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckbtc.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(40u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image_cketh.png".to_string(),
            },
            CreatePoolParams {
                token_name: "ckusdc".to_string(), // Replace with actual token name
                balance: Nat::from(100000u64),
                weight: Nat::from(30u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckusdc.png".to_string(),
            },
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

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
                result.is_ok(),
                "Expected successful pool creation with three tokens, got {:?}",
                result
            );
            println!("Pool creation successful for three tokens with 30/40/30 weights.");
        }
        WasmResult::Reject(message) => {
            panic!("Failed to create pool with three tokens: {}", message)
        }
    }
}

//SUCCESS TEST CASE - 3
#[test]
fn test_create_pool_with_equal_weights() {
    let (
        pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        ckusdc_canister,
        _lp_ledger_canister,
    ) = setup();

    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();

    // ICRC-2 Approve for ckbtc and cketh ledgers
    for canister in [&ckbtc_canister, &cketh_canister, &ckusdc_canister] {
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

        let response = pic
            .update_call(
                *canister,
                hardcoded_principal,
                "icrc2_approve",
                encoded_args,
            )
            .unwrap();

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

    // Create a pool with equal weights for four tokens
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(25u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_ckbtc.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(25u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image_cketh.png".to_string(),
            },
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(25u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckusdc_canister,
                image: "image_ckbtc2.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(25u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image_cketh2.png".to_string(),
            },
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

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
                result.is_ok(),
                "Expected successful pool creation with equal weights, got {:?}",
                result
            );
            println!("Pool creation successful for equal weights.");
        }
        WasmResult::Reject(message) => {
            panic!("Failed to create pool with equal weights: {}", message)
        }
    }
}

//SUCCESS TEST CASE - 4

#[test]
fn test_create_pool_with_four_tokens_equal_weights() {
    let (
        pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        ckusdc_canister,
        _lp_ledger_canister,
    ) = setup();

    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();

    // Approve tokens for ckbtc and cketh ledgers
    for canister in [&ckbtc_canister, &cketh_canister, &ckusdc_canister] {
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

        let response = pic
            .update_call(
                *canister,
                hardcoded_principal,
                "icrc2_approve",
                encoded_args,
            )
            .unwrap();

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

    // Create a pool with eight tokens having equal weights of 12.5%
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(50000u64),
                weight: Nat::from(12_5u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image1.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(50000u64),
                weight: Nat::from(12_5u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image2.png".to_string(),
            },
            CreatePoolParams {
                token_name: "ckusdc".to_string(),
                balance: Nat::from(50000u64),
                weight: Nat::from(12_5u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image3.png".to_string(),
            },
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(50000u64),
                weight: Nat::from(12_5u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister,
                image: "image4.png".to_string(),
            },
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

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
                result.is_ok(),
                "Expected successful pool creation with eight tokens at equal weights, got {:?}",
                result
            );
            println!("Pool creation successful for eight tokens with equal weights.");
        }
        WasmResult::Reject(message) => {
            panic!("Failed to create pool with eight tokens: {}", message)
        }
    }
}
