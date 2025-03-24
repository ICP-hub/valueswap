#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Define token ledger IDs and other parameters
declare -a token_ledger_ids=($CANISTER_ID_CKETH $CANISTER_ID_CKBTC $CANISTER_ID_USDC $CANISTER_ID_CKETH $CANISTER_ID_CKBTC $CANISTER_ID_USDC $CANISTER_ID_CKETH $CANISTER_ID_CKBTC)
balance="50000 : nat"
weight="125 : nat"  # Adjusted for integer value as percentage points
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Begin creating pool command
echo "Creating pool with eight tokens, each having 12.5% weight..."
pool_command="record { pool_data = vec {"

# Append each token's pool parameters
for i in "${!token_ledger_ids[@]}"; do
    token_name="Token$(($i + 1))"
    ledger_id="${token_ledger_ids[$i]}"
    pool_command+="record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$ledger_id\"; image = \"$image_url\" }"
    if [ $i -lt $((${#token_ledger_ids[@]} - 1)) ]; then
        pool_command+="; "  # Add semicolon separator
    fi
done

# Close the pool data vector and append swap fee
pool_command+=" }; swap_fee = $swap_fee; }"

# Execute the pool creation command
$base_command "$pool_command"

echo "Pool with eight tokens, each with effectively 12.5% weight, created successfully."
