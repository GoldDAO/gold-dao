#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

CID="j2neh-vqaaa-aaaal-aduxq-cai"
WASM_PATH="backend/canisters/icp_neuron/target/wasm32-unknown-unknown/release/icp_neuron_canister.wasm.gz"
UPGRADE_ARGS="(null)"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

./scripts/sns_testing/prepare_scripts.sh staging

quill sns \
    --canister-ids-file $CANISTER_IDS \
    --pem-file $PEM_FILE  \
    make-upgrade-canister-proposal \
    --target-canister-id $CID \
    --wasm-path $WASM_PATH \
    --canister-upgrade-arg $UPGRADE_ARGS \
    $DEVELOPER_NEURON_ID \
    > message.json


quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
