#!/bin/bash

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}Starting ValueSwap Pool Creation and Withdrawal Test${NC}"

# Get the canister IDs
BACKEND_CANISTER=$(dfx canister id valueswap_backend)
CKBTC_CANISTER=$(dfx canister id ckbtc)
CKETH_CANISTER=$(dfx canister id cketh)

echo "Backend Canister ID: $BACKEND_CANISTER"
echo "CKBTC Canister ID: $CKBTC_CANISTER"
echo "CKETH Canister ID: $CKETH_CANISTER"

# Function to create pool data
create_pool_data() {
    cat << EOF
{
    "pool_data": [
        {
            "token_name": "ckbtc",
            "balance": "100000000",
            "weight": "50",
            "value": "100",
            "ledger_canister_id": "$CKBTC_CANISTER",
            "image": "image.png"
        },
        {
            "token_name": "cketh",
            "balance": "2800000000",
            "weight": "50",
            "value": "100",
            "ledger_canister_id": "$CKETH_CANISTER",
            "image": "image.png"
        }
    ],
    "swap_fee": "5"
}
EOF
}

# Function to create approval data
create_approval_data() {
    cat << EOF
{
    "fee": null,
    "memo": null,
    "from_subaccount": null,
    "created_at_time": null,
    "amount": "10000000000000",
    "expected_allowance": null,
    "expires_at": null,
    "spender": {
        "owner": "$BACKEND_CANISTER",
        "subaccount": null
    }
}
EOF
}

# Approve token transfers
echo -e "\n${GREEN}Approving token transfers...${NC}"

# Approve CKBTC
echo "Approving CKBTC..."
dfx canister call ckbtc icrc2_approve "$(create_approval_data)" --argument-format candid

# Approve CKETH
echo "Approving CKETH..."
dfx canister call cketh icrc2_approve "$(create_approval_data)" --argument-format candid

# Create pool
echo -e "\n${GREEN}Creating pool...${NC}"
dfx canister call valueswap_backend create_pools "$(create_pool_data)" --argument-format candid

# Wait for pool creation to complete
echo "Waiting for pool creation to complete..."
sleep 5

# Get user's LP tokens
echo -e "\n${GREEN}Getting user's LP tokens...${NC}"
dfx canister call valueswap_backend get_user_pools_with_lp "$(dfx identity get-principal)" --argument-format candid

# Burn LP tokens
echo -e "\n${GREEN}Burning LP tokens...${NC}"
POOL_NAME="ckbtccketh"
AMOUNT="100000000"

# Create burn arguments
BURN_ARGS=$(cat << EOF
{
    "params": $(create_pool_data),
    "pool_name": "$POOL_NAME",
    "amount": "$AMOUNT",
    "ledger_canister_id": "$CKBTC_CANISTER"
}
EOF
)

# Execute burn
dfx canister call valueswap_backend burn_lp_tokens "$BURN_ARGS" --argument-format candid

# Verify remaining LP tokens
echo -e "\n${GREEN}Verifying remaining LP tokens...${NC}"
dfx canister call valueswap_backend get_user_pools_with_lp "$(dfx identity get-principal)" --argument-format candid

# Stop the local network
echo -e "\n${GREEN}Stopping local network...${NC}"
dfx stop

echo -e "${GREEN}Test completed!${NC}" 