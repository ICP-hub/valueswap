use ic_cdk::export_candid;
use std::collections::{HashMap,BTreeMap};
use candid::{Principal, Nat};
use candid::{self, CandidType, Deserialize};
use ic_cdk_macros::{query, update};
use std::cell::RefCell;
// use ic_xrc_types::GetExchangeRateResult;
use crate::api::transfer::BlockIndex;
mod utils;
pub mod vault;
mod api;
mod constants;
mod memory;
mod types;

pub use vault::pool_factory::*;

pub use utils::types::*;
// pub use utils::types::{PoolShare, UserShare,CreatePoolParams,TokenData};
pub use types::state_handlers::*;

thread_local! {
    // The memory manager is used for simulating multiple memories. Given a `MemoryId` it can
    // return a memory that can be used by stable structures.
    static STATE : RefCell<State> = RefCell::new(State::new());
}

pub fn with_state<R>(f: impl FnOnce(&mut State) -> R) -> R {
    STATE.with(|cell| f(&mut cell.borrow_mut()))
}

#[derive(CandidType, Deserialize, Eq, PartialEq, Debug)]
pub struct SupportedStandard {
    pub url: String,
    pub name: String,
}
 
#[query]
fn icrc10_supported_standards() -> Vec<SupportedStandard> {
    vec![
        SupportedStandard {
            url: "https://github.com/dfinity/ICRC/blob/main/ICRCs/ICRC-10/ICRC-10.md".to_string(),
            name: "ICRC-10".to_string(),
        },
        SupportedStandard {
            url: "https://github.com/dfinity/wg-identity-authentication/blob/main/topics/icrc_28_trusted_origins.md".to_string(),
            name: "ICRC-28".to_string(),
        },
    ]
}
 
#[derive(Clone, Debug, CandidType, Deserialize)]
pub struct Icrc28TrustedOriginsResponse {
    pub trusted_origins: Vec<String>
}
 
// list every base URL that users will authenticate to your app from
#[update]
fn icrc28_trusted_origins() -> Icrc28TrustedOriginsResponse {
    let trusted_origins = vec![
        String::from("https://ajzka-lyaaa-aaaak-ak5rq-cai.icp0.io"),
        String::from("http://localhost:3000"),
        String::from("http://by6od-j4aaa-aaaaa-qaadq-cai.localhost:4943"),
        String::from("http://127.0.0.1:4943/?canisterId=bd3sg-teaaa-aaaaa-qaaba-cai"),
        String::from("http://127.0.0.1:4943"),
        String::from("http://localhost:4200"),
    ];
 
    return Icrc28TrustedOriginsResponse { trusted_origins }
}

// Export Candid interface
export_candid!();
