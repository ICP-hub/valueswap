#!/bin/bash

# Exit immediately if a command exits with a non-zero status
set -e

# Set the source canister and receiver details
SOURCE_CANISTER="be2us-64aaa-aaaaa-qaabq-cai"
RECEIVER_PRINCIPAL="rgtib-ktq4g-rjiya-aishg-mv2om-ey54m-jhyut-2tuaf-tvjyz-ml54r-lae"

# Set the ledger canister ID (replace this with your actual ledger canister ID)
LEDGER_CANISTER_ID=$(dfx canister id ckbtc)

# Set the amount to transfer (10,000 tokens)
AMOUNT=10000000  # Adjust the amount as per ledger decimals

# Check balance before the transfer for both accounts
echo "Checking balance of source canister ($SOURCE_CANISTER) before transfer:"
dfx canister call $LEDGER_CANISTER_ID icrc1_balance_of "(record { owner = principal \"$SOURCE_CANISTER\"; subaccount = null })"

echo "Checking balance of receiver ($RECEIVER_PRINCIPAL) before transfer:"
dfx canister call $LEDGER_CANISTER_ID icrc1_balance_of "(record { owner = principal \"$RECEIVER_PRINCIPAL\"; subaccount = null })"

# Perform the transfer from the source canister to the receiver
echo "Initiating transfer of 10,000 tokens from $SOURCE_CANISTER to $RECEIVER_PRINCIPAL"
TRANSFER_RESULT=$(dfx canister call $LEDGER_CANISTER_ID icrc1_transfer "(record {
  from = record { owner = principal \"$SOURCE_CANISTER\"; subaccount = null };
  to = record { owner = principal \"$RECEIVER_PRINCIPAL\"; subaccount = null };
  amount = $AMOUNT;
  fee = null;
  memo = null;
  created_at_time = null;
})")

# Output the result of the transfer
echo "Transfer result: $TRANSFER_RESULT"

# Check balance after the transfer for both accounts
echo "Checking balance of source canister ($SOURCE_CANISTER) after transfer:"
dfx canister call $LEDGER_CANISTER_ID icrc1_balance_of "(record { owner = principal \"$SOURCE_CANISTER\"; subaccount = null })"

echo "Checking balance of receiver ($RECEIVER_PRINCIPAL) after transfer:"
dfx canister call $LEDGER_CANISTER_ID icrc1_balance_of "(record { owner = principal \"$RECEIVER_PRINCIPAL\"; subaccount = null })"