mod utils;

use candid::types::principal;
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
    let (pic, backend_canister, ckbtc_canister, lp_ledger_canister, cketh_canister, random_users) =
        setup();
    test_create_pools(
        &pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        lp_ledger_canister,
        random_users.clone(),
    );
    test_burn_lp_tokens(
        &pic,
        backend_canister,
        ckbtc_canister,
        lp_ledger_canister,
        cketh_canister,
        random_users.clone(),
    );
    test_swap(
        &pic,
        backend_canister,
        ckbtc_canister,
        cketh_canister,
        random_users,
    );
    // test_get_user_share_ratio(&pic, backend_canister, ckbtc_canister, cketh_canister);
}

fn setup() -> (
    PocketIc,
    Principal,
    Principal,
    Principal,
    Principal,
    Vec<Principal>,
) {
    ic_cdk::println!("Setting up Pocket IC...");

    let pic = PocketIc::new();
    ic_cdk::println!("Pocket IC setup complete.");

    let random_users: Vec<Principal> = generate_principals(4);

    // Deploy backend canister
    let backend_canister = pic.create_canister();
    ic_cdk::println!("backend canister = {}", backend_canister.to_text());
    pic.add_cycles(backend_canister, 2_000_000_000_000_000); // 2T Cycles
    let backend_wasm = fs::read(BACKEND_WASM).expect("Wasm file not found, run 'dfx build'.");
    pic.install_canister(backend_canister, backend_wasm, vec![], None);

    // Deploy CKBTC canister
    let ckbtc_canister = pic.create_canister();
    pic.add_cycles(ckbtc_canister, 2_000_000_000_000_000);
    let ckbtc_wasm = fs::read(CKBTC_WASM).expect("CKBTC Wasm not found");

    // Deploy LP Ledger canister
    let lp_ledger_canister = pic.create_canister();
    pic.add_cycles(lp_ledger_canister, 2_000_000_000_000);
    let lp_ledger_wasm = fs::read(LP_LEDGER_WASM).expect("LP Ledger Wasm not found");

    // Deploy CKETH canister
    let cketh_canister = pic.create_canister();
    pic.add_cycles(cketh_canister, 2_000_000_000_000);
    let cketh_wasm = fs::read(CKETH_WASM).expect("CKETH Wasm not found");

    // Generate initial balances for all random users
    let generate_balances = |amount: u128| -> Vec<(Account, Nat)> {
        random_users
            .iter()
            .map(|&user| {
                (
                    Account {
                        owner: user,
                        subaccount: None,
                    },
                    Nat::from(amount),
                )
            })
            .collect()
    };

    // ------------------------
    // CKBTC Init
    let ckbtc_args = InitArgs {
        token_symbol: "CKBTC".to_string(),
        token_name: "CKBTC".to_string(),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: generate_balances(10_000_000_000),
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
    let ckbtc_encoded = encode_args((LedgerArgument::Init(ckbtc_args),)).expect("encode failed");
    pic.install_canister(ckbtc_canister, ckbtc_wasm, ckbtc_encoded, None);
    println!("CKBTC canister: {}", ckbtc_canister);

    // ------------------------
    // LP Ledger Init
    let lp_args = InitArgs {
        token_symbol: "LP_Token".to_string(),
        token_name: "LP_Token".to_string(),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: generate_balances(10_000_000),
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
    let lp_encoded = encode_args((LedgerArgument::Init(lp_args),)).expect("encode failed");
    pic.install_canister(lp_ledger_canister, lp_ledger_wasm, lp_encoded, None);
    println!("LP Ledger canister: {}", lp_ledger_canister);

    // ------------------------
    // CKETH Init
    let cketh_args = InitArgs {
        token_symbol: "CKETH".to_string(),
        token_name: "CKETH".to_string(),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: generate_balances(10_000_000_000),
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
    let cketh_encoded = encode_args((LedgerArgument::Init(cketh_args),)).expect("encode failed");
    pic.install_canister(cketh_canister, cketh_wasm, cketh_encoded, None);
    println!("CKETH canister: {}", cketh_canister);

    (
        pic,
        backend_canister,
        ckbtc_canister,
        lp_ledger_canister,
        cketh_canister,
        random_users,
    )
}

fn test_create_pools(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
    lp_ledger_canister: Principal,
    random_users: Vec<Principal>,
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

    let test_cases: Vec<TestCase> = vec![TestCase {
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
        expect_success: true,
        expected_error_message: None,
    }
    ];

    set_canister_id(
        &pic,
        backend_canister,
        "lp_ledger".to_string(),
        lp_ledger_canister,
    );

    ic_cdk::println!(
        "\n======================== Starting Test create pools ========================\n"
    );

    for (i, case) in test_cases.iter().enumerate() {
        let mut msg_ids = Vec::new();

        ic_cdk::println!("\n============================================================");
        ic_cdk::println!("🔁 IC Test Case {}: Executing create_pools request", i + 1,);

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
            ic_cdk::println!("\n------------------------------------------------------------\n");
        }

        ic_cdk::println!("  ⚖ Swap Fee          : {}", case.pool_data.swap_fee);
        ic_cdk::println!("  ✅ Expect Success    : {}", case.expect_success);
        if let Some(err) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error    : {}", err);
        }
        ic_cdk::println!("\n------------------------------------------------------------\n");

        for user_principal in &random_users {
            let user = user_principal;

            for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
                icrc2_approve(pic, backend_canister, pool.ledger_canister_id, *user);
            }
            ic_cdk::println!("\n------------------------------------------------------------\n");

            let encoded_args = candid::encode_args((&case.pool_data,)).unwrap();
            let msg_id = pic
                .submit_call(
                    backend_canister,
                    *user,
                    "create_pools",
                    encoded_args.clone(),
                )
                .unwrap();

            msg_ids.push((msg_id, user));
        }

        for (idx, (msg_id, user)) in msg_ids.into_iter().enumerate() {
            let response = pic.await_call(msg_id).unwrap();

            // ic_cdk::println!("\n📨 Response for Attempt {} by User: {}", idx + 1, user.to_text());
            // ic_cdk::println!("Response: {:?}", response);

            match response {
                WasmResult::Reply(data) => {
                    let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                    if case.expect_success {
                        assert!(
                            result.is_ok(),
                            "❌ Attempt {} failed (User: {}): Expected success, got error: {:?}",
                            idx + 1,
                            user.to_text(),
                            result.unwrap_err()
                        );
                        ic_cdk::println!(
                            "✅ Attempt {} passed! Pool created successfully by User: {}.",
                            idx + 1,
                            user.to_text()
                        );
                    } else {
                        assert!(
                            result.is_err(),
                            "❌ Attempt {} failed (User: {}): Expected error, but got success.",
                            idx + 1,
                            user.to_text()
                        );
                        ic_cdk::println!(
                            "✅ Attempt {} passed as expected: {:?} (User: {})",
                            idx + 1,
                            result.unwrap_err(),
                            user.to_text()
                        );
                    }
                }
                WasmResult::Reject(message) => {
                    if case.expect_success {
                        ic_cdk::println!(
                            "❌ Attempt {} failed: Unexpected rejection: {} (User: {})",
                            idx + 1,
                            message,
                            user
                        );
                    } else {
                        ic_cdk::println!(
                            "✅ Attempt {} passed! Rejected as expected: {} (User: {})",
                            idx + 1,
                            message,
                            user
                        );
                    }
                }
            }
        }
        ic_cdk::println!("\n============================================================\n");
    }

    ic_cdk::println!(
        "\n======================== IC Create Pools Tests Completed ========================\n"
    );
}

