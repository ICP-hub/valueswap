#!/bin/bash

# Load environment variables from the correct path
if [ -f "/home/ray/valueswap/.env" ]; then
  export $(grep -v '^#' /home/ray/valueswap/.env | xargs)
else
  echo "Error: .env file not found!"
  exit 1
fi

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Fetch ledger IDs from environment variables
cketh_ledger_id=$CANISTER_ID_CKETH
ckbtc_ledger_id=$CANISTER_ID_CKBTC

# Correct and valid pool data
token_name="ValidToken"
balance="100000 : nat"  # Valid balance
weight="10 : nat"       # Valid weight
value="100 : nat"       # Valid value
image_url="img"
swap_fee="5 : nat"      # Valid swap fee

# Create pools using valid ledger IDs alternately for diversity
echo "Creating pool with ledger ID for cketh..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$cketh_ledger_id\"; image = \"$image_url\"; }}; swap_fee = $swap_fee; })"

echo "Creating pool with ledger ID for ckbtc..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$ckbtc_ledger_id\"; image = \"$image_url\"; }}; swap_fee = $swap_fee; })"

echo "Both pools created successfully."
