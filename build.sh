#!/bin/bash
set -e

cd navara-name-service-resolver

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ../out
cp target/wasm32-unknown-unknown/release/*.wasm ../out/resolver.wasm

cd ../navara-name-service-registry

RUSTFLAGS='-C link-arg=-s' cargo build --target wasm32-unknown-unknown --release
mkdir -p ../out
cp target/wasm32-unknown-unknown/release/*.wasm ../out/registry.wasm