#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1001

NNS_NEURON_ID=17481076647658761488

export BLOB="$(didc encode --format blob "(record {
    command = variant {
        RefreshVotingPower = record {}
    };
    neuron_id = ${NNS_NEURON_ID}:nat64
})")"

./scripts/_local/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Refresh voting power of neuron ${NNS_NEURON_ID}.\";
        url=\"https://example.com/\";
        summary=\"Refresh voting power of neuron ${NNS_NEURON_ID}.\";
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
