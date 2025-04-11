mod utils;
use candid::{encode_args, CandidType, Nat, Principal};
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
    test_burn_lp_tokens(
        &pic,
        backend_canister,
        ckbtc_canister,
        lp_ledger_canister,
        cketh_canister,
    );
    test_swap(&pic, backend_canister, ckbtc_canister, cketh_canister);
    // test_get_user_share_ratio(&pic, backend_canister, ckbtc_canister, cketh_canister);
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

    let test_case = PoolData {
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
    };
    let user_principal = get_user_principal();
    set_canister_id(pic, backend_canister, "lp_ledger".to_string(), lp_ledger_canister);

    ic_cdk::println!(
        "\n===================== Testing Mutex (Single User Concurrency) ====================="
    );
    for (j, pool) in test_case.pool_data.iter().enumerate() {
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

    // ✅ Approve first
    for pool in &test_case.pool_data {
        icrc2_approve(pic, backend_canister, pool.ledger_canister_id, user_principal);
    }

    // 🌀 Submit multiple concurrent calls for the same user
    let mut msg_ids = Vec::new();
    let encoded_args = candid::encode_args((&test_case,)).unwrap();

    ic_cdk::println!("\n------------------------------------------------------------\n");

    for i in 0..5 {
        ic_cdk::println!("🚀 Submitting request #{} from same user...", i + 1);
        let msg_id = pic
            .submit_call(backend_canister, user_principal, "create_pools", encoded_args.clone())
            .unwrap();
        msg_ids.push(msg_id);
    }

    ic_cdk::println!("\n------------------------------------------------------------\n");

    // ⏳ Await responses
    for (i, msg_id) in msg_ids.into_iter().enumerate() {
        let response = pic.await_call(msg_id).unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();

                match result {
                    Ok(_) => {
                        ic_cdk::println!(
                            "✅ Response #{}: Success! Pool created by user: {}.",
                            i + 1,
                            user_principal.to_text()
                        );
                    }
                    Err(err) => {
                        ic_cdk::println!(
                            "⚠️ Response #{}: Error (User: {}): {:?}",
                            i + 1,
                            user_principal.to_text(),
                            err
                        );
                    }
                }
            }
            WasmResult::Reject(msg) => {
                ic_cdk::println!(
                    "❌ Response #{}: Call rejected: {} (User: {})",
                    i + 1,
                    msg,
                    user_principal
                );
            }
        }
    }

    ic_cdk::println!(
        "\n================== Mutex Safety Test Completed ==================\n"
    );
}

