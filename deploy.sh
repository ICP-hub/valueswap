cargo build --release --target wasm32-unknown-unknown --package valueswap_backend

candid-extractor target/wasm32-unknown-unknown/release/valueswap_backend.wasm > src/valueswap_backend/valueswap_backend.did

# # # dfx deploy

cargo build --release --target wasm32-unknown-unknown --package swap

candid-extractor target/wasm32-unknown-unknown/release/swap.wasm > src/swap/swap.did


# #!/bin/bash

# # Build the project
# cargo build --release --target wasm32-unknown-unknown --package swap

# # Extract the Candid interface
# cargo test --package swap -- tests::export_candid --nocapture