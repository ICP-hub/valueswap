#!/bin/bash

# Define the pool data in JSON format
POOL_DATA=$(cat <<EOF
{
  "pool_data": [
    {
      "balance": "100000000n",
      "image": "https://coin-images.coingecko.com/coins/images/33818/large/01_ckBTC_Token_HEX__4x.png?1703046026",
      "ledger_canister_id": "Principal({_arr: Uint8Array(10), _isPrincipal: true})",
      "token_name": "ckbtc",
      "value": "93741n",
      "weight": "50n"
    },
    {
      "balance": "2738106713n",
      "image": "https://coin-images.coingecko.com/coins/images/33819/large/01_ckETH_Token_HEX__4x.png?1703046389",
      "ledger_canister_id": "Principal({_arr: Uint8Array(10), _isPrincipal: true})",
      "token_name": "cketh",
      "value": "93741n",
      "weight": "50n"
    }
  ],
  "swap_fee": 30
}
EOF
)

# Print the data to verify
echo "Pool Data: $POOL_DATA"

# Replace `your_command_here` with the actual function or API call
# Example: Using `curl` to make a POST request to an API
API_ENDPOINT="http://localhost:3000/valueswap/pool/create-pool/steps"  # Replace with your actual endpoint

# Make the API call
curl -X POST "$API_ENDPOINT" \
     -H "Content-Type: application/json" \
     -d "$POOL_DATA"

# Or if you're calling a local CLI tool
# echo "$POOL_DATA" | your_cli_tool create_pool
