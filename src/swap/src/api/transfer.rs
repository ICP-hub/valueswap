use candid::{CandidType, Deserialize, Nat, Principal};
use ic_cdk::api::call::call;
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
    mut amount: Nat,
) -> Result<Nat, String> {
    // Validate input parameters
    if amount == Nat::from(0u32) {
        return Err("Transfer amount must be greater than zero.".to_string());
    }

    if user_principal == Principal::anonymous() {
        return Err("Invalid user principal: Cannot be anonymous.".to_string());
    }

    if ledger_canister == Principal::anonymous() {
        return Err("Invalid ledger canister ID: Cannot be anonymous.".to_string());
    }

    let platform_principal = ic_cdk::id();
    let caller = ic_cdk::caller();

    ic_cdk::println!("Caller: {}", caller.to_text());
    ic_cdk::println!("Ledger Canister: {}", ledger_canister.to_text());
    ic_cdk::println!("User Principal: {}", user_principal.to_text());

    // Fetch the balance of the pool before the transfer
    #[derive(CandidType, Deserialize, Debug)]
    struct TransferAccount {
        pub owner: Principal,
        pub subaccount: Option<Vec<u8>>,
    }
    let balance = match call::<(TransferAccount,), (Nat,)>(
        ledger_canister,
        "icrc1_balance_of",
        (TransferAccount {
            owner: platform_principal,
            subaccount: None,
        },),
    )
    .await
    {
        Ok((bal,)) => bal,
        Err(err) => return Err(format!("Failed to fetch balance: {:?}", err)),
    };

    ic_cdk::println!("Current Pool Balance: {}", balance);

    // Ensure the pool has enough balance to make the transfer
    if balance < amount {
        return Err(format!(
            "Insufficient Funds: Available {} but required {}",
            balance, amount
        ));
    }

    let (fee,): (Nat,) = call(ledger_canister, "icrc1_fee", ())
        .await
        .map_err(|e| format!("Failed to fetch fee: {:?}", e))?;

    ic_cdk::println!("✅ Fetched transaction fee: {}", fee);

    if amount < fee {
        return Err(format!(
            "Transfer amount {} is less than the transaction fee {}",
            amount, fee
        ));
    }
    // Deduct the transaction fee
    amount -= fee;
    ic_cdk::println!("updated amount {}", amount);

    let args = TransferArg {
        from_subaccount: None,
        to: Account {
            owner: user_principal,
            subaccount: None,
        },
        amount,
        fee: None,
        memo: None,
        created_at_time: None,
    };

    ic_cdk::println!("argument =  {:?}", args);

    // Execute the transfer
    let (result,): (TransferResult,) = call(ledger_canister, "icrc1_transfer", (args,))
        .await
        .map_err(|e| format!("Transfer failed: {:?}", e))?;

    match result {
        TransferResult::Ok(balance) => {
            ic_cdk::println!("Transfer successful. New balance: {}", balance);
            Ok(balance)
        }
        // TransferResult::Err(err) => {
        //     ic_cdk::println!("Transfer failed: {:?}", err);
        //     Err(format!("Transfer failed: {:?}", err))
        // }
        TransferResult::Err(error) => {
            match error {
                TransferError::InsufficientFunds { ref balance } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Insufficient funds. Available balance: {:?}",
                        balance
                    );
                }
                TransferError::BadFee { ref expected_fee } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Incorrect fee. Expected: {:?}",
                        expected_fee,
                    );
                }
                TransferError::BadBurn {
                    ref min_burn_amount,
                } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Burn amount too low. Minimum required: {:?}",
                        min_burn_amount
                    );
                }
                TransferError::Duplicate { ref duplicate_of } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Duplicate transaction. Duplicate of BlockIndex: {:?}",
                        duplicate_of
                    );
                }
                TransferError::TemporarilyUnavailable => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Ledger temporarily unavailable. Try again later."
                    );
                }
                TransferError::CreatedInFuture { ref ledger_time } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Transaction timestamp is too far in the future. Ledger time: {:?}",
                        ledger_time
                    );
                }
                TransferError::TooOld => {
                    ic_cdk::println!("❌ Transfer failed: Transaction too old.");
                }
                TransferError::GenericError {
                    ref error_code,
                    ref message,
                } => {
                    ic_cdk::println!(
                        "❌ Transfer failed: Generic error. Code: {:?}, Message: {:?}",
                        error_code,
                        message
                    );
                }
            }
            Err(format!("Transfer failed: {:?}", error))
        }
    }
}
