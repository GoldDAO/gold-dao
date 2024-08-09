#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

NNS_NEURON_ID=16320996336187008175
HOTKEY_TO_ADD="j2neh-vqaaa-aaaal-aduxq-cai"

FID=1001

export BLOB="$(didc encode --format blob "(record {
    command = variant {
        Configure = record {
            operation = opt variant {
                AddHotKey = record {
                    new_hot_key = opt principal \"${HOTKEY_TO_ADD}\"
                }
            }
        }
    };
    neuron_id = ${NNS_NEURON_ID}:nat64
})")"

./scripts/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Add hotkey to neuron.\";
        url=\"https://example.com/\";
        summary=\"Add hotkey ${HOTKEY_TO_ADD} to neuron ${NNS_NEURON_ID}.\";
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
