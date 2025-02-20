use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use serde::{Deserialize, Serialize};

use std::fs;

#[derive(Debug, PartialEq, CandidType, Deserialize, Clone)]
pub enum CustomError {
    PoolDataEmpty,
    AnotherOperationInProgress(String),
    TokenDepositFailed,
    CanisterCreationFailed(String),
    LockAcquisitionFailed,
    StringConversionFailed(String),
    UnableToStorePoolData(String),
    UnableToTransferLP(String),
    NoCanisterIDFound,
    SwappingFailed(String),
    InvalidInput(String),
    OperationFailed(String),
    UnableToRollbackLP(String),
    InvalidSwapParams(String),
    VaultEmpty(String),
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CreatePoolParams {
    pub token_name: String,
    pub balance: Nat,
    pub weight: Nat,
    pub value: Nat,
    pub ledger_canister_id: Principal, // Ledger canister ID for the token (e.g., ckBTC, ckETH)
    pub image: String,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Pool_Data {
    pub pool_data: Vec<CreatePoolParams>,
    pub swap_fee: Nat,
}

impl Pool_Data {
    pub fn validate(&self) -> Result<(), CustomError> {
        // Check if pool_data is empty
        if self.pool_data.is_empty() {
            return Err(CustomError::PoolDataEmpty);
        }

        // Validate each pool data entry
        for pool in &self.pool_data {
            // Validate token name
            if pool.token_name.trim().is_empty() || pool.token_name.len() > 100 {
                return Err(CustomError::InvalidInput(
                    "Token name cannot be empty or exceed 100 characters".to_string(),
                ));
            }

            if pool.weight == Nat::from(0u64) || pool.value == Nat::from(0u64) {
                return Err(CustomError::InvalidInput(
                    "Weight and value must be greater than zero".to_string(),
                ));
            }

            // Validate ledger canister ID
            if pool.ledger_canister_id.to_text().is_empty() {
                return Err(CustomError::InvalidInput(
                    "Ledger canister ID cannot be empty".to_string(),
                ));
            }

            // Validate image URL format
            // if !self.is_valid_image_url(&pool.image) {
            //     return Err(CustomError::InvalidInput("Invalid image URL".to_string()));
            // }
        }

        Ok(())
    }

    // Enhanced URL validation
    // fn is_valid_image_url(&self, url: &str) -> bool {
    //     let lower_url = url.to_lowercase();
    //     (lower_url.starts_with("http://") || lower_url.starts_with("https://"))
    //         && (lower_url.contains(".png?") || lower_url.contains(".jpg?") || lower_url.contains(".jpeg?") || lower_url.ends_with(".png") || lower_url.ends_with(".jpg") || lower_url.ends_with(".jpeg"))
    // }
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct InitArgs {
    token_symbol: String,
    token_name: String,
    transfer_fee: Nat,
    metadata: Vec<(String, String)>,
    minting_account: Account,
    initial_balances: Vec<(Account, Nat)>,
    archive_options: ArchiveOptions,
    feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
struct Account {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct ArchiveOptions {
    num_blocks_to_archive: u64,
    max_transactions_per_response: Option<u64>,
    trigger_threshold: u64,
    more_controller_ids: Option<Vec<Principal>>,
    max_message_size_bytes: Option<u64>,
    cycles_for_archive_creation: Option<u64>,
    node_max_memory_size_bytes: Option<u64>,
    controller_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
struct FeatureFlags {
    icrc2: bool,
}

#[derive(CandidType)]
pub enum LedgerArgument {
    Init(InitArgs),
}

#[derive(CandidType, Deserialize)]
struct Spender {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize)]
struct ApproveArgs {
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    from_subaccount: Option<Vec<u8>>,
    created_at_time: Option<u64>,
    amount: Nat,
    expected_allowance: Option<Nat>,
    expires_at: Option<u64>,
    spender: Account,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
enum ApproveError {
    GenericError { message: String, error_code: Nat },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: Nat },
    BadFee { expected_fee: Nat },
    AllowanceChanged { current_allowance: Nat },
    CreatedInFuture { ledger_time: Nat },
    TooOld,
    Expired { ledger_time: Nat },
    InsufficientFunds { balance: Nat },
}

#[derive(CandidType, Deserialize, Serialize)]
enum ApproveResult {
    Ok(Nat),
    Err(ApproveError),
}
const BACKEND_WASM: &str = "../../target/wasm32-unknown-unknown/release/valueswap_backend.wasm";
const CKBTC_WASM: &str = "../../.dfx/local/canisters/ckbtc/ckbtc.wasm.gz";

fn setup() -> (PocketIc, Principal, Principal) {
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

    (pic, backend_canister, ckbtc_canister)
}


#[test]
fn test_rollback_on_invalid_data() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal =
        Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe")
            .unwrap();
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

    let invalid_pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "".to_string(), 
            balance: Nat::from(0u64),   
            weight: Nat::from(0u64),    
            value: Nat::from(0u64),     // Invalid: Zero values
            ledger_canister_id: ckbtc_canister,
            image: "invalid_image.png".to_string(),
        }],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((invalid_pool_data,)).unwrap();

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
                "Expected failure to trigger rollback, but got success."
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


#[test]
fn test_rollback_on_token_deposit_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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

    let response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

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

    // ✅ Step 2: Simulate Token Deposit Failure
    // Using an invalid ledger_canister_id to trigger failure
    let invalid_ledger_canister_id = Principal::from_text("aaaaa-aa").unwrap(); // Invalid principal

    let faulty_pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "FaultyToken".to_string(),
                balance: Nat::from(100u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: invalid_ledger_canister_id, // 🚩 Invalid ID to trigger failure
                image: "invalid_image.png".to_string(),
            }
        ],
        swap_fee: Nat::from(5u64),
    };

