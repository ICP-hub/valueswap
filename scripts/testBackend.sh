#!/bin/bash

set -e

# Get the principal ID for the specified identity
principalId=$(dfx --identity default identity get-principal)
echo "Principal ID: $principalId"

# Get the Canister ID of the backend canister
CANISTER=$(dfx canister id valueswap_backend)
echo "Canister ID: $CANISTER"

# Function to create pools with five tokens
create_pools() {
    # Token 1 details
    weight1=0.25
    balance1=100000     # nat64 (no decimal)
    value1=5000000      # nat64 (no decimal)
    image1="https://coin-images.coingecko.com/coins/images/14495/large/Internet_Computer_logo.png?1696514180"
    token_name1="icp"

    # Token 2 details
    weight2=0.20
    balance2=200000     # nat64 (no decimal)
    value2=10000000     # nat64 (no decimal)
    image2="https://coin-images.coingecko.com/coins/images/25788/large/Asset_19.png?1703173153"
    token_name2="ogy"

    # Token 3 details
    weight3=0.20
    balance3=150000     # nat64 (no decimal)
    value3=7500000      # nat64 (no decimal)
    image3="https://coin-images.coingecko.com/coins/images/37649/large/Gold_Dao_Logo.png?1715146761"
    token_name3="gldgov"

    # Token 4 details
    weight4=0.15
    balance4=180000     # nat64 (no decimal)
    value4=9000000      # nat64 (no decimal)
    image4="https://coin-images.coingecko.com/coins/images/34362/large/iDoge-Logo-200px.png?1704712572"
    token_name4="idoge"

    # Token 5 details
    weight5=0.20
    balance5=120000     # nat64 (no decimal)
    value5=6000000      # nat64 (no decimal)
    image5="https://coin-images.coingecko.com/coins/images/36765/large/icpanda-200x200.png?1712283814"
    token_name5="panda"

    # Swap fee (float64)
    swap_fee=0.001

    # Construct the pool_data Candid structure with proper quoting for text values
    pool_data="vec{record {weight=$weight1; balance=$balance1; value=$value1; image=\"$image1\"; token_name=\"$token_name1\"}; record {weight=$weight2; balance=$balance2; value=$value2; image=\"$image2\"; token_name=\"$token_name2\"}; record {weight=$weight3; balance=$balance3; value=$value3; image=\"$image3\"; token_name=\"$token_name3\"}; record {weight=$weight4; balance=$balance4; value=$value4; image=\"$image4\"; token_name=\"$token_name4\"}; record {weight=$weight5; balance=$balance5; value=$value5; image=\"$image5\"; token_name=\"$token_name5\"}}"

    # Call the backend canister function to create the pool
    dfx canister call $CANISTER create_pools "(record { pool_data = $pool_data; swap_fee = $swap_fee })"
}

# Execute the create_pools function
create_pools