fn test_burn_lp_tokens(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    lp_ledger_canister: Principal,
    cketh_canister: Principal,
    random_users: Vec<Principal>,
) {
    #[derive(Debug, Clone)]
    struct TestCase {
        pool_data: Pool_Data,
        pool_name: String,
        amount_to_burn: Nat,
        expect_success: bool,
        expected_error_message: Option<String>,
    }

    let test_cases: Vec<TestCase> = vec![
        // ✅ Valid burn
        TestCase {
            pool_data: Pool_Data {
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
                swap_fee: Nat::from(5u128),
            },
            pool_name: "ckbtccketh".to_string(),
            amount_to_burn: Nat::from(100u64),
            expect_success: true,
            expected_error_message: None,
        },
    ];

    ic_cdk::println!(
        "\n======================== Starting IC Burn Lp tokens Tests ========================\n"
    );

    for (i, case) in test_cases.iter().enumerate() {
        let mut msg_ids: Vec<(pocket_ic::common::rest::RawMessageId, &Principal)> = Vec::new();
        ic_cdk::println!("\n============================================================");
        ic_cdk::println!(
            "🔥 IC Test Case {}: Executing burn_lp_tokens request",
            i + 1
        );
        for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
            let contribution = pool.balance.clone() * pool.value.clone();

            ic_cdk::println!("  🧪 Pool {} Details:", j + 1);
            ic_cdk::println!("    ▸ Token Name      : {}", pool.token_name);
            ic_cdk::println!("    ▸ Balance         : {}", pool.balance);
            ic_cdk::println!("    ▸ Weight          : {}", pool.weight);
            ic_cdk::println!("    ▸ Value           : {}", pool.value);
            ic_cdk::println!("    ▸ Contribution    : {}", contribution);
            ic_cdk::println!("    ▸ Ledger Canister : {}", pool.ledger_canister_id);
            ic_cdk::println!("    ▸ Image           : {}", pool.image);
            ic_cdk::println!("\n------------------------------------------------------------\n");
        }

        ic_cdk::println!("  ⚖ Swap Fee              : {}", case.pool_data.swap_fee);
        ic_cdk::println!("  💠 Pool Name            : {}", case.pool_name);
        ic_cdk::println!("  🔥 Amount to Burn       : {}", case.amount_to_burn);
        ic_cdk::println!("  ✅ Expect Success       : {}", case.expect_success);
        if let Some(msg) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error      : {}", msg);
        }
        ic_cdk::println!("------------------------------------------------------------\n");

        for user_principal in &random_users {
            for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
                icrc2_approve(pic, backend_canister, lp_ledger_canister, *user_principal);
            }
            let encoded_args = candid::encode_args((
                case.pool_data.clone(),
                case.pool_name.clone(),
                case.amount_to_burn.clone(),
                lp_ledger_canister,
            ))
            .unwrap();

            let msg_id = pic
                .submit_call(
                    backend_canister,
                    *user_principal,
                    "burn_lp_tokens",
                    encoded_args,
                )
                .unwrap();
            msg_ids.push((msg_id, user_principal));
        }

        ic_cdk::println!("\n------------------------------------------------------------\n");

        for (idx, (msg_id, user)) in msg_ids.into_iter().enumerate() {
            let response = pic.await_call(msg_id).unwrap();
            match response {
                WasmResult::Reply(data) => {
                    let result: Result<(), String> = candid::decode_one(&data).unwrap();

                    if case.expect_success {
                        assert!(
                            result.is_ok(),
                            "❌ Test {} failed: Expected success but got error: {:?}",
                            i + 1,
                            result.unwrap_err()
                        );
                        ic_cdk::println!(
                            "✅ Test {} passed! LP tokens successfully burned.",
                            i + 1
                        );
                    } else {
                        assert!(
                            result.is_err(),
                            "❌ Test {} failed: Expected error but got success.",
                            i + 1
                        );
                        ic_cdk::println!(
                            "✅ Test {} passed! Error: {:?}",
                            i + 1,
                            result.unwrap_err()
                        );
                    }
                }
                WasmResult::Reject(msg) => {
                    if case.expect_success {
                        ic_cdk::println!("❌ Test {} failed: Unexpected rejection: {}", i + 1, msg);
                    } else {
                        ic_cdk::println!("✅ Test {} passed! Rejected as expected: {}", i + 1, msg);
                    }
                }
            }
        }
        ic_cdk::println!("\n============================================================\n");
    }
    ic_cdk::println!(
        "\n======================== IC Burn Lp Tokens Tests Completed ========================\n"
    );
}

