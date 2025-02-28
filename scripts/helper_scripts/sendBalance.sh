dfx identity use DevJourney

dfx canister call cketh icrc1_transfer ' (record {to=record {owner = principal "um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=5000000000})'

dfx canister call ckbtc icrc1_transfer ' (record {to=record {owner = principal "um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae"; subaccount=null}; fee=null; memo=null; from_subaccount=null; created_at_time=null; amount=5000000000})'

# ./approval.sh

# minter=$(dfx canister id swap)
# echo "$minter"