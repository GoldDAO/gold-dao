#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns  --canister-ids-file $CANISTER_IDS --pem-file $PEM_FILE  make-proposal $DEVELOPER_NEURON_ID --proposal '(
    record {
        title = "Test title";
        url = "https://gold-dao.org";
        summary = "Test summary.";
        action = opt variant {
            Motion = record {
                motion_text = "Test text"
            }
        };
    }
)' > message.json

quill send message.json -y

rm tmp.pem && rm message.json
