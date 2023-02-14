# deploy dao

near deploy \
    --wasmFile ./out/registry.wasm \
    --initFunction "new_default_meta" \
    --initArgs '{
        "owner_id": "navara.testnet"
    }' \
    --accountId nns.navara.testnet