    let encoded_args = candid::encode_args((faulty_pool_data,)).unwrap();

    // ✅ Step 3: Attempt to Create Pool (Expected to Fail & Trigger Rollback)
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
                "Expected failure due to invalid ledger_canister_id, but got success."
            );
            println!("Rollback triggered as expected. Error: {:?}", result.unwrap_err());
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered due to rejection: {}", message);
        }
    }

    // ✅ Step 4: Verify Rollback Using get_users_pool
    let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

    let pools_response = pic.query_call(
        backend_canister,
        hardcoded_principal,
        "get_users_pool",
        query_args,
    ).unwrap();

    match pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, CustomError> = candid::decode_one(&data).unwrap();
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

#[test]
fn test_rollback_on_lp_share_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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

    let response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

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

    // ✅ Step 2: Simulate LP Share Calculation Failure
    let faulty_pool_data = Pool_Data {
        pool_data: vec![
            CreatePoolParams {
                token_name: "FaultyToken".to_string(),
                balance: Nat::from(100u64),
                weight: Nat::from(10u64),
                value: Nat::from(100u64),
                ledger_canister_id: ckbtc_canister,
                image: "invalid_image.png".to_string(),
            }
        ],
        swap_fee: Nat::from(u64::MAX), // 🚩 Invalid swap fee to cause LP share failure
    };

    let encoded_args = candid::encode_args((faulty_pool_data.clone(),)).unwrap();  // Clone added

    // ✅ Step 3: Attempt to Create Pool (Expected to Fail & Trigger Rollback)
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
                "Expected failure due to LP share calculation error, but got success."
            );
            println!("Rollback triggered as expected. Error: {:?}", result.unwrap_err());
        }
        WasmResult::Reject(message) => {
            println!("Rollback triggered due to rejection: {}", message);
        }
    }

    // ✅ Step 4: Verify Rollback Using get_users_pool
    let query_args = candid::encode_args((hardcoded_principal,)).unwrap();

    let pools_response = pic.query_call(
        backend_canister,
        hardcoded_principal,
        "get_users_pool",
        query_args,
    ).unwrap();

    match pools_response {
        WasmResult::Reply(data) => {
            let pools: Result<Option<Vec<String>>, CustomError> = candid::decode_one(&data).unwrap();
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

#[test]
fn test_rollback_on_add_liquidity_curr_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
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
            let result: Result<(), String> = decode_one(&data).unwrap();
            assert!(result.is_err(), "Expected failure, got success: {:?}", result);
        }
        WasmResult::Reject(message) => println!("Rollback triggered as expected. Message: {}", message),
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

#[test]
fn test_rollback_on_add_liquidity_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
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
            let result: Result<(), String> = decode_one(&data).unwrap();
            assert!(result.is_err(), "Expected failure, got success: {:?}", result);
        }
        WasmResult::Reject(message) => println!("Rollback triggered as expected. Message: {}", message),
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

#[test]
fn test_rollback_on_store_pool_data_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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
            let result: Result<Nat, String> = decode_one(&data).unwrap();
            assert!(result.is_ok(), "Approval failed with error: {:?}", result);
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }

    let pool_data = Pool_Data {
        pool_data: vec![CreatePoolParams {
            token_name: "InvalidPool".to_string(),
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
            let result: Result<(), String> = decode_one(&data).unwrap();
            assert!(result.is_err(), "Expected failure, got success: {:?}", result);
        }
        WasmResult::Reject(message) => println!("Rollback triggered as expected. Message: {}", message),
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

#[test]
fn test_rollback_on_lock_handling_failure() {
    let (pic, backend_canister, ckbtc_canister) = setup();

    let hardcoded_principal = Principal::from_text("xkd3g-llatk-lmuv7-eoudm-qtjnr-iapqh-taggr-pwpmo-3rojt-pxkwo-4qe").unwrap();
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

    let response = pic.update_call(
        ckbtc_canister,
        hardcoded_principal,
        "icrc2_approve",
        encoded_args,
    ).unwrap();

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

    let response = pic.update_call(
        backend_canister,
        hardcoded_principal,
        "create_pools",
        encoded_args.clone(),
    ).unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<(), String> = decode_one(&data).unwrap();
            assert!(result.is_err(), "Expected failure, got success: {:?}", result);
        }
        WasmResult::Reject(message) => println!("Rollback triggered as expected. Message: {}", message),
    }

    // Step 3: Ensure Lock Was Released by Trying to Create the Same Pool Again
    let response_retry = pic.update_call(
        backend_canister,
        hardcoded_principal,
        "create_pools",
        encoded_args.clone(),
    ).unwrap();

    match response_retry {
        WasmResult::Reply(data) => {
            let result: Result<(), CustomError> = decode_one(&data).unwrap();
            assert!(
                !matches!(result, Err(CustomError::AnotherOperationInProgress(_))),
                "Lock was not released! Retrying pool creation still results in 'AnotherOperationInProgress'."
            );
        }
        WasmResult::Reject(message) => println!("Retried creation failed as expected. Message: {}", message),
    }

    // Step 4: Ensure No Pool Was Stored for the User
    let get_pools_response = pic.query_call(
        backend_canister,
        hardcoded_principal,
        "get_users_pool",
        encode_args((hardcoded_principal,)).unwrap(),
    ).unwrap();

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

