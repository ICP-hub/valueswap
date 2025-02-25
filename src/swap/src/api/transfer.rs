
use ic_cdk::api::call::call;
use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk_macros::update;

#[derive(CandidType, Deserialize, Debug)]
struct Account {
    pub owner: Principal,
    pub subaccount: Option<Vec<u8>>,
}

// #[derive(CandidType, Deserialize,Debug)]
// struct TransferArg {
//     from_subaccount: Option<Vec<u8>>,
//     to: Account,
//     amount: Nat,
//     fee: Option<u64>,
//     memo: Option<Vec<u8>>,
//     created_at_time: Option<u64>,
// }
pub type BlockIndex = Nat;

#[derive(CandidType, Deserialize, Debug)]
struct TransferArg {
    from_subaccount: Option<Vec<u8>>,
    to: Account,
    amount: Nat,
    fee: Option<Nat>,
    memo: Option<Vec<u8>>,
    created_at_time: Option<u64>,
}

#[derive(CandidType, Deserialize, Debug)]
enum TransferError {
    BadFee { expected_fee: Nat },
    BadBurn { min_burn_amount: Nat },
    InsufficientFunds { balance: Nat },
    TooOld,
    CreatedInFuture { ledger_time: u64 },
    TemporarilyUnavailable,
    Duplicate { duplicate_of: Nat },
    GenericError { error_code: Nat, message: String },
}

#[derive(CandidType, Deserialize)]
enum TransferResult {
    Ok(BlockIndex),
    Err(TransferError),
}

// #[derive(CandidType, Deserialize)]
// enum TransferResult {
//     Ok(Nat),
//     Err(String),
// }


// #[update]
// pub async fn icrc1_transfer(
//     canister_id: Principal,
//     user_principal: Principal,
//     amount: Nat,
// ) -> Result<Nat, String> {
//     // Validate input amount
//     if amount == Nat::from(0u32) {
//         return Err("Transfer amount must be greater than zero.".to_string());
//     }

//     // Validate user principal
//     if user_principal == Principal::anonymous() {
//         return Err("Invalid user principal: Cannot be anonymous.".to_string());
//     }

//     // Validate canister ID
//     if canister_id == Principal::anonymous() {
//         return Err("Invalid canister ID: Cannot be anonymous.".to_string());
//     }

//     // Debug: Log input arguments
//     ic_cdk::println!(
//         "Debug: Initiating transfer to {} with amount {} via canister {}",
//         user_principal, amount, canister_id
//     );

//     // Define the parameters for the ICRC2 transfer call
//     let args = TransferArg {
//         from_subaccount: None, // Optionally specify a subaccount if needed
//         to: Account {
//             owner: user_principal, // The recipient of the transfer
//             subaccount: None,
//         },
//         amount: amount.clone(), // The amount of tokens to transfer
//         fee: None, // Specify a fee if required
//         memo: None, // Optional memo for the transfer
//         created_at_time: None, // Optional timestamp
//     };

//     // Debug: Log constructed transfer arguments
//     ic_cdk::println!("Debug: TransferArg constructed: {:?}", args);

//     // Make the call to the ICRC2 token canister with the transfer arguments
//     let (result,): (TransferResult,) = call(
//         canister_id, // The canister ID of the token ledger (ICRC2)
//         "icrc1_transfer", // The method to call
//         (args,), // Transfer arguments
//     )
//     .await
//     .map_err(|e| format!("Transfer failed: {:?}", e))?;

//     // Check if the call was successful
//     match result {
//         TransferResult::Ok(balance) => {
//             // Debug: Log successful transfer
//             ic_cdk::println!(
//                 "Debug: Transfer successful. New balance: {}",
//                 balance
//             );
//             Ok(balance)
//         }
//         TransferResult::Err(err) => {
//             // Debug: Log transfer error
//             ic_cdk::println!("Error: Transfer failed: {:?}", err);
//             Err(format!("Transfer failed: {:?}", err))
//         }
//     }
// }

#[update]
pub async fn icrc1_transfer(
    ledger_canister: Principal,
    user_principal: Principal,
    amount: Nat,
) -> Result<Nat, String> {
    // Validate input amount
    if amount == Nat::from(0u32) {
        return Err("Transfer amount must be greater than zero.".to_string());
    }

    // Validate user principal
    if user_principal == Principal::anonymous() {
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    // Validate ledger canister principal
    if ledger_canister == Principal::anonymous() {
        return Err("Invalid ledger canister ID: Cannot be anonymous.".to_string());
    }

    // Debug: Log input arguments
    ic_cdk::println!(
        "Debug: Initiating faucet transfer to {} with amount {} via ledger {}",
        user_principal, amount, ledger_canister
    );

    // Define the parameters for the ICRC2 transfer call
    let args = TransferArg {
        from_subaccount: None, // Optionally specify a subaccount if needed
        to: Account {
            owner: user_principal, // The recipient of the transfer
            subaccount: None,
        },
        amount, // The amount of tokens to transfer
        fee: None, // Specify a fee if required
        memo: None, // Optional memo for the transfer
        created_at_time: None, // Optional timestamp
    };

    // Debug: Log constructed transfer arguments
    ic_cdk::println!("Debug: TransferArg constructed: {:?}", args);

    // Make the call to the ICRC2 token canister with the transfer arguments
    let (result,): (TransferResult,) = call(ledger_canister, "icrc1_transfer", (args,))
        .await
        .map_err(|e| format!("Transfer failed: {:?}", e))?;

    // Check if the call was successful
    match result {
        TransferResult::Ok(balance) => {
            // Debug: Log successful transfer
            ic_cdk::println!(
                "Debug: Faucet transfer successful. New balance: {}",
                balance
            );
            Ok(balance)
        }
        TransferResult::Err(err) => {
            // Debug: Log transfer error
            ic_cdk::println!("Error: Faucet transfer failed: {:?}", err);
            Err(format!("Faucet transfer failed: {:?}", err))
        }
    }
}




