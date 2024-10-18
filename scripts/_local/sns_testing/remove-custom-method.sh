#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem
./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Remove method from SNS.\";
        url=\"https://example.com/\";
        summary=\"Remove method 1005 from SNS.
\";
        action= opt variant {
            RemoveGenericNervousSystemFunction = 1_005:nat64
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
