# deploy dao

near deploy \
    --wasmFile ./out/registry.wasm \
    --initFunction "migrate" \
    --initArgs '{
        "owner_id": "michaelng.testnet"
    }' \
    --accountId navara.testnet