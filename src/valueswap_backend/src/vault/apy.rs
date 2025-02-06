use std::cell::RefCell;
use candid::{Nat, Principal};
use ic_cdk::update;
use std::collections::BTreeMap;

use crate::vault::lp_tokens::*;
use crate::api::balance::*;

thread_local! {
    pub static FEE_SHARE: RefCell<BTreeMap<Principal, BTreeMap<String, Nat>>> = RefCell::new(BTreeMap::new());
}

#[update]
pub async fn users_apy(canister_id: Principal, pool_name: String, fee: Nat, token_amount: Nat) {
    let swap_fee = (token_amount.clone() * fee.clone()) / Nat::from(1000u128); // Convert to appropriate scale

    // Retrieve the list of users participating in this pool
    let users: Vec<Principal> = USERS_POOL.with(|users_pool| {
        users_pool
            .borrow()
            .iter()
            .filter_map(|(user, pools)| {
                if pools.contains(&pool_name) {
                    Some(*user)
                } else {
                    None
                }
            })
            .collect()
    });

    if users.is_empty() {
        ic_cdk::println!("No users found in pool: {}", pool_name);
        return;
    }

    let mut total_pool_balance = Nat::from(0u128);
    let mut user_balances: BTreeMap<Principal, Nat> = BTreeMap::new();

    // Calculate total pool balance and user contributions
    for user in &users {
        let user_balance: Nat = icrc_get_balance(*user, canister_id).await.unwrap_or(Nat::from(0u128));
        total_pool_balance += user_balance.clone();
        user_balances.insert(*user, user_balance);
    }

    if total_pool_balance == Nat::from(0u128) {
        ic_cdk::println!("Total pool balance is zero. No distribution possible.");
        return;
    }

    // Distribute the swap fee to each user proportionally
    for (user, balance) in user_balances {
        let user_share = (balance.clone() * swap_fee.clone()) / total_pool_balance.clone();

        // Store the user's share in FEE_SHARE
        FEE_SHARE.with(|fee_share| {
            let mut fee_data = fee_share.borrow_mut();
            let pool_fees = fee_data.entry(canister_id.clone()).or_insert_with(BTreeMap::new);

            pool_fees
                .entry(pool_name.clone())
                .and_modify(|fee| *fee += user_share.clone())
                .or_insert(user_share.clone());

            ic_cdk::println!(
                "Distributed {} LP tokens to user {} for pool {}",
                user_share,
                user,
                pool_name
            );
        });
    }

    // total_apr();
}



// #[update]
// pub fn total_apr() {
//     FEE_SHARE.with(|fee_share| {
//         let mut fee_data = fee_share.borrow_mut();


//     });
//     ic_cdk::println!("Total APR distribution completed successfully.");
// }
