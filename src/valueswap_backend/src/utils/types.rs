
use serde::{Deserialize, Serialize};
use candid::CandidType;
use candid::{Nat, Principal};
use ic_cdk::{
    api::{
        // call::{call_with_payment128, CallResult},
        // canister_version,
        management_canister::main::{CanisterInstallMode, WasmModule},
    },
    // call, api,
};
// use std::fmt::Display;

/// Represents the pool's share with token balances and weights.
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct PoolShare {
    pub token_names: Vec<String>,    // Names of the tokens
    pub token_balances: Vec<f64>,    // Balances of the tokens
    pub token_weights: Vec<f64>,     // Weights of the tokens
    pub token_value: Vec<f64>,
}

impl PoolShare {
    /// Creates a new PoolShare instance with given names, balances, and weights.
    pub fn new(names: Vec<String>, balances: Vec<f64>, weights: Vec<f64>, values: Vec<f64>) -> Self {
        PoolShare {
            token_names: names,
            token_balances: balances,
            token_weights: weights,
            token_value: values,
            
        }
    }
}

// Define the input struct for create_pool
// #[derive(CandidType, Deserialize, Serialize, Clone)]
// pub struct CreatePoolParams {
//     pub token_names: Vec<String>,   // Names of the tokens
//     pub balances: Vec<u64>,         // Token balances
//     pub weights: Vec<f64>,          // Token weights
//     pub values: Vec<u64>,
//     pub swap_fees: f64,
// }

#[derive(CandidType, Deserialize, Serialize, Clone, Debug)]
pub struct CreatePoolParams{
    pub token_name : String,
    pub balance : Nat,
    pub weight : Nat,
    pub value : Nat,
    pub ledger_canister_id: Principal, // Ledger canister ID for the token (e.g., ckBTC, ckETH)
    pub image : String
}

#[derive(CandidType, Deserialize, Serialize, Clone , Debug)]
pub struct Pool_Data{
    pub pool_data : Vec<CreatePoolParams>,
    pub swap_fee : Nat
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



/// Represents the user's share with their token balances.
#[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
pub struct UserShare {
    pub token_balances: Vec<f64>, // User's balances of the tokens
}

impl UserShare {
    /// Creates a new UserShare instance with zero balances for the given number of tokens.
    pub fn new(num_tokens: usize) -> Self {
        UserShare {
            token_balances: vec![0.0; num_tokens],
        }
    }
}

// Implementing the Default trait for UserShare to provide a default initialization.
impl Default for UserShare {
    fn default() -> Self {
        UserShare::new(8) // Default to 8 tokens
    }
}

/// Enum representing different types of tokens.
// #[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
// pub enum TokenType {
//     TokenA,
//     TokenB,
//     // Add more token types as needed
// }

// impl TokenType {
//     /// Returns the name of the token type as a string.
//     pub fn name(&self) -> &str {
//         match self {
//             TokenType::TokenA => "TokenA",
//             TokenType::TokenB => "TokenB",
//             // Add more token names as needed
//         }
//     }
// }

// pub type TokenType = String;
// pub type Amount = u64;

// #[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
// pub struct Pools{
//     children: BTreeMap<String, HashMap<TokenType, Amount>>,
// }

// impl Pools{
//     pub fn new() -> Self{
//         Pools {
//             children: BTreeMap::new(),
//         }
//     }

//     pub fn add_child(&mut self , id: String){
//         self.children.insert(id , HashMap::new());
//     }

//     pub fn add_token(&mut self, child_id : &str , token : TokenType , amount : Amount){
//         if let Some(child) = self.children.get_mut(child_id){
//             child.insert(token, amount);
//         }else{
//             println!("Child ID {} not found!", child_id);
//         }
//     } 
// }

// Define the transfer_from arguments and result types
#[derive(CandidType, Deserialize,Debug)]
pub struct TransferFromArgs {
    pub to: TransferAccount,
    pub fee: Option<u64>,
    pub spender_subaccount: Option<Vec<u8>>,
    pub from: TransferAccount,
    pub memo: Option<Vec<u8>>,
    pub created_at_time: Option<u64>,
    pub amount: Nat,
}

#[derive(CandidType, Deserialize,Debug)]
pub struct TransferAccount {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

#[derive(CandidType, Deserialize,Debug)]
pub enum TransferFromResult {
    Ok(Nat),
    Err(TransferFromError),
}

#[derive(CandidType, Deserialize, Debug)]
pub enum TransferFromError {
    GenericError { message: String, error_code: Nat },
    TemporarilyUnavailable,
    InsufficientAllowance { allowance: Nat },
    BadBurn { min_burn_amount: Nat },
    Duplicate { duplicate_of: Nat },
    BadFee { expected_fee: Nat },
    CreatedInFuture { ledger_time: u64 },
    TooOld,
    InsufficientFunds { balance: Nat },
}


#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CanisterSettings {
    pub controllers: Option<Vec<Principal>>,

    pub compute_allocation: Option<Nat>,

    pub memory_allocation: Option<Nat>,

    pub freezing_threshold: Option<Nat>,

    pub reserved_cycles_limit: Option<Nat>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub(crate) struct InstallCodeArgumentExtended {
    pub mode: CanisterInstallMode,
    pub canister_id: CanisterId,
    pub wasm_module: WasmModule,
    pub arg: Vec<u8>,
    pub sender_canister_version: Option<u64>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub struct CreateCanisterArgument {
    pub settings: Option<CanisterSettings>,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone,
)]
pub struct InstallCodeArgument {
    pub mode: CanisterInstallMode,
    pub canister_id: CanisterId,
    pub wasm_module: WasmModule,
    pub arg: Vec<u8>,
}

pub type CanisterId = Principal;

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy,
)]
pub struct CanisterIdRecord {
    pub canister_id: CanisterId,
}

#[derive(
    CandidType, Serialize, Deserialize, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Default,
)]
pub(crate) struct CreateCanisterArgumentExtended {
    pub settings: Option<CanisterSettings>,
    pub sender_canister_version: Option<u64>,
}

#[derive(Debug,CandidType)]
pub enum CreateCanisterError {
    CreateError(String),
    DepositError(String),
    InstallError(String),
}

impl std::fmt::Display for CreateCanisterError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Self::CreateError(e) => write!(f, "Canister creation failed: {}", e),
            Self::DepositError(e) => write!(f, "Deposit cycles failed: {}", e),
            Self::InstallError(e) => write!(f, "Install code failed: {}", e),
        }
    }
}

#[derive(Debug,CandidType)]
pub enum InstallError {
    InvalidArgument(String),
    Rejection(ic_cdk::api::call::RejectionCode, String),
    Unexpected(String),
}


// #[derive(Debug, Clone, CandidType, Deserialize, Serialize)]
// pub struct TokenData{
//     pub pool_key: String,
//     pub user_id : Principal,
//     pub amount : BTreeMap<String , u64>
// }



#[derive(CandidType, Deserialize ,Serialize, Clone)]

pub struct SwapParams {
    pub token1_name : String,
    pub token_amount : Nat,
    pub token2_name : String,
    pub ledger_canister_id1 : Principal,
    pub ledger_canister_id2 : Principal
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
