use crate::utils::structs::*;
use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};

pub fn get_user_principal() -> Principal {
    Principal::from_text("4jwha-xpj7p-sk2lp-bdo4u-cijhx-xskuu-qj34g-kqty4-n6jhy-ixgjd-aqe").unwrap()
}

pub fn icrc2_approve(pic: &PocketIc, backend_canister: Principal, ckbtc_canister: Principal) {
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
            get_user_principal(),
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
    pool_name: &str,
    target_canister_id: Principal,
) {
    let encoded_args = candid::encode_args((pool_name.to_string(), target_canister_id)).unwrap();

    let response = pic
        .update_call(
            backend_canister,
            get_user_principal(),      // the caller principal
            "set_canister_id_by_name", // the update method name
            encoded_args,
        )
        .unwrap();

    match response {
        WasmResult::Reply(_) => {
            println!("✅ set_canister_id call successful for '{}'", pool_name);
        }
        WasmResult::Reject(msg) => {
            panic!("❌ set_canister_id rejected with message: {}", msg);
        }
    }
}

pub fn remove_canister_id_by_name_pocket_ic(
    pic: &PocketIc,
    backend_canister: Principal,
    caller: Principal,
    name: &str,
) {
    let args = candid::encode_args((name.to_string(),)).unwrap();
    let result = pic
        .update_call(backend_canister, caller, "remove_canister_id_by_name", args)
        .unwrap();

    match result {
        WasmResult::Reply(_) => {
            ic_cdk::println!(
                "✅ Called remove_canister_id_by_name('{}') successfully",
                name
            );
        }
        WasmResult::Reject(reason) => {
            ic_cdk::println!(
                "❌ Failed to call remove_canister_id_by_name('{}'): {}",
                name,
                reason
            );
        }
    }
}

pub fn get_user_pool_by_principal(
    pic: &PocketIc,
    backend_canister: Principal,
    caller: Principal,
    principal_id: Principal,
) -> Result<String, String> {
    let args = candid::encode_args((principal_id,)).unwrap();

    let response = pic
        .query_call(backend_canister, caller, "get_user_pool_by_principal", args)
        .unwrap();

    match response {
        WasmResult::Reply(bytes) => {
            let result: Result<String, String> = candid::decode_one(&bytes).unwrap();
            match &result {
                Ok(pool_name) => println!("✅ User's pool: {}", pool_name),
                Err(err_msg) => println!("❌ Failed to get user pool: {}", err_msg),
            }
            result
        }
        WasmResult::Reject(msg) => {
            let err = format!("❌ Query rejected: {}", msg);
            println!("{}", err);
            Err(err)
        }
    }
}
