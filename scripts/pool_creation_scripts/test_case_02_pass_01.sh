#!/bin/bash
./sendBalance.sh

source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Ledger IDs for CKETH, CKBTC, CKUSDC
ledger_ids=($CANISTER_ID_CKETH $CANISTER_ID_CKBTC $CANISTER_ID_CKUSDC)

# Simulate multiple simultaneous calls for different tokens
for i in {1..3}
do
   # Cycle through the ledger IDs for each iteration
   ledger_id=${ledger_ids[$((i % 3))]}  # 3 different ledger IDs for the pool tokens
   
   # Adjust token name dynamically
   token_name="Token$i"

   echo "Starting call $i with ledger ID $ledger_id in background"

   # Prepare the pool data for each concurrent test run
   $base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = 100000 : nat; weight = 30 : nat; value = 100 : nat; ledger_canister_id = principal \"$ledger_id\"; image = \"https://example.com/image$i.png\"; }}; swap_fee = 5 : nat; })" &

done

# Wait for all background processes to complete
wait
echo "All concurrent pool creation calls have been made."
