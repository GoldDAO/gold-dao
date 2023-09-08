#!/usr/bin/env bash

# this only works with ic-cdk version 0.10.0 or higher. Due to crate compatibility issues with candid, I had to
# downgrade and this is not working at the moment (Dustin, 24.08.23)

# call this from the root directly of gldt_swap project like `./scripts/generate-did.sh`

function generate_did() {
  local canister=$1
  did_path="canister/$canister/src"

  cargo build --target wasm32-unknown-unknown \
      --release --package "$canister" \
      --features "ic-cdk/wasi"

  # Installation https://docs.wasmtime.dev/cli-install.html
  wasmtime "target/wasm32-unknown-unknown/release/$canister.wasm" > "$did_path/$canister.did"
  dfx generate $canister
}

# The list of canisters of your project, comma separated, e.g. CANISTERS=gldt_core,gldt_ledger,...
CANISTERS=gldt_core

for canister in $(echo $CANISTERS | sed "s/,/ /g")
do
    generate_did "$canister"
done
