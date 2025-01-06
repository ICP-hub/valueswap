#!/bin/bash

set -e

# Get the principal ID for the specified identity
identity_name=${DFX_IDENTITY:-default}
principalId=$(dfx --identity $identity_name identity get-principal)
echo "Principal ID: $principalId"

# Get the Canister ID of the backend canister
CANISTER=$(dfx canister id valueswap_backend)
echo "Canister ID: $CANISTER"

# Function to create a single pool
create_pool() {
    local pool_data=$1
    local swap_fee=$2

    # Call the backend canister function to create the pool
    response=$(dfx canister call $CANISTER create_pools "(record { pool_data = $pool_data; swap_fee = $swap_fee : nat })")
    echo "Response: $response"

    # Error handling
    if [[ "$response" == *"Err"* ]]; then
        echo "Error occurred while creating pool: $response"
        exit 1
    fi
}

# Define data for 3 pools
declare -a pools=(
    "vec { record { weight=25 : nat; balance=100000000 : nat; value=48000000000000 : nat; image=\"https://coin-images.coingecko.com/coins/images/14495/large/Internet_Computer_logo.png?1696514180\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"DAI\" }; record { weight=20 : nat; balance=20000000 : nat; value=1340000000000 : nat; image=\"https://coin-images.coingecko.com/coins/images/25788/large/Asset_19.png?1703173153\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"FFS\" } }"
    "vec { record { weight=30 : nat; balance=50000000 : nat; value=24000000000000 : nat; image=\"https://example.com/token1.png\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"BTC\" }; record { weight=15 : nat; balance=10000000 : nat; value=670000000000 : nat; image=\"https://example.com/token2.png\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"ETH\" } }"
    "vec { record { weight=10 : nat; balance=30000000 : nat; value=16000000000000 : nat; image=\"https://example.com/token3.png\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"USDT\" }; record { weight=40 : nat; balance=40000000 : nat; value=800000000000 : nat; image=\"https://example.com/token4.png\"; ledger_canister_id=principal \"b77ix-eeaaa-aaaaa-qaada-cai\"; token_name=\"ADA\" } }"
)

# Swap fee for all pools (as a nat)
swap_fee=2

# Create each pool
for pool_data in "${pools[@]}"; do
    echo "Creating pool with data: $pool_data"
    create_pool "$pool_data" "$swap_fee"
done

echo "All pools created successfully!"
