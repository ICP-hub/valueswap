#!/bin/bash

set -e

# Create and use the DevJourney identity
# dfx identity new DevJourney || true
# dfx identity use DevJourney

export MINTER="iuem3-hyaaa-aaaac-aadaq-cai"
echo "MINTER principal (backend canister): $MINTER"

# Get the principal ID for the minter account
echo "MINTER principal: $MINTER"

# Set token details
export TOKEN_NAME="LP"
export TOKEN_SYMBOL="LP"
echo "Token Name: $TOKEN_NAME"
echo "Token Symbol: $TOKEN_SYMBOL"

# Set initial parameters
export PRE_MINTED_TOKENS=10_000_000_000
export TRANSFER_FEE=10_000

# Switch to the default identity and get its principal ID
dfx identity use DevJourney --network ic
export DEFAULT=$(dfx identity get-principal --network ic)
echo "DEFAULT principal: $DEFAULT"

# Set archive controller as the default identity for now
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

# Set archive options
export TRIGGER_THRESHOLD=2000
export NUM_OF_BLOCK_TO_ARCHIVE=1000
export CYCLE_FOR_ARCHIVE_CREATION=10000000000000
export FEATURE_FLAGS=true

# Deploy the cketh canister with the specified initialization arguments
DEPLOY_ARGUMENTS="(variant {Init = record {
  token_symbol = \"${TOKEN_SYMBOL}\";
  token_name = \"${TOKEN_NAME}\";
  minting_account = record { owner = principal \"${MINTER}\" };
  transfer_fee = ${TRANSFER_FEE};
  metadata = vec {};
  feature_flags = opt record{icrc2 = ${FEATURE_FLAGS}};
  initial_balances = vec { record { record { owner = principal \"${DEFAULT}\"; }; ${PRE_MINTED_TOKENS}; }; };
  archive_options = record {
    num_blocks_to_archive = ${NUM_OF_BLOCK_TO_ARCHIVE};
    trigger_threshold = ${TRIGGER_THRESHOLD};
    controller_id = principal \"${ARCHIVE_CONTROLLER}\";
    cycles_for_archive_creation = opt ${CYCLE_FOR_ARCHIVE_CREATION};
  };
}})"
echo "Deploy arguments: $DEPLOY_ARGUMENTS"

<<<<<<< HEAD
dfx deploy LP_ledger_canister --argument "$DEPLOY_ARGUMENTS" --network ic --no-wallet
=======
dfx deploy LP_ledger_canister --argument "$DEPLOY_ARGUMENTS" --network ic
>>>>>>> 26ad3fc (add faucet)

echo "LP ledger got deployed"

# Check the balance of the default identity
balance=$(dfx canister call LP_ledger_canister icrc1_balance_of "(record {owner=principal\"${DEFAULT}\"; subaccount=null})")
echo "Balance of the DEFAULT account: $balance"

echo "Approved backend canister to transfer tokens on behalf of the DEFAULT account"
