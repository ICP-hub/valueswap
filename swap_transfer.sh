#!/bin/bash

# Define the canister name
CANISTER_NAME="valueswap_backend"

# Function to call the canister with the provided arguments
test_out_given_in() {
  local b_i=$1
  local w_i=$2
  local b_o=$3
  local w_o=$4
  local amount_in=$5
  local fee=$6

  echo "Testing out_given_in with values: ($b_i, $w_i, $b_o, $w_o, $amount_in, $fee)"
  
  # Execute the dfx canister call
  dfx canister call $CANISTER_NAME out_given_in "($b_i, $w_i, $b_o, $w_o, $amount_in, $fee)"
  
  echo "------------------------------------------------------"
}

# Start of the script
echo "Starting tests for out_given_in function..."

# Test cases
test_out_given_in 500000000 30 141000000000000000000 70 1 0
test_out_given_in 500000000 30 141000000000000000000 70 2 0
test_out_given_in 1000000000 40 150000000000000000000 60 10 1
test_out_given_in 750000000 25 160000000000000000000 75 5 0
test_out_given_in 250000000 50 130000000000000000000 50 3 0

# End of the script
echo "Tests completed."
