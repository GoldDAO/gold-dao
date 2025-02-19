#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1001

NNS_NEURON_ID=8820325433984509404
NEURON_TO_FOLLOW=7577629725324745012

export BLOB="$(didc encode --format blob "(record {
    command = variant {
        Follow = record {
            topic = 4 : int32;
            followees = vec {
                record { id = $NEURON_TO_FOLLOW : nat64 };
            };
        }
    };
    neuron_id = ${NNS_NEURON_ID}:nat64
})")"

./scripts/_local/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file $CANISTER_IDS --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Adjust NNS neurons followee.\";
        url=\"https://gold-dao.org/\";
        summary=\"Adjust followee of neuron ${NNS_NEURON_ID}.\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json

quill send message.json

rm tmp.pem && rm message.json && rm sns_canister_ids.json
