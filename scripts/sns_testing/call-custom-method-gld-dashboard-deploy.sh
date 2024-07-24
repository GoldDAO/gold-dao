#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=2001

BATCH_ID=4
EVIDENCE_RAW="e213a273712d9e0113092c873cb354aa5d0e1fa9033d7690b8513458e400b95f"

EVIDENCE_BLOB=$(echo "$EVIDENCE_RAW" | sed 's/../\\&/g')

export BLOB="$(didc encode --format blob "(record {
    batch_id = $BATCH_ID : nat;
    evidence = blob \"$EVIDENCE_BLOB\"
})")"

./scripts/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Update neuron's dissolve delay.\";
        url=\"https://example.com/\";
        summary=\"Adding ${ADDITIONAL_SECONDS} seconds to dissolve delay of neuron ${NEURON_ID}.\";
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
