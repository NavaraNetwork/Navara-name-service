# deploy dao

near deploy \
    --wasmFile ./out/registry.wasm \
    --initFunction "new" \
    --initArgs '{
        "metadata": {
            "spec": "nft-1.0.0",
            "name": "Navara name service",
            "symbol": "NNS"          
        },
        "owner_id": "navara.testnet"
    }' \
    --accountId dev_nns.testnet