#!/bin/bash

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Use the blackhole principal ID, which is syntactically correct but does not correspond to any canister
invalid_ledger_id="aaaaa-aa"

# Define test cases
declare -a tests=(
  # Test with a syntactically correct but non-existent ledger canister ID
  "(record { pool_data = vec { record { token_name = \"ValidToken\"; balance = 100 : nat; weight = 10 : nat; value = 100 : nat; ledger_canister_id = principal \"$invalid_ledger_id\"; image = \"https://example.com/valid-image.png\"; }}; swap_fee = 5 : nat; })"
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
