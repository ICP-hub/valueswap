#!/bin/bash

# Define invalid token names
invalid_names=("" "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa" "!@#$%^&*()_+=-[]';,/{}|:<>?~")

# Base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Loop through invalid token names
for name in "${invalid_names[@]}"
do
  # Construct the input for the pool data with an invalid token name for cketh and a valid name for ckbtc
  pool_data="(
    record {
      pool_data = vec {
        record {
          token_name = \"$name\";
          weight = 10 : nat;
          balance = 1000 : nat;
          value = 500 : nat;
          image = \"https://example.com/token.png\";
          ledger_canister_id = principal \"b77ix-eeaaa-aaaaa-qaada-cai\";
        };
        record {
          token_name = \"ckbtc\";
          weight = 10 : nat;
          balance = 1000 : nat;
          value = 500 : nat;
          image = \"https://example.com/token.png\";
          ledger_canister_id = principal \"bw4dl-smaaa-aaaaa-qaacq-cai\";
        };
      };
      swap_fee = 5 : nat;
    },
  )"

  echo "Testing with token name: $name"
  # Execute the command and capture the output
  output=$($base_command "$pool_data")

  # Display the output
  echo "Output:"
  echo "$output"
  echo "-----------------------------------"
done

echo "Tests completed."
