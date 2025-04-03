#!/bin/bash
./sendApproval.sh

# Load environment variables from the correct path
source "/home/ray/valueswap/.env"

# Define base command for dfx canister call
base_command="dfx canister call valueswap_backend create_pools"

# Define token ledger IDs and other parameters
declare -a token_ledger_ids=($CANISTER_ID_CKBTC $CANISTER_ID_CKETH $CANISTER_ID_CKUSDC)
balances=("100000 : nat" "50000 : nat" "25000 : nat")
weights=("50 : nat" "30 : nat" "10 : nat") # Total weight 90%, less than 100%
value="100 : nat"
image_url="default_img.png"
swap_fee="5 : nat"

# Begin creating pool command
echo "Creating pool with three tokens having weights of 50%, 30%, and 10%..."
pool_command="record { pool_data = vec {"

# Append each token's pool parameters
for i in "${!token_ledger_ids[@]}"; do
    token_name="Token$(($i + 1))"
    ledger_id="${token_ledger_ids[$i]}"
    balance="${balances[$i]}"
    weight="${weights[$i]}"
    pool_command+="record { token_name = \"$token_name\"; balance = $balance; weight = $weight; value = $value; ledger_canister_id = principal \"$ledger_id\"; image = \"$image_url\"; };"
done

# Close the pool data vector and append swap fee
pool_command+="}; swap_fee = $swap_fee; }"

# Execute the pool creation command
$base_command "$pool_command"

echo "Pool with three tokens created successfully with specified weights."
