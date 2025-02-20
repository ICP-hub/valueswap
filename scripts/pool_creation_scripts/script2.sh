#!/bin/bash

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Use the specific ledger canister IDs for cketh and ckbtc
cketh_ledger_id="b77ix-eeaaa-aaaaa-qaada-cai"
ckbtc_ledger_id="bw4dl-smaaa-aaaaa-qaacq-cai"

# Define test cases
declare -a tests=(
  # Test with zero balance
  "(record { pool_data = vec { record { token_name = \"ValidToken\"; balance = 0 : nat; weight = 10 : nat; value = 100 : nat; ledger_canister_id = principal \"$cketh_ledger_id\"; image = \"https://example.com/valid-image.png\"; }}; swap_fee = 5 : nat; })"

  # Test with zero weight
  "(record { pool_data = vec { record { token_name = \"ValidToken\"; balance = 100 : nat; weight = 0 : nat; value = 100 : nat; ledger_canister_id = principal \"$ckbtc_ledger_id\"; image = \"https://example.com/valid-image.png\"; }}; swap_fee = 5 : nat; })"

  # Test with zero value
  "(record { pool_data = vec { record { token_name = \"ValidToken\"; balance = 100 : nat; weight = 10 : nat; value = 0 : nat; ledger_canister_id = principal \"$cketh_ledger_id\"; image = \"https://example.com/valid-image.png\"; }}; swap_fee = 5 : nat; })"
)

# Loop through test cases
for test in "${tests[@]}"
do
  echo "Testing with input: $test"
  # Execute the command and capture the output
  output=$($base_command "$test")

  # Display the output
  echo "Output:"
  echo "$output"
  echo "-----------------------------------"
done

echo "Tests completed."
