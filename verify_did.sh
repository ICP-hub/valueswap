#!/bin/bash

# Define paths
UPDATED_DID_PATH="src/swap/swap.did"
REFERENCE_DID_PATH="src/swap/reference_swap.did"

# Compare the updated .did file with the reference file
diff "$UPDATED_DID_PATH" "$REFERENCE_DID_PATH"

# Check the exit status of the diff command
if [ $? -eq 0 ]; then
    echo "The .did file is updated correctly."
else
    echo "The .did file differs from the reference file."
fi