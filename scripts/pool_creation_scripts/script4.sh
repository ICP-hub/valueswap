#!/bin/bash

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Ledger IDs for cketh and ckbtc
cketh_ledger_id="b77ix-eeaaa-aaaaa-qaada-cai"
ckbtc_ledger_id="bw4dl-smaaa-aaaaa-qaacq-cai"

# Simulate multiple simultaneous calls using both ledger IDs alternately
for i in {1..10}
do
   # Use cketh_ledger_id for odd indexed calls and ckbtc_ledger_id for even indexed calls
   if [ $((i % 2)) -eq 0 ]; then
       ledger_id=$ckbtc_ledger_id
   else
       ledger_id=$cketh_ledger_id
   fi
   
   echo "Starting call $i with ledger ID $ledger_id in background"
   $base_command "(record { pool_data = vec { record { token_name = \"Token$i\"; balance = 100 : nat; weight = 10 : nat; value = 100 : nat; ledger_canister_id = principal \"$ledger_id\"; image = \"https://example.com/image$i.png\"; }}; swap_fee = 5 : nat; })" &
done

wait
echo "All concurrent calls have been made."
