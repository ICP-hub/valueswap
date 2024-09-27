#!/bin/bash

# List of identities
identities=("user1" "user2" "user3" "user4")

# Canister ID of the ICRC-1 ledger
LEDGER_CANISTER_ID="icrc1_ledger_canister"

echo "Checking balances for all identities..."

for identity in "${identities[@]}"; do
  echo "Switching to identity: $identity"
  dfx identity use $identity

  principal=$(dfx identity get-principal)
  echo "Principal: $principal"

  balance=$(dfx canister call $LEDGER_CANISTER_ID icrc1_balance_of "(record { owner = principal \"$principal\" })")
  echo "Balance for $identity ($principal): $balance"
  echo "----------------------------------------"
done

# Switch back to the default identity
dfx identity use default