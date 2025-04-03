use candid::{encode_args, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use std::fs;
mod utils;
use crate::utils::structs::*;

const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";
const CKETH_WASM: &str = "../../.dfx/local/canisters/cketh/cketh.wasm.gz";
const LP_LEDGER_WASM: &str =
    "../../.dfx/local/canisters/LP_ledger_canister/LP_ledger_canister.wasm.gz";

fn setup() -> (PocketIc, Principal, Principal, Principal, Principal) {
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

    (
        pic,
        backend_canister,
        ckbtc_canister,
        lp_ledger_canister,
        cketh_canister,
    )
}

#[test]
fn test_create_pools() {
    let (pic, backend_canister, ckbtc_canister, _lp_ledger_canister, cketh_canister) = setup();

    //icrc2_approve for ckbtc ledger
    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();
    let spender_canister = backend_canister;

    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(10000000u64),
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
    //icrc2_approve for cketh ledger
    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();
    let spender_canister = backend_canister;

    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(10000000u64),
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
            cketh_canister,
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

    //create pool test
    let pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "ckbtc".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "image.png".to_string(),
            },
            CreatePoolParams {
                token_name: "cketh".to_string(),
                balance: Nat::from(100000u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: cketh_canister, // Ensure cketh_canister is correctly defined and accessible in this scope
                image: "image1.png".to_string(),
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
    //2nd approval

    // let approval_args = ApproveArgs {
    //     fee: None,
    //     memo: None,
    //     from_subaccount: None,
    //     created_at_time: None,
    //     amount: Nat::from(10000000u64),
    //     expected_allowance: None,
    //     expires_at: None,
    //     spender: Account {
    //         owner: spender_canister,
    //         subaccount: None,
    //     },
    // };

    // let encoded_args = candid::encode_args((approval_args,)).unwrap();

    // let response = pic
    //     .update_call(
    //         lp_ledger_canister,
    //         hardcoded_principal,
    //         "icrc2_approve",
    //         encoded_args,
    //     )
    //     .unwrap();

    // match response {
    //     WasmResult::Reply(data) => {
    //         let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();

    //         match result {
    //             Ok(allowance) => {
    //                 println!("Approval successful. Allowance set to: {:?}", allowance);
    //                 assert!(
    //                     allowance > Nat::from(0u64),
    //                     "Allowance should be greater than zero."
    //                 );
    //             }
    //             Err(e) => panic!("Approval failed with error: {:?}", e),
    //         }
    //     }
    //     WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    // }

    // //burn lp tokens test

    // let pool_name = "ckbtccketh".to_string(); // Assuming this pool was successfully created
    // let amount_to_burn = Nat::from(5u64); // Example amount to burn
    // let ledger_canister_id = lp_ledger_canister;

    // let pool_data = Pool_Data {
    //     pool_data: vec![
    //         CreatePoolParams {
    //             token_name: "ckbtc".to_string(),
    //             balance: Nat::from(100000u64),
    //             weight: Nat::from(10u64),
    //             value: Nat::from(100u64),
    //             ledger_canister_id: ckbtc_canister,
    //             image: "image.png".to_string(),
    //         },
    //         CreatePoolParams {
    //             token_name: "cketh".to_string(),
    //             balance: Nat::from(100000u64),
    //             weight: Nat::from(10u64),
    //             value: Nat::from(100u64),
    //             ledger_canister_id: cketh_canister, // Ensure cketh_canister is correctly defined and accessible in this scope
    //             image: "image1.png".to_string(),
    //         },
    //     ],
    //     swap_fee: Nat::from(5u64),
    // };

    // // Now include the ledger_canister_id in the arguments
    // let encoded_args =
    //     candid::encode_args((pool_data, pool_name, amount_to_burn, ledger_canister_id)).unwrap();

    // let response = pic
    //     .update_call(
    //         backend_canister,
    //         hardcoded_principal, // Typically, the user who calls the function, change as needed
    //         "burn_lp_tokens",
    //         encoded_args,
    //     )
    //     .unwrap();

    // match response {
    //     WasmResult::Reply(data) => {
    //         let result: Result<(), String> = candid::decode_one(&data).unwrap();
    //         assert!(
    //             result.is_ok(),
    //             "Expected successful LP token burning, got {:?}",
    //             result
    //         );
    //         println!("LP tokens successfully burned.");
    //     }
    //     WasmResult::Reject(message) => panic!("Failed to burn LP tokens: {}", message),
    // }
}
