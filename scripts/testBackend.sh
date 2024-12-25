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
    weight1=0.25
    balance1=100000000000     # nat64 (no decimal)
    value1=50000000000000     # nat64 (no decimal)
    image1="https://coin-images.coingecko.com/coins/images/14495/large/Internet_Computer_logo.png?1696514180"
    token_name1="ckBTC"

    # Token 2 details
    weight2=0.20
    balance2=20000000000     # nat64 (no decimal)
    value2=1000000000000     # nat64 (no decimal)
    image2="https://coin-images.coingecko.com/coins/images/25788/large/Asset_19.png?1703173153"
    token_name2="ckETH"

    # Swap fee (float64)
    swap_fee=0.005

    # Construct the pool_data Candid structure with proper quoting for text values
    pool_data="vec{record {weight=$weight1; balance=$balance1; value=$value1; image=\"$image1\"; token_name=\"$token_name1\"}; record {weight=$weight2; balance=$balance2; value=$value2; image=\"$image2\"; token_name=\"$token_name2\"}}"

    # Call the backend canister function to create the pool
    dfx canister call $CANISTER create_pools "(record { pool_data = $pool_data; swap_fee = $swap_fee })"
}

# Execute the create_pools function
create_pools
