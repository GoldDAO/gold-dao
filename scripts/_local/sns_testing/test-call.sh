#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

NNS_NEURON_ID=3677999446070400472

FID=1001

export BLOB="$(didc encode --format blob "(record {
    by = record {
        id = vec { 7eac04f2e207c04b8a7ac01b7505c715821ebbd7bc9815cef0a6842514f3b832 }
    }
})")"

./scripts/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Start dissolving neuron.\";
        url=\"https://example.com/\";
        summary=\"Start dissolving neuron ${NNS_NEURON_ID}.\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json


quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
