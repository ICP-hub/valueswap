#!/bin/bash
./sendLessApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Define parameters for a pool with an unrealistically high token balance
token_name="CKBTC"
ledger_id=$CANISTER_ID_CKBTC
excessive_balance="1000000000 : nat"  # Excessively high balance to ensure failure
weight="50 : nat"
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Attempt to create a pool with an insufficient balance
echo "Attempting to create pool with excessive token balance..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = $excessive_balance; weight = $weight; value = $value; ledger_canister_id = principal \"$ledger_id\"; image = \"$image_url\" } }; swap_fee = $swap_fee; })"

echo "Test completed: Expected failure due to insufficient balance."
