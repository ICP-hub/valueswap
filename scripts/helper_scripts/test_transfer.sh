#!/bin/bash

# Replace these values with actual values
CANISTER_NAME="valueswap_backend"  # Replace with your canister name
USER_PRINCIPAL="rgtib-ktq4g-rjiya-aishg-mv2om-ey54m-jhyut-2tuaf-tvjyz-ml54r-lae"  # Replace with the target user principal
AMOUNT="100000000"  # Replace with the transfer amount

# Call the icrc1_transfer function
dfx canister call $CANISTER_NAME icrc1_transfer "(
  principal \"$USER_PRINCIPAL\", 
  $AMOUNT : nat
)"
