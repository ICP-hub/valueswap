#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Fetch ledger IDs from environment variables
token1_ledger_id=$CANISTER_ID_CKETH
token2_ledger_id=$CANISTER_ID_CKBTC
token3_ledger_id=$CANISTER_ID_CKUSDC

# Define pool parameters
token_name1="CKBTC"
token_name2="CKETH"
token_name3="CKUSDC"
balance1="300000 : nat"
balance2="400000 : nat"
balance3="300000 : nat"
weight1="30 : nat"
weight2="40 : nat"
weight3="30 : nat"
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Create pool with specified token weights and balances
echo "Creating pool with weights 30%, 40%, and 30% for Token1, Token2, and Token3..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name1\"; balance = $balance1; weight = $weight1; value = $value; ledger_canister_id = principal \"$token1_ledger_id\"; image = \"$image_url\"; }; record { token_name = \"$token_name2\"; balance = $balance2; weight = $weight2; value = $value; ledger_canister_id = principal \"$token2_ledger_id\"; image = \"$image_url\"; }; record { token_name = \"$token_name3\"; balance = $balance3; weight = $weight3; value = $value; ledger_canister_id = principal \"$token3_ledger_id\"; image = \"$image_url\"; } }; swap_fee = $swap_fee; })"

echo "Pool with weights 30%, 40%, and 30% created successfully."
