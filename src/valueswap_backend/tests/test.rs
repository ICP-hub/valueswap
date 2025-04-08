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
    std::env::set_var("POCKET_IC_BIN", "/Users/admin/Documents/Projects/ICP/valueswap/src/valueswap_backend/tests/pocket-ic"); // Path of the pocket-ic binary
    let (pic, backend_canister, ckbtc_canister, lp_ledger_canister,cketh_canister) = setup();
    test_create_pools(&pic, backend_canister, ckbtc_canister, cketh_canister);
    // test_burn_lp_tokens(&pic, backend_canister, ckbtc_canister, lp_ledger_canister, cketh_canister);
    // test_swap(&pic, backend_canister, ckbtc_canister, cketh_canister);
    test_get_user_share_ratio(&pic, backend_canister, ckbtc_canister, cketh_canister);
}

fn setup() -> (PocketIc, Principal, Principal, Principal, Principal) {
    std::env::set_var("POCKET_IC_BIN", "/Users/admin/Documents/Projects/ICP/valueswap/src/valueswap_backend/tests/pocket-ic"); // Path of the pocket-ic binary
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
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![(
            Account {
                owner: get_user_principal(),
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

    ic_cdk::println!("LP Ledger canister: {}", lp_ledger_canister);

    // Define the initialization arguments for the LP Ledger canister
    let lp_ledger_args = InitArgs {
        token_symbol: String::from("LP_Token"),
        token_name: String::from("LP_Token"),
        transfer_fee: Nat::from(100u64),
        metadata: vec![],
        minting_account: Account {
            owner: backend_canister,
            subaccount: None,
        },
        initial_balances: vec![(
            Account {
                owner: get_user_principal(),
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

    let args = InitArgs {
        token_symbol: String::from("CKETH"),
        token_name: String::from("CKETH"),
        transfer_fee: Nat::from(100u64),
        metadata: vec![], 
        minting_account: Account {
            owner : backend_canister,
            subaccount: None,
        },
        initial_balances: vec![
            (Account {
                owner: get_user_principal(),
                subaccount: None,
            }, Nat::from(10_000_000_000u64)) 
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
    
    let args_encoded = encode_args((LedgerArgument::Init(args),))
        .expect("Failed to encode arguments");

    
    pic.install_canister(cketh_canister, cketh_wasm, args_encoded, None);
    println!("CKETH canister: {}", cketh_canister);

    (pic, backend_canister, ckbtc_canister, lp_ledger_canister, cketh_canister)
}

fn test_create_pools(pic: &PocketIc, backend_canister: Principal, ckbtc_canister: Principal, cketh_canister: Principal   ) {
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
        // ✅ Valid: Equal weights, proper values
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "ckbtc".to_string(),
                        balance: Nat::from(200_000_00u128),
                        weight: Nat::from(50u128),
                        value: Nat::from(100u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "btc.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "cketh".to_string(),
                        balance: Nat::from(200_000u128),
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

// ___________________________________________________________________________________

        // TestCase {
        //     pool_data: PoolData {
        //         pool_data: vec![
        //             CreatePoolParams {
        //                 token_name: "zero-val".to_string(),
        //                 balance: Nat::from(150_000_000u128),
        //                 weight: Nat::from(50u128),
        //                 value: Nat::from(0u128),
        //                 ledger_canister_id: ckbtc_canister,
        //                 image: "zero.png".to_string(),
        //             },
        //             CreatePoolParams {
        //                 token_name: "valid-val".to_string(),
        //                 balance: Nat::from(200_000_000u128),
        //                 weight: Nat::from(50u128),
        //                 value: Nat::from(120u128),
        //                 ledger_canister_id: cketh_canister,
        //                 image: "valid.png".to_string(),
        //             },
        //         ],
        //         swap_fee: Nat::from(2u128),
        //     },
        //     expect_success: false,
        //     expected_error_message: Some("Invalid value: value cannot be zero.".to_string()),
        // },
        // ❌ Invalid: One token balance = 0
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "zero-bal".to_string(),
                        balance: Nat::from(0u128),
                        weight: Nat::from(40u128),
                        value: Nat::from(100u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "zero-bal.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "valid-bal".to_string(),
                        balance: Nat::from(180_000_000u128),
                        weight: Nat::from(60u128),
                        value: Nat::from(120u128),
                        ledger_canister_id: cketh_canister,
                        image: "valid-bal.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(1u128),
            },
            expect_success: false,
            expected_error_message: Some("Invalid balance: balance cannot be zero.".to_string()),
        },
        // ❌ Invalid: One token weight = 0
        TestCase {
            pool_data: PoolData {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "zero-weight".to_string(),
                        balance: Nat::from(200_000_000u128),
                        weight: Nat::from(0u128),
                        value: Nat::from(100u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "zero-weight.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "valid-weight".to_string(),
                        balance: Nat::from(150_000_000u128),
                        weight: Nat::from(100u128),
                        value: Nat::from(90u128),
                        ledger_canister_id: cketh_canister,
                        image: "valid-weight.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(1u128),
            },
            expect_success: false,
            expected_error_message: Some("Invalid weight: weight must be non-zero.".to_string()),
        },
        // ❌ Invalid: value is zero
        // TestCase {
        //     pool_data: PoolData {
        //         pool_data: vec![CreatePoolParams {
        //             token_name: "ZeroValueToken".to_string(),
        //             balance: Nat::from(100u128),
        //             weight: Nat::from(10u128),
        //             value: Nat::from(0u128),
        //             ledger_canister_id: ckbtc_canister,
        //             image: "zero.png".to_string(),
        //         }],
        //         swap_fee: Nat::from(2u128),
        //     },
        //     expect_success: false,
        //     expected_error_message: Some("Invalid value: value cannot be zero.".to_string()),
        // },
        // // ❌ Invalid: balance is zero
        // TestCase {
        //     pool_data: PoolData {
        //         pool_data: vec![CreatePoolParams {
        //             token_name: "ZeroBalance".to_string(),
        //             balance: Nat::from(0u128),
        //             weight: Nat::from(5u128),
        //             value: Nat::from(100u128),
        //             ledger_canister_id: ckbtc_canister,
        //             image: "zero_balance.png".to_string(),
        //         }],
        //         swap_fee: Nat::from(1u128),
        //     },
        //     expect_success: false,
        //     expected_error_message: Some("Invalid balance: balance cannot be zero.".to_string()),
        // },
        // // ❌ Invalid: weight is zero
        // TestCase {
        //     pool_data: PoolData {
        //         pool_data: vec![CreatePoolParams {
        //             token_name: "ZeroWeight".to_string(),
        //             balance: Nat::from(100u128),
        //             weight: Nat::from(0u128),
        //             value: Nat::from(100u128),
        //             ledger_canister_id: ckbtc_canister,
        //             image: "zero_weight.png".to_string(),
        //         }],
        //         swap_fee: Nat::from(1u128),
        //     },
        //     expect_success: false,
        //     expected_error_message: Some("Invalid weight: weight must be non-zero.".to_string()),
        // },
    ];

    let hardcoded_principal = get_user_principal();

    ic_cdk::println!(
        "\n======================== Starting Test create pools ========================\n"
    );

    for (i, case) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n============================================================");
        ic_cdk::println!("🔵 IC Test Case {}: Executing create_pools request", i + 1);
        ic_cdk::println!("============================================================");

        for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
            ic_cdk::println!("  🧪 Pool {} Details:", j + 1);
            ic_cdk::println!("    ▸ Token Name      : {}", pool.token_name);
            ic_cdk::println!("    ▸ Balance         : {}", pool.balance);
            ic_cdk::println!("    ▸ Weight          : {}", pool.weight);
            ic_cdk::println!("    ▸ Value           : {}", pool.value);
            ic_cdk::println!("    ▸ Ledger Canister : {}", pool.ledger_canister_id);
            ic_cdk::println!("    ▸ Image           : {}", pool.image);

            icrc2_approve(pic, backend_canister, pool.ledger_canister_id);
            ic_cdk::println!("\n------------------------------------------------------------\n");
        }

        ic_cdk::println!("  ⚖ Swap Fee          : {}", case.pool_data.swap_fee);
        ic_cdk::println!("  ✅ Expect Success    : {}", case.expect_success);
        if let Some(err) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error    : {}", err);
        }
        ic_cdk::println!("------------------------------------------------------------\n");

        // Approve tokens before pool creation
        // icrc2_approve(pic, backend_canister, ckbtc_canister);
        // icrc2_approve(pic, backend_canister, cketh_canister);

        let encoded_args = candid::encode_args((&case.pool_data,)).unwrap();
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

                if case.expect_success {
                    assert!(
                        result.is_ok(),
                        "❌ Test {} failed: Expected success, got error: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                    ic_cdk::println!("✅ Test {} passed! Pool created successfully.", i + 1);
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected error, but got success.",
                        i + 1
                    );
                    ic_cdk::println!(
                        "✅ Test {} passed as expected: {:?}",
                        i + 1,
                        result.unwrap_err()
                    );
                }
            }
            WasmResult::Reject(message) => {
                if case.expect_success {
                    ic_cdk::println!(
                        "❌ Test {} failed: Unexpected rejection: {}",
                        i + 1,
                        message
                    );
                } else {
                    ic_cdk::println!(
                        "✅ Test {} passed! Rejected as expected: {}",
                        i + 1,
                        message
                    );
                }
            }
        }
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
    cketh_canister: Principal
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
                        value: Nat::from(100u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "btc.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "cketh".to_string(),
                        balance: Nat::from(200_000u128),
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
        // ✅ Valid burn with different amount
        // TestCase {
        //     pool_data: Pool_Data {
        //        pool_data: vec![
        //             CreatePoolParams {
        //                 token_name: "btc-heavy".to_string(),
        //                 balance: Nat::from(300_000u128),
        //                 weight: Nat::from(70u128),
        //                 value: Nat::from(150u128),
        //                 ledger_canister_id: ckbtc_canister,
        //                 image: "btc-heavy.png".to_string(),
        //             },
        //             CreatePoolParams {
        //                 token_name: "eth-light".to_string(),
        //                 balance: Nat::from(100_000u128),
        //                 weight: Nat::from(30u128),
        //                 value: Nat::from(90u128),
        //                 ledger_canister_id: cketh_canister,
        //                 image: "eth-light.png".to_string(),
        //             },
        //         ],
        //         swap_fee: Nat::from(2u128),
        //     },
        //     pool_name: "btc-heavyeth-light".to_string(),
        //     amount_to_burn: Nat::from(500u64),
        //     expect_success: true,
        //     expected_error_message: None,
        // },
        // ❌ Invalid: amount_to_burn = 0
        TestCase {
            pool_data: Pool_Data {
                pool_data: vec![
                    CreatePoolParams {
                        token_name: "zero-val".to_string(),
                        balance: Nat::from(150_000_000u128),
                        weight: Nat::from(50u128),
                        value: Nat::from(0u128),
                        ledger_canister_id: ckbtc_canister,
                        image: "zero.png".to_string(),
                    },
                    CreatePoolParams {
                        token_name: "valid-val".to_string(),
                        balance: Nat::from(200_000_000u128),
                        weight: Nat::from(50u128),
                        value: Nat::from(120u128),
                        ledger_canister_id: cketh_canister,
                        image: "valid.png".to_string(),
                    },
                ],
                swap_fee: Nat::from(1u128),
            },
            pool_name: "zero-valvalid-val".to_string(),
            amount_to_burn: Nat::from(0u64),
            expect_success: false,
            expected_error_message: Some("Amount to burn must be greater than zero.".to_string()),
        },
        // ❌ Invalid: pool does not exist
        TestCase {
            pool_data: Pool_Data {
                pool_data: vec![], // Empty pool_data
                swap_fee: Nat::from(2u128),
            },
            pool_name: "NonExistentPool".to_string(),
            amount_to_burn: Nat::from(10u64),
            expect_success: false,
            expected_error_message: Some("Pool not found.".to_string()),
        },
        // ❌ Invalid: insufficient LP token balance
        TestCase {
            pool_data: Pool_Data {
                pool_data: vec![CreatePoolParams {
                    token_name: "LowBalanceToken".to_string(),
                    balance: Nat::from(100u128),
                    weight: Nat::from(10u128),
                    value: Nat::from(100u128),
                    ledger_canister_id: ckbtc_canister,
                    image: "low_balance.png".to_string(),
                }],
                swap_fee: Nat::from(2u128),
            },
            pool_name: "LowBalanceToken".to_string(),
            amount_to_burn: Nat::from(10_000u64),
            expect_success: false,
            expected_error_message: Some("Insufficient LP token balance.".to_string()),
        },
    ];

    let user = get_user_principal();

    ic_cdk::println!(
        "\n======================== Starting IC Burn Lp tokens Tests ========================\n"
    );

    for (i, case) in test_cases.iter().enumerate() {
        ic_cdk::println!("\n============================================================");
        ic_cdk::println!("🔥 IC Test Case {}: Executing burn_lp_tokens request", i + 1);
        ic_cdk::println!("============================================================");
        for (j, pool) in case.pool_data.pool_data.iter().enumerate() {
            ic_cdk::println!("  🧪 Pool {} Details:", j + 1);
            ic_cdk::println!("    ▸ Token Name      : {}", pool.token_name);
            ic_cdk::println!("    ▸ Balance         : {}", pool.balance);
            ic_cdk::println!("    ▸ Weight          : {}", pool.weight);
            ic_cdk::println!("    ▸ Value           : {}", pool.value);
            ic_cdk::println!("    ▸ Ledger Canister : {}", pool.ledger_canister_id);
            ic_cdk::println!("    ▸ Image           : {}", pool.image);
        }

        ic_cdk::println!("\n------------------------------------------------------------\n");

        ic_cdk::println!("  ⚖ Swap Fee              : {}", case.pool_data.swap_fee);
        ic_cdk::println!("  💠 Pool Name            : {}", case.pool_name);
        ic_cdk::println!("  🔥 Amount to Burn       : {}", case.amount_to_burn);
        ic_cdk::println!("  ✅ Expect Success       : {}", case.expect_success);
        if let Some(msg) = &case.expected_error_message {
            ic_cdk::println!("  ❗ Expected Error      : {}", msg);
        }
        ic_cdk::println!("------------------------------------------------------------\n");

        // Approve LP tokens before burning
        icrc2_approve(pic, backend_canister, lp_ledger_canister);

        let encoded_args = candid::encode_args((
            case.pool_data.clone(),
            case.pool_name.clone(),
            case.amount_to_burn.clone(),
            lp_ledger_canister,
        ))
        .unwrap();

        let response = pic
            .update_call(backend_canister, user, "burn_lp_tokens", encoded_args)
            .unwrap();

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
                    ic_cdk::println!("✅ Test {} passed! LP tokens successfully burned.", i + 1);
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
    ic_cdk::println!(
        "\n======================== IC Burn Lp Tokens Tests Completed ========================\n"
    );
}


// #[test]
fn test_swap(pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    cketh_canister:Principal ) {

    let spender_canister = backend_canister; 


    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(10000000000000u128),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: spender_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    let response = pic.update_call(
        ckbtc_canister,
        get_user_principal(),
        "icrc2_approve",
        encoded_args.clone(),
    ).expect("Failed to approve CKBTC");

    let response2 = pic.update_call(
        cketh_canister,
        get_user_principal(),
        "icrc2_approve",
        encoded_args,
    ).expect("Failed to approve CKETH");


    match response {
        WasmResult::Reply(data) => {
            
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();

            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(allowance > Nat::from(0u64), "Allowance should be greater than zero.");
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }
    
    match response2 {
        WasmResult::Reply(data) => {
            
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();

            match result {
                Ok(allowance) => {
                    println!("Approval successful. Allowance set to: {:?}", allowance);
                    assert!(allowance > Nat::from(0u64), "Allowance should be greater than zero.");
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }


    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000000u128),
                weight: Nat::from(50u128),
                value: Nat::from(100u128),
                ledger_canister_id: ckbtc_canister,
                image: "image.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(2800000000u128),
                weight: Nat::from(50u128),
                value: Nat::from(100u128),
                ledger_canister_id: cketh_canister,
                image: "image.png".to_string(),
            },
        ],
        swap_fee: Nat::from(5u128),
    };

    let encoded_args = candid::encode_args((pool_data,)).unwrap();

    let response = pic.update_call(
        backend_canister,
        get_user_principal(),
        "create_pools",
        encoded_args,
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(result.is_ok(), "Expected successful pool creation, got {:?}", result);
        },
        WasmResult::Reject(message) => panic!("Failed to create pools: {}", message),
    }

    let swap_params = SwapParams {
        token1_name: "ckbtc".to_string(),
        token_amount: Nat::from(100000u128),
        token2_name: "cketh".to_string(),
        ledger_canister_id1: ckbtc_canister,
        ledger_canister_id2: cketh_canister,
        fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((swap_params,)).unwrap();

    let response = pic.update_call(
        backend_canister,
        get_user_principal(),
        "compute_swap",
        encoded_args,
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = candid::decode_one(&data).unwrap();
            assert!(result.is_ok(), "Expected successful swap, got {:?}", result);
            println!("Swap successful.");
        },
        WasmResult::Reject(message) => {
            panic!("Swap failed with message: {}", message);
        }
    }

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
        let encoded_args = candid::encode_args((&test.pool_data, &test.pool_name, &test.amount)).unwrap();

        let response = pic
            .update_call(
                backend_canister,
                user,
                "get_user_share_ratio",
                encoded_args,
            )
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
                    ic_cdk::println!("✅ Test {} passed! Received response: {:?}", i + 1, result.unwrap());
                } else {
                    assert!(
                        result.is_err(),
                        "❌ Test {} failed: Expected error, got success: {:?}",
                        i + 1,
                        result.unwrap()
                    );
                    ic_cdk::println!("✅ Test {} passed! Got expected error: {:?}", i + 1, result.unwrap_err());
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

    ic_cdk::println!(
        "\n======================== Test Completed ========================\n"
    );
}





