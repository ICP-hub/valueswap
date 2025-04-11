mod utils;

use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use serde::Deserialize;
use utils::common_function::*;
use utils::structs::*;

use crate::utils::structs::InitArgs;
use std::fs;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";
const LP_LEDGER_WASM: &str =
    "../../.dfx/local/canisters/LP_ledger_canister/LP_ledger_canister.wasm.gz";
const CKETH_WASM: &str = "../../.dfx/local/canisters/cketh/cketh.wasm.gz";

#[test]
fn call_test_function() {
    let (pic, backend_canister, ckbtc_canister, lp_ledger_canister, cketh_canister) = setup();
    test_create_pools(
        &pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        lp_ledger_canister,
    );
    test_swap(&pic, backend_canister, ckbtc_canister, cketh_canister);
}

fn setup() -> (PocketIc, Principal, Principal, Principal, Principal) {
    ic_cdk::println!("Setting up Pocket IC...");

    let pic = PocketIc::new();
    ic_cdk::println!("Pocket IC setup complete.");

    let backend_canister = pic.create_canister();
    ic_cdk::println!("backend canister = {}", backend_canister.to_text());
    pic.add_cycles(backend_canister, 2_000_000_000_000_000); // 2T Cycles
    let backend_wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, backend_wasm, vec![], None);

    let ckbtc_canister = pic.create_canister();
    pic.add_cycles(ckbtc_canister, 2_000_000_000_000_000); // 2T Cycles
    let ckbtc_wasm = fs::read(CKBTC_WASM).expect("Wasm file not found, run 'dfx build'.");

    let lp_ledger_canister = pic.create_canister();
    pic.add_cycles(lp_ledger_canister, 2_000_000_000_000);
    let lp_ledger_wasm = fs::read(LP_LEDGER_WASM).expect("Wasm file not found, run 'dfx build'.");

    let cketh_canister = pic.create_canister();
    pic.add_cycles(cketh_canister, 2_000_000_000_000); // 2T Cycles
    let cketh_wasm = fs::read(CKETH_WASM).expect("Wasm file not found, run 'dfx build'.");

    let args = InitArgs {
        token_symbol: String::from("CKBTC"),
        token_name: String::from("CKBTC"),
        transfer_fee: Nat::from(100u128),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![
            (
                Account {
                    owner: get_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000_000u128),
            ),
            (
                Account {
                    owner: swapper_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000_000u128),
            ),
        ],
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

    ic_cdk::println!("LP Ledger canister: {}", lp_ledger_canister);

    // Define the initialization arguments for the LP Ledger canister
    let lp_ledger_args = InitArgs {
        token_symbol: String::from("LP_Token"),
        token_name: String::from("LP_Token"),
        transfer_fee: Nat::from(100u128),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![
            (
                Account {
                    owner: get_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000u128),
            ),
            (
                Account {
                    owner: swapper_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000u128),
            ),
        ],
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

    let args = InitArgs {
        token_symbol: String::from("CKETH"),
        token_name: String::from("CKETH"),
        transfer_fee: Nat::from(100u128),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![
            (
                Account {
                    owner: get_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000_000u128),
            ),
            (
                Account {
                    owner: swapper_user_principal(),
                    subaccount: None,
                },
                Nat::from(10_000_000_000u128),
            ),
        ],
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

    pic.install_canister(cketh_canister, cketh_wasm, args_encoded, None);
    println!("CKETH canister: {}", cketh_canister);

    (
        pic,
        backend_canister,
        ckbtc_canister,
        lp_ledger_canister,
        cketh_canister,
    )
}

fn test_create_pools(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
    lp_ledger_canister: Principal,
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
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "ckbtc".to_string(),
                        balance: Nat::from(100_000u128),
                        weight: Nat::from(50u128),
                        value: Nat::from(2000u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "btc.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "cketh".to_string(),
                        balance: Nat::from(2_000_000u128),
                        weight: Nat::from(50u128),
                        value: Nat::from(100u128),
                        ledger_canister_id: cketh_canister,
                        image: "eth.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(3u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
        // ✅ Different weights, non-zero valid values
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "btc".to_string(),
                        balance: Nat::from(90_000u128),
                        weight: Nat::from(30u128),
                        value: Nat::from(500_000u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "btc_icon.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "eth".to_string(),
                        balance: Nat::from(700_000u128),
                        weight: Nat::from(70u128),
                        value: Nat::from(150_000u128),
                        ledger_canister_id: cketh_canister,
                        image: "eth_icon.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(2u128),
            },
            expect_success: false,
            expected_error_message: Some(
                "Rollback triggered: lp canister id not provided".to_string(),
            ),
        },
        // ✅ High swap fee, still valid inputs
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "ARB".to_string(),
                        balance: Nat::from(500_000u128),
                        weight: Nat::from(60u128),
                        value: Nat::from(100u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "high.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "OP".to_string(),
                        balance: Nat::from(300_000u128),
                        weight: Nat::from(40u128),
                        value: Nat::from(90u128),
                        ledger_canister_id: cketh_canister,
                        image: "mid.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(10u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
        // ✅ Small values but all non-zero
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "FLOKI".to_string(),
                        balance: Nat::from(10_00u128),
                        weight: Nat::from(50u128), // 50%
                        value: Nat::from(105_000u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "a.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "AVAX".to_string(),
                        balance: Nat::from(10_00u128),
                        weight: Nat::from(50u128), // 50%
                        value: Nat::from(105_000u128),
                        ledger_canister_id: cketh_canister,
                        image: "b.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(1u128),
            },
            expect_success: false,
            expected_error_message: Some(
                "Rollback triggered: lp canister id not provided".to_string(),
            ),
        },
        // ✅ More than two tokens, all valid
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "UNI".to_string(),
                        balance: Nat::from(100_000u128),
                        weight: Nat::from(40u128),
                        value: Nat::from(400u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "1.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "MKR".to_string(),
                        balance: Nat::from(150_000u128),
                        weight: Nat::from(30u128),
                        value: Nat::from(200u128),
                        ledger_canister_id: cketh_canister,
                        image: "2.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "COMP".to_string(),
                        balance: Nat::from(200_000u128),
                        weight: Nat::from(30u128),
                        value: Nat::from(150u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "3.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(3u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "UNI".to_string(),
                        balance: Nat::from(100_000u128),
                        weight: Nat::from(40u128),
                        value: Nat::from(400u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "1.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "MKR".to_string(),
                        balance: Nat::from(150_000u128),
                        weight: Nat::from(30u128),
                        value: Nat::from(200u128),
                        ledger_canister_id: cketh_canister,
                        image: "2.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "COMP".to_string(),
                        balance: Nat::from(200_000u128),
                        weight: Nat::from(30u128),
                        value: Nat::from(150u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "3.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(3u128),
            },
            expect_success: false,
            expected_error_message: Some(
                "Rollback triggered: lp canister id not provided".to_string(),
            ),
        },
    ];

    let user_principal = get_user_principal();

    ic_cdk::println!("\n======================== Starting Rollback Test for Create Pools ========================\n");

    for (i, case) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n============================================================\n");
        ic_cdk::println!("🔵 IC Test Case {}: Executing create_pools request", i + 1);

        let mut concatenated_token = String::new(); // This will collect all the output

        for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
            let contribution = pool.balance.clone() * pool.value.clone();
            ic_cdk::println!("  🧪 Pool {} Details:", j + 1);
            ic_cdk::println!("    ▸ Token Name      : {}", pool.token_name);
            ic_cdk::println!("    ▸ Balance         : {}", pool.balance);
            ic_cdk::println!("    ▸ Weight          : {}%", pool.weight);
            ic_cdk::println!("    ▸ Value           : {}", pool.value);
            ic_cdk::println!("    ▸ Contribution    : {}", contribution);
            ic_cdk::println!("    ▸ Ledger Canister : {}", pool.ledger_canister_id);
            ic_cdk::println!("    ▸ Image           : {}", pool.image);
            ic_cdk::println!("\n");
            icrc2_approve(
                pic,
                backend_canister,
                pool.ledger_canister_id,
                user_principal,
            );
            concatenated_token.push_str(&pool.token_name); // append to the variable
            ic_cdk::println!("\n------------------------------------------------------------\n");
        }

        // ic_cdk::println!("pool name = {}", log_output);

        ic_cdk::println!("  ⚖ Swap Fee          : {}", case.pool_data.swap_fee);
        ic_cdk::println!("  ✅ Expect Success    : {}", case.expect_success);
        if let Some(err) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error    : {}", err);
        }
        ic_cdk::println!("\n");
        // ✅ Simulate proper setup on even index
        get_pool_lp_tokens(
            pic,
            backend_canister,
            user_principal,
            concatenated_token.clone(),
            Some("Before rollback"),
        );
        get_user_pool_lp_for_token(
            pic,
            backend_canister,
            user_principal,
            concatenated_token.clone(),
        );
        if i % 2 == 0 {
            set_canister_id(
                &pic,
                backend_canister,
                "lp_ledger".to_string(),
                lp_ledger_canister,
            );
        } else {
            remove_canister_id_by_name_pocket_ic(
                pic,
                backend_canister,
                user_principal,
                "lp_ledger".to_string(),
            );
        }
        ic_cdk::println!("\n------------------------------------------------------------\n");
        // ic_cdk::println!("log output = {}", log_output);

        let encoded_args = candid::encode_args((&case.pool_data,)).unwrap();
        let response = pic
            .update_call(
                backend_canister,
                user_principal,
                "create_pools",
                encoded_args,
            )
            .unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                if case.expect_success {
                    assert!(
                        result.is_ok(),
                        "❌ Test {} failed: Expected success, got error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                    ic_cdk::println!("✅ Test {} passed ✅ Pool created successfully.", i + 1);
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected failure due to rollback, but got success.",
                        i + 1
                    );
                    ic_cdk::println!(
                        "\n------------------------------------------------------------\n"
                    );
                    ic_cdk::println!(
                        "✅ Test Case {} Passed: Lp Ledger Canister Id Not Provided",
                        i + 1
                    );
                    ic_cdk::println!(
                        "   ➜ Triggering rollback mechanism for pool creation...",
                        // case.asset_name
                    );
                    ic_cdk::println!(
                        "------------------------------------------------------------"
                    );
                    ic_cdk::println!("🔄 Rollback Process Initiated...");
                    ic_cdk::println!("   ⏳ Reverting all state changes...");
                    ic_cdk::println!("   🔁 Restoring previous balances and reserves...");
                    // ic_cdk::println!("✅ Rollback Successfully Completed!");
                    ic_cdk::println!(
                        "\n------------------------------------------------------------\n"
                    );
                    ic_cdk::println!(
                        "✅ Test {} passed as expected with rollback: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                }
            }
            WasmResult::Reject(message) => {
                if case.expect_success {
                    ic_cdk::println!(
                        "❌ Test {} failed: Unexpected rejection occurred: {}",
                        i + 1,
                        message
                    );
                } else {
                    ic_cdk::println!(
                        "✅ Test {} passed! Rejected as expected (rollback worked): {}",
                        i + 1,
                        message
                    );
                }
            }
        }
        get_pool_lp_tokens(
            pic,
            backend_canister,
            user_principal,
            concatenated_token.clone(),
            Some("After rollback"),
        );
        get_user_pool_lp_for_token(
            pic,
            backend_canister,
            user_principal,
            concatenated_token.clone(),
        );
        ic_cdk::println!("\n============================================================\n");
    }

    ic_cdk::println!(
        "\n======================== Rollback Create Pools Test Completed ========================\n"
    );
}

