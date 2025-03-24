#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Fetch ledger IDs from environment variables
token1_ledger_id=$CANISTER_ID_CKETH
token2_ledger_id=$CANISTER_ID_CKBTC

# Define pool parameters with invalid weight (negative weight for Token1)
token_name1="CKBTC"
token_name2="CKETH"
balance1="100000 : nat"
balance2="200000 : nat"
weight1="-10 : nat"  # Invalid weight for Token1 (negative value)
weight2="110 : nat"  # Invalid weight for Token2 (greater than 100)
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Create pool with specified token weights and balances
echo "Creating pool with invalid weight for Token1 (-10) and Token2 (110)..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name1\"; balance = $balance1; weight = $weight1; value = $value; ledger_canister_id = principal \"$token1_ledger_id\"; image = \"$image_url\"; }; record { token_name = \"$token_name2\"; balance = $balance2; weight = $weight2; value = $value; ledger_canister_id = principal \"$token2_ledger_id\"; image = \"$image_url\"; } }; swap_fee = $swap_fee; })"

echo "Pool creation with invalid token weight should fail."
