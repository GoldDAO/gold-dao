#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

CID="s2ryu-oyaaa-aaaap-qhq2q-cai"
METHOD_NAME="manage_sns_neuron"
VALIDATE_METHOD_NAME="manage_sns_neuron_validate"

./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Register the method ${METHOD_NAME} of the canister ${CID}.
\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_010 : nat64);
                name = \"$METHOD_NAME.\";
                description = opt \"$METHOD_NAME\";
                function_type = opt variant {
                    GenericNervousSystemFunction = record {
                        validator_canister_id = opt principal \"$CID\";
                        target_canister_id = opt principal \"$CID\";
                        validator_method_name = opt \"$VALIDATE_METHOD_NAME\";
                        target_method_name = opt \"$METHOD_NAME\"
                    }
                }
            }
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
