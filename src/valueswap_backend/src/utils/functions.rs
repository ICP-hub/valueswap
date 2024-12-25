// use ic_cdk::api::call::{CallResult, RejectionCode};


// // execute methods of other canisters
// pub async fn call_inter_canister<T, U>(
//   function: &str,
//   args: T,
//   canister_id: candid::Principal,
// ) -> Result<U, String>
// where
//   T: candid::CandidType + serde::Serialize,
//   U: candid::CandidType + for<'de> serde::Deserialize<'de>,
// {
//   let response: CallResult<(U,)> = ic_cdk::call(canister_id, function, (args,)).await;

//   let res0: Result<(U,), (RejectionCode, String)> = response;

//   match res0 {
//       Ok(val) => Ok(val.0),
//       Err((code, message)) => match code {
//           RejectionCode::NoError => Err("NoError".to_string()),
//           RejectionCode::SysFatal => Err("SysFatal".to_string()),
//           RejectionCode::SysTransient => Err("SysTransient".to_string()),
//           RejectionCode::DestinationInvalid => Err("DestinationInvalid".to_string()),
//           RejectionCode::CanisterReject => Err("CanisterReject".to_string()),
//           _ => Err(format!("Unknown rejection code: {:?}: {}", code, message)),
//       },
//   }
// }
