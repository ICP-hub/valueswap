#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"  

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Fetch ledger IDs from environment variables
ckbtc_ledger_id=$CANISTER_ID_CKBTC
cketh_ledger_id=$CANISTER_ID_CKETH

# Define pool parameters
token_name_ckbtc="CKBTC"
token_name_cketh="CKETH"
balance_ckbtc="800000 : nat"  # Higher balance for higher weight
balance_cketh="200000 : nat"  # Lower balance for lower weight
weight_ckbtc="80 : nat"
weight_cketh="20 : nat"
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Create pool with specified token weights and balances
echo "Creating pool with varied weights for CKBTC and CKETH..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name_ckbtc\"; balance = $balance_ckbtc; weight = $weight_ckbtc; value = $value; ledger_canister_id = principal \"$ckbtc_ledger_id\"; image = \"$image_url\"; }; record { token_name = \"$token_name_cketh\"; balance = $balance_cketh; weight = $weight_cketh; value = $value; ledger_canister_id = principal \"$cketh_ledger_id\"; image = \"$image_url\"; } }; swap_fee = $swap_fee; })"

echo "Pool with varied weights created successfully."
