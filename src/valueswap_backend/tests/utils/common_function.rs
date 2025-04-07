use candid::{decode_one, encode_args, CandidType, Nat, Principal};
use pocket_ic::{PocketIc, WasmResult};
use crate::utils::structs::*;

pub fn get_user_principal() -> Principal {
    Principal::from_text("4jwha-xpj7p-sk2lp-bdo4u-cijhx-xskuu-qj34g-kqty4-n6jhy-ixgjd-aqe").unwrap()
}

pub fn icrc2_approve(pic: &PocketIc, backend_canister: Principal, ckbtc_canister: Principal,) {
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

