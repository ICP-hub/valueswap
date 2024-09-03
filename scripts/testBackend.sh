#!/bin/bash

set -e

# Get the principal ID for the specified identity
principalId=$(dfx --identity default identity get-principal)
echo "Principal ID: $principalId"

# Get the Canister ID of the backend canister
CANISTER=$(dfx canister id valueswap_backend)
echo "Canister ID: $CANISTER"

# Function to create pools with two tokens
create_pools() {
    # Token 1 details
    weight1=0.50
    balance1=100000
    value1=5000000
    token_name1="TokenA"

    # Token 2 details
    weight2=0.50
    balance2=200000
    value2=10000000
    token_name2="TokenB"

    # Swap fee
    swap_fee=0.005

    # Construct the pool_data Candid structure
    pool_data="vec{record {weight=$weight1; balance=$balance1; value=$value1; token_name=\"$token_name1\"}; record {weight=$weight2; balance=$balance2; value=$value2; token_name=\"$token_name2\"}}"

    # Call the backend canister function to create the pool
    dfx canister call $CANISTER create_pools "(record { pool_data = $pool_data; swap_fee = $swap_fee })"
}

# Execute the create_pools function
create_pools