fn test_burn_lp_tokens(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    lp_ledger_canister: Principal,
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
    struct Pool_Data {
        pool_data: Vec<CreatePoolParams>,
        swap_fee: Nat,
    }

    #[derive(Debug, Clone)]
    struct TestCase {
        pool_data: Pool_Data,
        pool_name: String,
        amount_to_burn: Nat,
    }

    let test_case = TestCase {
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
    };

    let user_principal = get_user_principal();

    ic_cdk::println!(
        "\n==================== Starting Mutex Burn LP Tokens Test ====================\n"
    );

    for (j, pool) in test_case.pool_data.pool_data.iter().enumerate() {
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

    // ✅ Approve LP ledger canister first
    icrc2_approve(pic, backend_canister, lp_ledger_canister, user_principal);

    // 🌀 Submit multiple concurrent burn_lp_tokens calls for same user
    let mut msg_ids = Vec::new();
    let encoded_args = candid::encode_args((
        test_case.pool_data.clone(),
        test_case.pool_name.clone(),
        test_case.amount_to_burn.clone(),
        lp_ledger_canister,
    ))
    .unwrap();

    ic_cdk::println!("\n------------------------------------------------------------\n");

    for i in 0..5 {
        ic_cdk::println!("🔥 Submitting burn request #{} from same user...", i + 1);
        let msg_id = pic
            .submit_call(
                backend_canister,
                user_principal,
                "burn_lp_tokens",
                encoded_args.clone(),
            )
            .unwrap();
        msg_ids.push(msg_id);
    }

    ic_cdk::println!("\n------------------------------------------------------------\n");

    for (i, msg_id) in msg_ids.into_iter().enumerate() {
        let response = pic.await_call(msg_id).unwrap();

        match response {
            WasmResult::Reply(data) => {
                let result: Result<(), String> = candid::decode_one(&data).unwrap();
                match result {
                    Ok(_) => {
                        ic_cdk::println!(
                            "✅ Response #{}: LP tokens burned successfully by user: {}.",
                            i + 1,
                            user_principal.to_text()
                        );
                    }
                    Err(err) => {
                        ic_cdk::println!(
                            "⚠️ Response #{}: Error while burning (User: {}): {}",
                            i + 1,
                            user_principal.to_text(),
                            err
                        );
                    }
                }
            }
            WasmResult::Reject(msg) => {
                ic_cdk::println!(
                    "❌ Response #{}: Rejected burn call: {} (User: {})",
                    i + 1,
                    msg,
                    user_principal
                );
            }
        }
    }

    ic_cdk::println!(
        "\n================== Mutex Burn LP Tokens Test Completed ==================\n"
    );
}

pub fn test_swap(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister: Principal,
) {
    let test_cases = vec![
        SwapTestCaseMutex {
            params: SwapParams {
                token1_name: "ckbtc".to_string(),
                token_amount: Nat::from(1000u128),
                token2_name: "cketh".to_string(),
                ledger_canister_id1: ckbtc_canister,
                ledger_canister_id2: cketh_canister,
                fee: Nat::from(3u64),
            },
        },
        // More cases can be added here
    ];

    println!("\n======================== 🔁 Starting Mutex Swap Tests ========================\n");

    for (i, case) in test_cases.iter().enumerate() {
        println!("\n============================================================");
        println!("🔵 Test Case {}: Executing multiple swap requests for same user", i + 1);

        println!("📦 Swap Parameters:");
        println!("    ▸ Token 1 Name     : {}", case.params.token1_name);
        println!("    ▸ Token 2 Name     : {}", case.params.token2_name);
        println!("    ▸ Token Amount     : {}", case.params.token_amount);
        println!("    ▸ Fee              : {}", case.params.fee);
        println!("    ▸ Ledger Canister1 : {}", case.params.ledger_canister_id1);
        println!("    ▸ Ledger Canister2 : {}", case.params.ledger_canister_id2);
        println!("------------------------------------------------------------\n");

        let user = get_user_principal();

        // Approve tokens for user
        icrc2_approve(pic, backend_canister, case.params.ledger_canister_id1, user);
        icrc2_approve(pic, backend_canister, case.params.ledger_canister_id2, user);
        // println!("🔐 Approved backend for both tokens for user: {}", user);

        println!("\n------------------------------------------------------------\n");

        let mut msg_ids = vec![];

        // Submit multiple swap calls for the same user
        for attempt in 1..=3 {
            let encoded_args = candid::encode_args((case.params.clone(),)).unwrap();

            let msg_id = pic
                .submit_call(
                    backend_canister,
                    user,
                    "compute_swap",
                    encoded_args,
                )
                .unwrap();

            println!("📨 Swap Request {} submitted for user", attempt);
            msg_ids.push((attempt, msg_id));
        }

        
        println!("\n------------------------------------------------------------\n");

        // Await and log responses
        for (attempt, msg_id) in msg_ids {
            let response = pic.await_call(msg_id);

            match response {
                Ok(WasmResult::Reply(data)) => {
                    let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
                    match result {
                        Ok(_) => println!("✅ Test {} Attempt {}: Swap executed successfully.", i + 1, attempt),
                        Err(err) => println!("⚠️  Test {} Attempt {}: Swap failed as expected with error: {:?}", i + 1, attempt, err),
                    }
                }
                Ok(WasmResult::Reject(message)) => {
                    println!("⚠️  Test {} Attempt {}: Swap rejected: {}", i + 1, attempt, message);
                }
                Err(err) => {
                    println!("❌ Test {} Attempt {}: Execution error: {}", i + 1, attempt, err);
                }
            }
        }

        println!("\n============================================================\n");
    }

    println!("\n======================== ✅ All Mutex Swap Tests Completed ========================\n");
}
