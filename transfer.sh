#!/bin/bash
set -e

# Define the canister IDs and user principal
CANISTER_ID_CKBTC_LEDGER="bd3sg-teaaa-aaaaa-qaaba-cai"  # Ledger canister ID
USER_PRINCIPAL="um2uc-3w7jg-5lycg-pmjio-is7ms-jjzwo-kvwfa-xdeus-ur2tx-4sbzd-cae"  # Harshit's user principal
CANISTER_ID_SWAP="bw4dl-smaaa-aaaaa-qaacq-cai"  # Swap canister ID (target canister)
CANISTER_ID_VALUESWAP_BACKEND="br5f7-7uaaa-aaaaa-qaaca-cai"  # Valueswap backend canister ID
APPROVAL_AMOUNT=10000  # Amount to approve and transfer

# Step 1: Approve the valueswap_backend canister to transfer tokens on behalf of Harshit
echo "Approving valueswap_backend canister to transfer $APPROVAL_AMOUNT tokens on behalf of Harshit..."
APPROVE_RESULT=$(dfx canister call "$CANISTER_ID_CKBTC_LEDGER" icrc2_approve "(record { amount = $APPROVAL_AMOUNT:nat; spender = record { owner = principal \"$CANISTER_ID_VALUESWAP_BACKEND\"} })")
echo "Approval result: $APPROVE_RESULT"

# Step 2: Call the deposit_tokens function to perform the transfer (ensure the amount is nat64)
echo "Transferring $APPROVAL_AMOUNT tokens from Harshit to the swap canister ($CANISTER_ID_SWAP)..."
TRANSFER_RESULT=$(dfx canister call "$CANISTER_ID_VALUESWAP_BACKEND" deposit_tokens "($APPROVAL_AMOUNT:nat, principal \"$USER_PRINCIPAL\", principal \"$CANISTER_ID_SWAP\")")
echo "Transfer result: $TRANSFER_RESULT"

# Step 3: Check the balance of the swap canister
echo "Checking the balance of the swap canister ($CANISTER_ID_SWAP)..."
RECIEVER_BALANCE=$(dfx canister call "$CANISTER_ID_CKBTC_LEDGER" icrc1_balance_of "(record {owner = principal \"$CANISTER_ID_SWAP\"})")
echo "Swap canister's balance: $RECIEVER_BALANCE"