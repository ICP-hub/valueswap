use candid::{CandidType, Deserialize};
use serde::{Deserialize as SerdeDeserialize, Serialize};
use std::fmt;
use candid::{Nat, Principal};

#[derive(CandidType, SerdeDeserialize, Serialize, Clone, Debug)]
pub enum SwapError {
    InternalError(String), // Generic internal error with a message
    InSufficientBalance,
    InvalidAmount,
    UserNotFound,
    PoolNotFound,
    UnauthorizedAccess,
    OverflowError,
    UnderFlowError
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            SwapError::InSufficientBalance=> write!(f, "Insufficiant Balance"),
            SwapError::InvalidAmount => write!(f , "Invalid Amount"),
            SwapError::UserNotFound => write!(f , "UserNotFount"),
            SwapError::PoolNotFound => write!(f , "PoolNotFound"),
            SwapError::UnauthorizedAccess => write!(f , "UnauthorizedAccess"),
            SwapError::OverflowError => write!(f , "OverflowError"),
            SwapError::UnderFlowError => write!(f , "UnderFlowError")
        }
    }
}

impl std::error::Error for SwapError {}

impl From<String> for SwapError {
    fn from(err: String) -> Self {
        SwapError::InternalError(err)

    }
}

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CreatePoolParams{
    pub token_name : String,
    pub balance : Nat,
    pub weight : Nat,
    pub value : Nat,
    pub ledger_canister_id: Principal, // Ledger canister ID for the token (e.g., ckBTC, ckETH)
    pub image : String
}

#[derive(CandidType, Deserialize ,Serialize, Clone)]

pub struct SwapParams {
    pub token1_name : String,
    pub token_amount : Nat,
    pub token2_name : String,
    pub ledger_canister_id1 : Principal,
    pub  ledger_canister_id2 : Principal
}

#[derive(CandidType, Deserialize, Clone)]
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

           
        }


        Ok(())
    }

 

    
    
}

#[derive(Debug, PartialEq , CandidType)]
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

#[derive(CandidType , Deserialize , Clone)]
pub struct SwapResult{
    pub amount1 : f64,
    pub amount2 : f64
}