use candid::{Decode, CandidType , Principal};
use ic_agent::Agent;

#[derive(CandidType, serde::Deserialize, Debug)]
pub struct MetadataEntry {
    pub key: String,
    pub value: MetadataValue,
}

#[derive(CandidType, serde::Deserialize, Debug)]
pub enum MetadataValue {
    Nat(candid::Nat),
    Text(String),
}

#[derive(CandidType, serde::Deserialize, Debug)]
pub struct TokenMetadata(Vec<MetadataEntry>);


pub async fn icrc_get_metadata(canister_id: Principal) -> Result<TokenMetadata, String> {
    let agent = Agent::builder()
        .build()
        .map_err(|e| format!("Failed to build agent: {}", e))?;

    let response = agent
        .query(&canister_id, "icrc1_metadata")
        .call()
        .await
        .map_err(|e| format!("Query failed: {}", e))?;

    Decode!(&response, TokenMetadata)
        .map_err(|e| format!("Failed to decode metadata: {}", e))
}

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
//                     decimals = Some(value.0.try_into().ok());
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