#!/bin/bash


set -e

# Use the Harshit identity for the approval and transfer
dfx identity use Harshit
DEFAULT=$(dfx identity get-principal)
CANISTER=$(dfx canister id valueswap_backend)
RECIEVER="be2us-64aaa-aaaaa-qaabq-cai"
LEDGER_CANISTER_ID=$(dfx canister id ckbtc)

echo "Harshit principal (DEFAULT): $DEFAULT"
echo "Valueswap Backend Canister: $CANISTER"
echo "Receiver Canister: $RECIEVER"

# Check the balance of the Harshit identity before approval
echo "Checking balance before approval:"
dfx canister call ckbtc icrc1_balance_of "(record {owner = principal \"$DEFAULT\"})"

# Approve the valueswap_backend canister to transfer 10,000 tokens from the Harshit identity
echo "Approving valueswap_backend to transfer 10,000 tokens on behalf of Harshit"
APPROVE=$(
dfx --identity Harshit canister call ckbtc icrc2_approve "(record { amount = 510000; spender = record { owner = principal \"$CANISTER\"} })"
)
echo "Approval result: $APPROVE"

# Confirm allowance
echo "Checking allowance:"
ALLOWANCE=$(
dfx --identity Harshit canister call ckbtc icrc2_allowance "(record { account = record { owner = principal \"$DEFAULT\"}; spender = record { owner = principal \"$CANISTER\"} })"
)
echo "Allowance result: $ALLOWANCE"

# Transfer 10,000 tokens to the receiver canister using deposit_tokens
echo "Transferring 10,000 tokens to the receiver canister using deposit_tokens"
USER_TRANSFER=$(
dfx --identity Harshit canister call valueswap_backend deposit_tokens "(500000, principal \"$LEDGER_CANISTER_ID\", principal \"$RECIEVER\")"
)
echo "Deposit result: $USER_TRANSFER"

# Print balances after the transfer
echo "Checking balance after transfer:"
dfx canister call ckbtc icrc1_balance_of "(record {owner = principal \"$DEFAULT\"})"
dfx canister call ckbtc icrc1_balance_of "(record {owner = principal \"$RECIEVER\"})"
