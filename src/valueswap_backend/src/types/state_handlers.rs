use std::borrow::Cow;

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::StableBTreeMap;
use ic_stable_structures::{storable::Bound, Storable};

use serde::{Deserialize, Serialize};


use crate::memory::*;

use crate::utils::types::*;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct UserPrincipal{
    pub principal : Principal
}


pub struct State{
    pub token_pools : StableBTreeMap<String , UserPrincipal , Memory>,
    pub users_tokens : StableBTreeMap<Principal ,Pool_Data , Memory >,
}


impl State {
    pub fn new() -> Self {
        Self {
            token_pools: init_pools(),
            users_tokens: init_user_tokens(),
        }
    }
}

fn init_pools() -> StableBTreeMap<String , UserPrincipal , Memory> {
    StableBTreeMap::init(get_pool_data())
}

fn init_user_tokens() -> StableBTreeMap<Principal , Pool_Data , Memory> {
    StableBTreeMap::init(get_user_token_memory())
}

impl Storable for UserPrincipal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
}
    const BOUND: Bound = Bound::Unbounded;
}


impl Storable for Pool_Data {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

fn from_bytes(bytes: Cow<[u8]>) -> Self {
    Decode!(bytes.as_ref(), Self).unwrap()
}
    const BOUND: Bound = Bound::Unbounded;
}
