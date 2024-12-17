// use ic_cdk::api::call;
// use candid::Principal;
// use icrc1::ledger::{Account, QueryError, QueryResult};

// // Define the function to query the balance of an account
// async fn icrc1_balance_of(token_canister_id: Principal, account: Account) -> Result<u64, QueryError> {
//     // Construct the method name and arguments
//     let method = "balance_of";
//     let args = (account,);

//     // Call the ICRC-1 token canister's balance_of function
//     let result: QueryResult<u64> = call::query(
//         token_canister_id,
//         method,
//         args,
//     )
//     .await
//     .map_err(|err| QueryError::CallError(err.to_string()))?;

//     // Return the result
//     result.map_err(|err| QueryError::QueryError(err.to_string()))
// }


// // let token_canister_id = Principal::from_text("your-token-canister-id").unwrap();
// // let account = Account {
// //     owner: Principal::from_text("account-principal-id").unwrap(),
// //     subaccount: None,  // You can provide an optional subaccount if needed
// // };

// // let balance = icrc1_balance_of(token_canister_id, account).await;
// // match balance {
// //     Ok(amount) => println!("Balance: {}", amount),
// //     Err(e) => eprintln!("Error: {}", e),
// // }

use ic_cdk::api::call::CallResult;
use candid::Principal;
use ic_cdk_macros::*;
use candid::{CandidType, Deserialize};
use serde::Serialize;
use ic_cdk::api::call::RejectionCode;
use candid::Nat;

#[derive(CandidType,Serialize, Deserialize, Debug)]
pub struct Account {
    pub owner: Principal,
    pub subaccount: Option<[u8; 32]>,
}


#[update]
pub async fn icrc_get_balance(ledger_canister_id : Principal, id: Principal) -> Result<Nat, String> {
    call_inter_canister::<Account, Nat>(
        "icrc1_balance_of",
        Account {
            owner: id,
            subaccount: None,
        },
        ledger_canister_id,
    ).await
}


// execute methods of other canisters
pub async fn call_inter_canister<T, U>(
    function: &str,
    args: T,
    canister_id: Principal,
) -> Result<U, String>
where
    T: CandidType + Serialize,
    U: CandidType + for<'de> serde::Deserialize<'de>,
{
    let response: CallResult<(U,)> = ic_cdk::call(canister_id, function, (args,)).await;

    let res0: Result<(U,), (RejectionCode, String)> = response;

    match res0 {
        Ok(val) => Ok(val.0),
        Err((code, message)) => match code {
            RejectionCode::NoError => Err("NoError".to_string()),
            RejectionCode::SysFatal => Err("SysFatal".to_string()),
            RejectionCode::SysTransient => Err("SysTransient".to_string()),
            RejectionCode::DestinationInvalid => Err("DestinationInvalid".to_string()),
            RejectionCode::CanisterReject => Err("CanisterReject".to_string()),
            _ => Err(format!("Unknown rejection code: {:?}: {}", code, message)),
        },
    }
}