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

// // Utility functions for Nat and Int conversions
// pub mod utils {
//     use candid::Nat;

//     pub fn int_to_nat(val: i64, bits: u32) -> Nat {
//         Nat::from(val as u64)
//     }

//     // pub fn nat_to_int(val: Nat) -> i64 {
//     //     val.0.to_u64().unwrap_or(0) as i64
//     // }
// }

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CreatePoolParams{
    pub token_name : String,
    pub balance : u64,
    pub weight : f64,
    pub value : u64,
    pub ledger_canister_id: Principal, // Ledger canister ID for the token (e.g., ckBTC, ckETH)
    pub image : String
}

#[derive(CandidType, Deserialize ,Serialize, Clone)]

pub struct SwapParams {
    pub token1_name : String,
    pub token_amount : f64,
    pub token2_name : String,
    pub ledger_canister_id : Principal,
    pub  ledger_canister_id2 : Principal
}

#[derive(CandidType, Deserialize, Clone)]
pub struct Pool_Data {
    pub pool_data: Vec<CreatePoolParams>,
    pub swap_fee: f64,
}

#[derive(CandidType , Deserialize , Clone)]
pub struct swap_result{
    pub amount1 : f64,
    pub amount2 : f64
}