use candid::{CandidType, Deserialize, Nat, Principal};
use serde::{Deserialize as SerdeDeserialize, Serialize};
use std::fmt;

// #[derive(CandidType, SerdeDeserialize, Serialize, Clone, Debug)]
// pub struct SwapResult {
//     pub amount0: i64, // Resulting amount of token 0
//     pub amount1: i64, // Resulting amount of token 1
//     // Add other necessary fields for the swap result
// }

#[derive(CandidType, SerdeDeserialize, Serialize, Clone, Debug)]
pub enum SwapError {
    InternalError(String), // Generic internal error with a message
    // Add other error types as needed
}

impl fmt::Display for SwapError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SwapError::InternalError(msg) => write!(f, "Internal error: {}", msg),
            // Add other error formats as needed
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

#[derive(CandidType, Deserialize, Clone)]
pub struct CreatePoolParams {
    pub token_name: String,
    pub balance: u64,
    pub weight: f64,
    pub value: u64,
}

#[derive(CandidType, Deserialize ,Clone)]
pub struct SwapParams {
    pub token1 : f64,
    pub token2 : f64,
    pub zero_for_one : bool
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