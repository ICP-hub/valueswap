# dfx canister call cketh_ledger icrc1_transfer ' (record {to=record {owner = principal "rgtib-ktq4g-rjiya-aishg-mv2om-ey54m-jhyut-2tuaf-tvjyz-ml54r-lae"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=500000000})'

# dfx canister call LP_ledger_canister icrc1_transfer ' (record {to=record {owner = principal "b77ix-eeaaa-aaaaa-qaada-cai"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=5000000000})'
#
dfx canister --network ic call LP_ledger_canister icrc1_balance_of "(record {
  owner = principal \"fm5wa-dkllv-d7h4z-uaukh-z5aqz-ck4hr-5oaxi-7m6hy-hxkgc-rufr3-xqe\";
})"

# dfx canister call LP_ledger_canister icrc1_balance_of "(record {owner=principal\"by6od-j4aaa-aaaaa-qaadq-cai\"; subaccount=null; memo=null; from_subaccount=null; created_at_time=null; amount=500000000})"
