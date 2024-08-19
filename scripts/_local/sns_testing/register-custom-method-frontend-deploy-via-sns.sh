#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

CID="mpece-iqaaa-aaaal-qizma-cai"
METHOD_NAME="commit_proposed_batch"
VALIDATE_METHOD_NAME="validate_commit_proposed_batch"

./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Adding the methods to upgrade GLD dashboard staging via SNS.\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (2_001 : nat64);
                name = \"Upgrade GLD dashboard staging frontend.\";
                description = opt \"Proposal to upgrade the GLD dashboard staging frontend\";
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
