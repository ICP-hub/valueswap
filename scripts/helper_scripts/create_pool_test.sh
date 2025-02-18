#!/bin/bash

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Ledger IDs for cketh and ckbtc
cketh_ledger_id="$(dfx canister id cketh)"
ckbtc_ledger_id="$(dfx canister id ckbtc)"

# Simulate multiple simultaneous calls using both ledger IDs alternately
for i in {1..1}
do
   # Use cketh_ledger_id for odd indexed calls and ckbtc_ledger_id for even indexed calls
   if [ $((i % 2)) -eq 0 ]; then
       ledger_id=$ckbtc_ledger_id
   else
       ledger_id=$cketh_ledger_id
   fi
   
   echo "Starting call $i with ledger ID $ledger_id in background"
   $base_command "(record { pool_data = vec { record { token_name = \"Token$i\"; balance = 10000000 : nat; weight = 100 : nat; value = 100 : nat; ledger_canister_id = principal \"$ledger_id\"; image = \"https://example.com/image$i.png\"; }}; swap_fee = 5 : nat; })" &
done

wait
echo "All concurrent calls have been made."

