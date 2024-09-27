use candid::{CandidType, Deserialize};
use candid::Principal;
use serde::{Deserialize as SerdeDeserialize, Serialize};
use std::fmt;

#[derive(CandidType, SerdeDeserialize, Serialize, Clone, Debug)]
pub enum SwapError {
    InternalError(String),
    InSufficientBalance,
    InvalidAmount,
    UserNotFound,
    PoolNotFound,
    UnauthorizedAccess,
    OverflowError,
    UnderFlowError,
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            SwapError::InSufficientBalance => write!(f, "Insufficient Balance"),
            SwapError::InvalidAmount => write!(f, "Invalid Amount"),
            SwapError::UserNotFound => write!(f, "User Not Found"),
            SwapError::PoolNotFound => write!(f, "Pool Not Found"),
            SwapError::UnauthorizedAccess => write!(f, "Unauthorized Access"),
            SwapError::OverflowError => write!(f, "Overflow Error"),
            SwapError::UnderFlowError => write!(f, "Underflow Error"),
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
    pub balance : u64,
    pub weight : f64,
    pub value : u64,
    pub ledger_canister_id: Principal, // Ledger canister ID for the token (e.g., ckBTC, ckETH)
    pub image : String
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SwapParams {
    pub token1_name : String,
    pub token_amount : u64,
    pub token2_name : String,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct PoolData {
    pub pool_data: Vec<CreatePoolParams>,
    pub swap_fee: f64,
}

#[derive(CandidType, Deserialize, Clone)]
pub struct SwapResult {
    pub amount1: f64,
    pub amount2: f64,
}