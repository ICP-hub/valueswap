// use serde::{Deserialize, Serialize};
use ic_cdk_macros::*;


#[init]
fn init() {
    ic_cdk::println!("Swap canister initialized.");
}
