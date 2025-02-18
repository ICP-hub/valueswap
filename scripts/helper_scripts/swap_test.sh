# dfx canister call valueswap_backend out_given_in '(
#     500000000 : nat,
#     30 : nat,
#     141000000000000000000 : nat,
#     70  : nat,
#     100000000 : nat
# )'

# INPUT_TOKEN=100000000;
# IDENTITY=$(dfx identity get-principal)
# VALUESWAP_BACKEND=$(dfx canister id valueswap_backend)

dfx canister call valueswap_backend pre_compute_swap '(
    record {
        token1_name = "ckbtc";
        token_amount = 100000000 : nat;
        token2_name = "cketh";
        ledger_canister_id1 = principal "br5f7-7uaaa-aaaaa-qaaca-cai";
        ledger_canister_id2 = principal "bw4dl-smaaa-aaaaa-qaacq-cai";
        fee = 30
    }
)'

# echo "Approving valueswap_backend to transfer 1000000000 tokens on behalf of DevJourney"

# APPROVE=$(
#     dfx --identity Harshit canister call ckbtc icrc2_approve "(record{ amount = 1000000000 ; spender = record { owner = principal \"$VALUESWAP_BACKEND\"}})"
# )
# echo "Approve result: $APPROVE"

# echo "Checking allowance"
# ALLOWANCE=$(
#     dfx --identity Harshit canister call ckbtc icrc2_allowance "(record { account = record {owner = principal \"$IDENTITY\"}; spender = record { owner = principal \"$VALUESWAP_BACKEND\"}})"
# )
# echo "Allowance result : $ALLOWANCE"

# # dfx canister call ckbtc icrc1_balance_of "(record {owner=principal\"hyhkx-53cuq-lmkqq-yhjmt-eve7b-j5pyf-3evrj-tncch-ilmtl-nrcee-sqe\"; subaccount=null; memo=null; from_subaccount=null; created_at_time=null;})"

# dfx canister call ckbtc icrc1_transfer ' (record {to=record {owner = principal "um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=100000000})'

# dfx identity use Harshit

# dfx canister call valueswap_backend compute_swap '(
#     record {
#         token1_name = "ckbtc";
#         token_amount = 100000000 : nat;
#         token2_name = "cketh";
#         ledger_canister_id1 = principal "br5f7-7uaaa-aaaaa-qaaca-cai";
#         ledger_canister_id2 = principal "bw4dl-smaaa-aaaaa-qaacq-cai";
#     }
# )'

# dfx canister call ckbtc icrc1_balance_of "(record {owner=principal\"um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae\"; subaccount=null; memo=null; from_subaccount=null; created_at_time=null;})"
# dfx canister call cketh icrc1_balance_of "(record {owner=principal\"um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae\"; subaccount=null; memo=null; from_subaccount=null; created_at_time=null;})"