pub fn test_swap(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
    random_users: Vec<Principal>,
) {
    let test_cases = vec![SwapTestCase {
        expect_success: true,
        expected_error_message: None,
        params: SwapParams {
            token1_name: "ckbtc".to_string(),
            token_amount: Nat::from(1000u128),
            token2_name: "cketh".to_string(),
            ledger_canister_id1: ckbtc_canister,
            ledger_canister_id2: cketh_canister,
            fee: Nat::from(3u64),
        },
    }];

    println!("\n======================== 🔁 Starting Swap Tests ========================\n");

    for (i, case) in test_cases.iter().enumerate() {
        println!("\n============================================================");
        println!("🔵 Test Case {}: Executing swap function ", i + 1);

        println!("📦 Swap Parameters:");
        println!("    ▸ Token 1 Name     : {}", case.params.token1_name);
        println!("    ▸ Token 2 Name     : {}", case.params.token2_name);
        println!("    ▸ Token Amount     : {}", case.params.token_amount);
        println!("    ▸ Fee              : {}", case.params.fee);
        println!(
            "    ▸ Ledger Canister1 : {}",
            case.params.ledger_canister_id1
        );
        println!(
            "    ▸ Ledger Canister2 : {}",
            case.params.ledger_canister_id2
        );

        println!("\n------------------------------------------------------------\n");

        for user_principal in &random_users {
            // Approve on both ledgers
            icrc2_approve(
                pic,
                backend_canister,
                case.params.ledger_canister_id1,
                *user_principal,
            );
            icrc2_approve(
                pic,
                backend_canister,
                case.params.ledger_canister_id2,
                *user_principal,
            );

            println!(
                "🔐 Approved backend for both tokens for user: {}",
                user_principal
            );

            let encoded_args = candid::encode_args((case.params.clone(),)).unwrap();

            let msg_id = pic
                .submit_call(
                    backend_canister,
                    *user_principal,
                    "compute_swap",
                    encoded_args,
                )
                .unwrap();

            let response = pic.await_call(msg_id);

            match response {
                Ok(WasmResult::Reply(data)) => {
                    let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                    if case.expect_success {
                        assert!(
                            result.is_ok(),
                            "❌ Test {} failed: Expected success, got error: {:?}",
                            i + 1,
                            result.unwrap_err()
                        );
                        println!("✅ Test {} passed! Swap executed successfully.", i + 1);
                    } else {
                        assert!(
                            result.is_err(),
                            "❌ Test {} failed: Expected rejection, but swap succeeded.",
                            i + 1
                        );
                        let expected_msg = case.expected_error_message.as_ref().unwrap();
                        println!(
                            "✅ Test {} passed! Swap failed as expected with message: {}",
                            i + 1,
                            expected_msg
                        );
                    }
                }
                Ok(WasmResult::Reject(message)) => {
                    if case.expect_success {
                        println!(
                            "❌ Test {} failed: Unexpected rejection occurred: {}",
                            i + 1,
                            message
                        );
                    } else {
                        println!(
                            "✅ Test {} passed! Swap call rejected as expected. Reason: {}",
                            i + 1,
                            message
                        );
                    }
                }
                Err(err) => {
                    println!("❌ Test {} failed due to error: {}", i + 1, err);
                }
            }

            println!("\n------------------------------------------------------------\n");
        }

        println!("\n============================================================\n");
    }

    println!("\n======================== ✅ All Swap Tests Completed ========================\n");
}

