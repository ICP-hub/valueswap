use std::{borrow::Cow,};

use candid::{CandidType, Decode, Encode, Principal};
use ic_stable_structures::{StableBTreeMap,};
use ic_stable_structures::{storable::Bound, Storable};

use serde::{Deserialize, Serialize};


use crate::{memory::*, with_state,};

use crate::utils::types::*;

#[derive(Debug, Serialize, Deserialize, CandidType, Clone)]
pub struct user_principal{
    pub principal : Principal
}


pub struct State{
    pub TOKEN_POOLS : StableBTreeMap<String , user_principal , Memory>,
    pub USERS_TOKENS : StableBTreeMap<Principal ,Pool_Data , Memory >,
}


impl State {
    pub fn new() -> Self {
        Self {
            TOKEN_POOLS: init_pools(),
            USERS_TOKENS: init_user_tokens(),
        }
    }
}

fn init_pools() -> StableBTreeMap<String , user_principal , Memory> {
    StableBTreeMap::init(get_pool_data())
}

fn init_user_tokens() -> StableBTreeMap<Principal , Pool_Data , Memory> {
    StableBTreeMap::init(get_user_token_memory())
}

impl Storable for user_principal {
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
