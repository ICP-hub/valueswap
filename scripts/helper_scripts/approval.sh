dfx identity use Harshit
# dfx canister call cketh icrc2_approve '(
#   record {
#     fee = null;
#     memo = null;
#     from_subaccount = null;
#     created_at_time = null;
#     amount = 5_000_000_000 : nat;
#     expected_allowance = null;
#     expires_at = null;
#     spender = record {
#       owner = principal "be2us-64aaa-aaaaa-qaabq-cai";
#       subaccount = null;
#     };
#   },
# )'
dfx canister call ckbtc icrc2_approve '(
  record {
    fee = null;
    memo = null;
    from_subaccount = null;
    created_at_time = null;
    amount = 5_000_000_000 : nat;
    expected_allowance = null;
    expires_at = null;
    spender = record {
      owner = principal "be2us-64aaa-aaaaa-qaabq-cai";
      subaccount = null;
    };
  },
)'
