pub mod transfer;

use candid::Principal;
use ic_cdk::api::caller;

pub fn get_caller() -> Principal {
    caller()
}