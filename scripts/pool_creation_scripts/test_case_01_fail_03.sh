#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Fetch ledger IDs from environment variables
token1_ledger_id=$CANISTER_ID_CKETH
token2_ledger_id=$CANISTER_ID_CKBTC

# Define pool parameters with an invalid balance (zero balance for Token1)
token_name1="CKBTC"
token_name2="CKETH"
balance1="0 : nat"  # Invalid balance for Token1 (zero)
balance2="100000 : nat"  # Valid balance for Token2
weight1="50 : nat"
weight2="50 : nat"
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Create pool with specified token balances and weights
echo "Creating pool with invalid balance (0) for Token1..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name1\"; balance = $balance1; weight = $weight1; value = $value; ledger_canister_id = principal \"$token1_ledger_id\"; image = \"$image_url\"; }; record { token_name = \"$token_name2\"; balance = $balance2; weight = $weight2; value = $value; ledger_canister_id = principal \"$token2_ledger_id\"; image = \"$image_url\"; } }; swap_fee = $swap_fee; })"

echo "Pool creation with invalid token balance should fail."