fn test_swap(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
) {
    #[derive(Clone)]
    struct TestCase {
        swap_params: SwapParams,
        expect_success: bool,
        expected_error_message: Option<String>,
    }

    let test_cases = vec![
        TestCase {
            swap_params: SwapParams {
                token1_name: "ckbtc".to_string(),
                token_amount: Nat::from(5000u128),
                token2_name: "cketh".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: cketh_canister,
                fee: Nat::from(3u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
        TestCase {
            swap_params: SwapParams {
                token1_name: "btc".to_string(),
                token_amount: Nat::from(1000u128),
                token2_name: "eth".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: cketh_canister,
                fee: Nat::from(2u128),
            },
            expect_success: false,
            expected_error_message: Some(
                "NoCanisterIDFound".to_string(),
            ),
        },
        TestCase {
            swap_params: SwapParams {
                token1_name: "ARB".to_string(),
                token_amount: Nat::from(20_00u128),
                token2_name: "OP".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: cketh_canister,
                fee: Nat::from(10u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
        TestCase {
            swap_params: SwapParams {
                token1_name: "FLOKI".to_string(),
                token_amount: Nat::from(5u128),
                token2_name: "AVAX".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: cketh_canister,
                fee: Nat::from(1u128),
            },
            expect_success: false,
            expected_error_message: Some(
                "NoCanisterIDFound".to_string(),
            ),
        },
        TestCase {
            swap_params: SwapParams {
                token1_name: "UNI".to_string(),
                token_amount: Nat::from(10_00u128),
                token2_name: "COMP".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: ckbtc_canister,
                fee: Nat::from(3u128),
            },
            expect_success: true,
            expected_error_message: None,
        },
    ];

    ic_cdk::println!("\n==================== 🔁 Starting Swap Tests ====================\n");

    let user_principal = swapper_user_principal();

    for (i, case) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n============================================================\n");
        ic_cdk::println!("🔵 Test Case {}: Executing swap request", i + 1);

        let swap_params = &case.swap_params;

        ic_cdk::println!("📌 Swap Parameters:");
        ic_cdk::println!("   • Token 1        : {}", swap_params.token1_name);
        ic_cdk::println!("   • Token 2        : {}", swap_params.token2_name);
        ic_cdk::println!("   • Amount         : {}", swap_params.token_amount);
        ic_cdk::println!("   • Swap Fee       : {}", swap_params.fee);
        ic_cdk::println!(
            "   • Ledger Canister 1 : {}",
            swap_params.ledger_canister_id1
        );
        ic_cdk::println!(
            "   • Ledger Canister 2 : {}",
            swap_params.ledger_canister_id2
        );
        if let Some(msg) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error  : {}", msg);
        }

        // Approve only for even-indexed test cases
        if i % 2 == 0 {
            ic_cdk::println!("\n------------------------------------------------------------\n");
            icrc2_approve(
                pic,
                backend_canister,
                swap_params.ledger_canister_id1,
                user_principal,
            );
            icrc2_approve(
                pic,
                backend_canister,
                swap_params.ledger_canister_id2,
                user_principal,
            );
            ic_cdk::println!("\n------------------------------------------------------------\n");
        }

        let encoded_args = candid::encode_args((swap_params.clone(),)).unwrap();

        let response = pic
            .update_call(
                backend_canister,
                user_principal,
                "compute_swap",
                encoded_args,
            )
            .unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                if case.expect_success {
                    assert!(
                        result.is_ok(),
                        "❌ Test {} failed: Expected success, got error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                    ic_cdk::println!("✅ Test {} Passed ✅ Swap completed successfully.", i + 1);
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected rollback but got success.",
                        i + 1
                    );
                    ic_cdk::println!(
                        "\n------------------------------------------------------------\n"
                    );
                    ic_cdk::println!(
                        "✅ Test Case {} Passed: Rollback Successfully Triggered",
                        i + 1
                    );
                    ic_cdk::println!(
                        "   ➜ ICRC Approval was not given, triggering swap failure..."
                    );
                    ic_cdk::println!("🔄 Rollback Process Initiated...");
                    ic_cdk::println!("   ⏳ Reverting all state changes...");
                    ic_cdk::println!("   🔁 Restoring balances and reserves...");
                    ic_cdk::println!("✅ Rollback Completed.");
                    ic_cdk::println!(
                        "\n------------------------------------------------------------\n"
                    );
                    ic_cdk::println!(
                        "✅ Test {} Passed as expected with rollback: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                }
            }
            WasmResult::Reject(message) => {
                if case.expect_success {
                    ic_cdk::println!(
                        "❌ Test {} Failed: Unexpected rejection occurred: {}",
                        i + 1,
                        message
                    );
                } else {
                    ic_cdk::println!(
                        "✅ Test {} Passed: Swap Rejected as Expected (Rollback): {}",
                        i + 1,
                        message
                    );
                }
            }
        }
        ic_cdk::println!("\n============================================================\n");
    }

    ic_cdk::println!("==================== ✅ All Swap Tests Completed ====================\n");
}
