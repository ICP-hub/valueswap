#!/bin/bash

# Set variables
SOURCE_CANISTER="bkyz2-fmaaa-aaaaa-qaaaq-cai"
DESTINATION_CANISTER="by6od-j4aaa-aaaaa-qaadq-cai"
LEDGER_CANISTER="bw4dl-smaaa-aaaaa-qaacq-cai"
AMOUNT=1000000000  # Amount in e8s (1 token = 100000000 e8s)


# Perform the transfer
RESULT=$(dfx canister call $LEDGER_CANISTER icrc1_transfer "( 
  record {
    to = record {
      owner = principal \"$DESTINATION_CANISTER\";
      subaccount = null;
    };
    amount = $AMOUNT;
    fee = null;
    memo = null;
    from_subaccount = null;
    created_at_time = null;
  }
)")

# Check the result
if echo "$RESULT" | grep -q "Ok"; then
    echo "Transfer successful. Block index: $(echo "$RESULT" | sed -n 's/.*Ok = \([0-9]*\).*/\1/p')"
else
    echo "Transfer failed: $RESULT"
fi
