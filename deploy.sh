# deploy dao

near deploy \
    --wasmFile ./out/registry.wasm \
    --initFunction "new_default_meta" \
    --initArgs '{
        "owner_id": "michaelng.testnet"
    }' \
    --accountId navara.testnet