cargo build --release --target wasm32-unknown-unknown --package valueswap_backend

candid-extractor target/wasm32-unknown-unknown/release/valueswap_backend.wasm >src/valueswap_backend/valueswap_backend.did

# cargo build --release --target wasm32-unknown-unknown --package daohouse_backend

# candid-extractor target/wasm32-unknown-unknown/release/daohouse_backend.wasm >src/daohouse_backend/daohouse_backend.did

# cargo build --release --target wasm32-unknown-unknown --package ic_asset_handler

# candid-extractor target/wasm32-unknown-unknown/release/ic_asset_handler.wasm >src/ic_asset_handler/ic_asset_handler.did