fn test_get_user_share_ratio(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
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
        pool_name: String,
        amount: Nat,
        expect_success: bool,
        expected_error_message: Option<String>,
    }

    let test_cases: Vec<TestCase> = vec![TestCase {
        pool_data: PoolData {
            pool_data: vec![
                CreatePoolParams {
                    token_name: "ckbtc".to_string(),
                    balance: Nat::from(200_000_000u128),
                    weight: Nat::from(50u128),
                    value: Nat::from(100u128),
                    ledger_canister_id: ckbtc_canister,
                    image: "btc.png".to_string(),
                },
                CreatePoolParams {
                    token_name: "cketh".to_string(),
                    balance: Nat::from(200_000_000u128),
                    weight: Nat::from(50u128),
                    value: Nat::from(100u128),
                    ledger_canister_id: cketh_canister,
                    image: "eth.png".to_string(),
                },
            ],
            swap_fee: Nat::from(3u128),
        },
        pool_name: "ckbtccketh".to_string(),
        amount: Nat::from(300_000u128),
        expect_success: true,
        expected_error_message: None,
    }];

    let user = get_user_principal();

    ic_cdk::println!(
        "\n======================== Starting Test: get_user_share_ratio ========================\n"
    );

    for (i, test) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n============================================================");
        ic_cdk::println!("🔵 Test Case {}: Pool Name = {}", i + 1, test.pool_name);
        ic_cdk::println!("============================================================");

        // Log test config
        for (j, pool) in test.pool_data.pool_data.iter().enumerate() {
            ic_cdk::println!("  Pool {}:", j + 1);
            ic_cdk::println!("    ▸ Token: {}", pool.token_name);
            ic_cdk::println!("    ▸ Balance: {}", pool.balance);
            ic_cdk::println!("    ▸ Weight: {}", pool.weight);
            ic_cdk::println!("    ▸ Value: {}", pool.value);
        }
        ic_cdk::println!("  ➤ Swap Fee: {}", test.pool_data.swap_fee);
        ic_cdk::println!("  ➤ Amount: {}", test.amount);
        ic_cdk::println!("  ➤ Expect Success: {}", test.expect_success);

        // Approve tokens
        // icrc2_approve(pic, backend_canister, ckbtc_canister);
        // icrc2_approve(pic, backend_canister, cketh_canister);

        // Encode all three arguments
        let encoded_args =
            candid::encode_args((&test.pool_data, &test.pool_name, &test.amount)).unwrap();

        let response = pic
            .update_call(backend_canister, user, "get_user_share_ratio", encoded_args)
            .unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<Vec<Nat>, String> = candid::decode_one(&data).unwrap();

                if test.expect_success {
                    assert!(
                        result.is_ok(),
                        "❌ Test {} failed: Expected success, got error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                    ic_cdk::println!(
                        "✅ Test {} passed! Received response: {:?}",
                        i + 1,
                        result.unwrap()
                    );
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected error, got success: {:?}",
                        i + 1,
                        result.unwrap()
                    );
                    ic_cdk::println!(
                        "✅ Test {} passed! Got expected error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                }
            }
            WasmResult::Reject(msg) => {
                if test.expect_success {
                    ic_cdk::println!("❌ Test {} failed: Unexpected rejection: {}", i + 1, msg);
                } else {
                    ic_cdk::println!("✅ Test {} passed: Rejected as expected: {}", i + 1, msg);
                }
            }
        }
    }

    ic_cdk::println!("\n======================== Test Completed ========================\n");
}
