use ic_cdk::export_candid;
use std::collections::{HashMap,BTreeMap};
use candid::{Principal, Nat};
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

// Export Candid interface
export_candid!();
