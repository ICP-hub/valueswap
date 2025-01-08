
use candid::{CandidType, Principal};
use ic_cdk::{call, update};
use serde::Deserialize;

#[derive(CandidType, Deserialize, Debug)]
pub enum MetadataValue {
    Nat(u128),      
    Int(i128),     
    Text(String),  
    Blob(Vec<u8>),  
}



#[update]
pub async fn get_decimals(target_canister_id: Principal) -> Result<u128, String> {
    if target_canister_id == Principal::anonymous() {
        return Err("Invalid target canister ID: Cannot be anonymous.".to_string());
    }

    ic_cdk::println!("Fetching decimals from canister {}", target_canister_id);

    // Fetch all metadata
    match call::<(), (Vec<(String, MetadataValue)>,)>(
        target_canister_id,
        "icrc1_metadata",
        (),
    )
    .await
    {
        Ok((metadata,)) => {
            // Filter for "icrc1:decimals" and extract its value
            for (key, value) in metadata {
                if key == "icrc1:decimals" {
                    if let MetadataValue::Nat(decimals) = value {
                        ic_cdk::println!("Decimals found: {}", decimals);
                        return Ok(decimals);
                    } else {
                        let error_message = "Invalid type for decimals, expected Nat.".to_string();
                        ic_cdk::println!("{}", error_message);
                        return Err(error_message);
                    }
                }
            }
            let error_message = "Decimals not found in metadata.".to_string();
            ic_cdk::println!("{}", error_message);
            Err(error_message)
        }
        Err((rejection_code, message)) => {
            let error_message = format!(
                "Failed to fetch metadata: {:?} - {}",
                rejection_code, message
            );
            ic_cdk::println!("{}", error_message);
            Err(error_message)
        }
    }
}

// #[update]
// pub async fn get_metadata(target_canister_id: Principal) -> Result<Vec<(String, MetadataValue)>, String> {
//     if target_canister_id == Principal::anonymous() {
//         return Err("Invalid target canister ID: Cannot be anonymous.".to_string());
//     }

//     ic_cdk::println!("Fetching metadata from canister {}", target_canister_id);

//     match call::<(), (Vec<(String, MetadataValue)>,)>(
//         target_canister_id,
//         "icrc1_metadata",
//         (),
//     )
//     .await
//     {
//         Ok((metadata,)) => {
//             ic_cdk::println!("Metadata retrieved successfully: {:?}", metadata);
//             Ok(metadata)
//         }
//         Err((rejection_code, message)) => {
//             let error_message = format!(
//                 "Failed to fetch metadata: {:?} - {}",
//                 rejection_code, message
//             );
//             ic_cdk::println!("{}", error_message);
//             Err(error_message)
//         }
//     }
// }


// fn parse_metadata(metadata: TokenMetadata) -> Result<(String, String, Option<u8>), String> {
//     let mut name = None;
//     let mut symbol = None;
//     let mut decimals = None;

//     for entry in metadata.0 {
//         match entry.key.as_str() {
//             "icrc1:name" => {
//                 if let MetadataValue::Text(value) = entry.value {
//                     name = Some(value);
//                 }
//             },
//             "icrc1:symbol" => {
//                 if let MetadataValue::Text(value) = entry.value {
//                     symbol = Some(value);
//                 }
//             },
//             "icrc1:decimals" => {
//                 if let MetadataValue::Nat(value) = entry.value {
//                     decimals = Some(value.0.try_into().ok()?);
//                 }
//             },
//             _ => {}
//         }
//     }

//     Ok((
//         name.ok_or("Name not found")?,
//         symbol.ok_or("Symbol not found")?,
//         decimals,
//     ))
// }