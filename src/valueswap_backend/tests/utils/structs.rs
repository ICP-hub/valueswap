use candid::{CandidType, Nat, Principal};
use serde::{Deserialize, Serialize};

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
    pub(crate) token_symbol: String,
    pub(crate) token_name: String,
    pub(crate) transfer_fee: Nat,
    pub(crate)metadata: Vec<(String, String)>,
    pub(crate) minting_account: Account,
    pub(crate) initial_balances: Vec<(Account, Nat)>,
    pub(crate) archive_options: ArchiveOptions,
    pub(crate) feature_flags: Option<FeatureFlags>,
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct Account {
    pub(crate) owner: Principal,
    pub(crate) subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub(crate) struct ArchiveOptions {
    pub(crate) num_blocks_to_archive: u64,
    pub(crate) max_transactions_per_response: Option<u64>,
    pub(crate) trigger_threshold: u64,
    pub(crate) more_controller_ids: Option<Vec<Principal>>,
    pub(crate) max_message_size_bytes: Option<u64>,
    pub(crate) cycles_for_archive_creation: Option<u64>,
    pub(crate) node_max_memory_size_bytes: Option<u64>,
    pub(crate) controller_id: Principal,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct FeatureFlags {
    pub(crate) icrc2: bool,
}

#[derive(CandidType)]
pub enum LedgerArgument {
    Init(InitArgs),
}

#[derive(CandidType, Deserialize)]
pub struct Spender {
    owner: Principal,
    subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize, Serialize)]
pub struct ApproveArgs {
    pub(crate) fee: Option<Nat>,
    pub(crate) memo: Option<Vec<u8>>,
    pub(crate) from_subaccount: Option<Vec<u8>>,
    pub(crate) created_at_time: Option<u64>,
    pub(crate) amount: Nat,
    pub(crate) expected_allowance: Option<Nat>,
    pub(crate) expires_at: Option<u64>,
    pub(crate) spender: Account,
}

#[derive(CandidType, Deserialize, Serialize, Debug)]
pub enum ApproveError {
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
pub enum ApproveResult {
    Ok(Nat),
    Err(ApproveError),
}