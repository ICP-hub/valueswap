#!/bin/bash

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Ledger IDs for cketh and ckbtc
cketh_ledger_id="b77ix-eeaaa-aaaaa-qaada-cai"
ckbtc_ledger_id="bw4dl-smaaa-aaaaa-qaacq-cai"

# Correct and valid pool data
token_name="ValidToken"
balance="100 : nat"  # Valid balance
weight="10 : nat"    # Valid weight
value="100 : nat"    # Valid value
image_url="https://example.com/valid-image.png"
swap_fee="5 : nat"   # Valid swap fee

# Create pools using valid ledger IDs alternately for diversity
echo "Creating pool with ledger ID for cketh..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$cketh_ledger_id\"; image = \"$image_url\"; }}; swap_fee = $swap_fee; })"

echo "Creating pool with ledger ID for ckbtc..."
$base_command "(record { pool_data = vec { record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$ckbtc_ledger_id\"; image = \"$image_url\"; }}; swap_fee = $swap_fee; })"

echo "Both pools created successfully."
