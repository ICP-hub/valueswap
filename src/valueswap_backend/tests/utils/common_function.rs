use crate::utils::structs::*;
use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};

pub fn get_user_principal() -> Principal {
    Principal::from_text("4jwha-xpj7p-sk2lp-bdo4u-cijhx-xskuu-qj34g-kqty4-n6jhy-ixgjd-aqe").unwrap()
}

pub fn swapper_user_principal() -> Principal {
    Principal::from_text("zjufx-s5v2q-jv7jn-a5qra-pw5vc-rt4hf-arhzn-7kzkq-56msw-xhq5o-yae").unwrap()
}

pub fn generate_principals(count: usize) -> Vec<Principal> {
    (1..=count)
        .map(|i| Principal::from_slice(&[i as u8; 29])) // 29-byte unique Principal
        .collect()
}


pub fn icrc2_approve(
    pic: &PocketIc,
    backend_canister: Principal,
    ckbtc_canister: Principal,
    user_principal: Principal,
) {
    let approval_args = ApproveArgs {
        fee: None,
        memo: None,
        from_subaccount: None,
        created_at_time: None,
        amount: Nat::from(100_000_000u128),
        expected_allowance: None,
        expires_at: None,
        spender: Account {
            owner: backend_canister,
            subaccount: None,
        },
    };

    let encoded_args = candid::encode_args((approval_args,)).unwrap();

    let response = pic
        .update_call(
            ckbtc_canister,
            user_principal,
            "icrc2_approve",
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(data) => {
            let result: Result<Nat, ApproveError> = candid::decode_one(&data).unwrap();
            match result {
                Ok(allowance) => {
                    println!("☑️  icrc1 Approval Approving...");
                    println!("✅ icrc1 Approval successful. Allowance granted");
                    assert!(
                        allowance > Nat::from(0u64),
                        "Allowance should be greater than zero."
                    );
                }
                Err(e) => panic!("Approval failed with error: {:?}", e),
            }
        }
        WasmResult::Reject(message) => panic!("Approval failed with message: {}", message),
    }
}

pub fn set_canister_id(
    pic: &PocketIc,
    backend_canister: Principal,
    pool_name: String,
    target_canister_id: Principal,
) {
    let encoded_args = candid::encode_args((pool_name.to_string(), target_canister_id)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            get_user_principal(),      // the caller principal
            "set_canister_id", // the update method name
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(_) => {
            println!(
                "✅ Canister ID successfully set for Lp ledger canister ➜ \"{}\"",
                pool_name
            );
        }
        WasmResult::Reject(msg) => {
            panic!("❌ Error: set_canister_id rejected! Message ➜ {}", msg);
        }
    }
}

pub fn remove_canister_id_by_name_pocket_ic(
    pic: &PocketIc,
    backend_canister: Principal,
    caller: Principal,
    name: String,
) {
    let args = candid::encode_args((name.to_string(),)).unwrap();
    let result = pic
        .update_call(backend_canister, caller, "remove_canister_id_by_name", args)
        .unwrap();

    match result {
        WasmResult::Reply(_) => {
            ic_cdk::println!(
                    "🟢 Success: Canister ID '{}' was removed successfully via `remove_canister_id_by_name`.",
                    name
                );
        }
        WasmResult::Reject(reason) => {
            ic_cdk::println!(
                "🔴 Error: Failed to remove canister ID '{}'. Reason: {}",
                name,
                reason
            );
        }
    }
}

pub fn get_pool_lp_tokens(
    pic: &PocketIc,
    backend_canister: Principal,
    caller: Principal,
    pool_name: String,
    label: Option<&str>, // Optional context label: "Before rollback", "After rollback"
) -> Nat {
    let args = candid::encode_args((pool_name.clone(),)).unwrap();

    let response = pic
        .query_call(backend_canister, caller, "get_pool_lp_tokens", args)
        .unwrap();

    match response {
        WasmResult::Reply(bytes) => {
            let result: Nat = candid::decode_one(&bytes).unwrap();
            match label {
                Some(context) => println!("✅ [{}] LP tokens for pool '{}': {}", context, pool_name, result),
                None => println!("✅ LP tokens for pool '{}': {}", pool_name, result),
            }
            result
        }
        WasmResult::Reject(msg) => {
            let err = match label {
                Some(context) => format!(
                    "❌ [{}] Failed to query LP tokens for pool '{}': {}",
                    context, pool_name, msg
                ),
                None => format!(
                    "❌ Failed to query LP tokens for pool '{}': {}",
                    pool_name, msg
                ),
            };
            println!("{}", err);
            panic!("{}", err); // Or convert this to a Result if needed
        }
    }
}

pub fn get_user_pool_lp_for_token(
    pic: &PocketIc,
    backend_canister: Principal,
    caller: Principal,
    token_name: String,
) -> Option<Nat> {
    let args = candid::encode_args((caller, token_name.clone())).unwrap();

    let response = pic
        .query_call(backend_canister, caller, "get_user_pool_lp_for_token", args)
        .unwrap();

    match response {
        WasmResult::Reply(bytes) => {
            let result: Option<Nat> = candid::decode_one(&bytes).unwrap();
            match &result {
                Some(val) => println!("✅ LP tokens for '{}' (user {}): {}", token_name, caller, val),
                None => println!("⚠️  No LP tokens found for '{}' (user {})", token_name, caller),
            }
            result
        }
        WasmResult::Reject(msg) => {
            let err = format!(
                "❌ Query rejected for token '{}' (user {}): {}",
                token_name, caller, msg
            );
            println!("{}", err);
            panic!("{}", err);
        }
    }
}

