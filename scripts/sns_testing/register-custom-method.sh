#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

./scripts/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Adding the set_reserve_transfer_amounts method to allow to set the daily tokens sent to the reward pool as governance rewards.\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_004 : nat64);
                name = \"Set GLDGov reserve transfer amount.\";
                description = opt \"Proposal to update the daily reserve transfer amount of GLDGov that defines the reward rate of Gold DAO voters.\";
                function_type = opt variant {
                    GenericNervousSystemFunction = record {
                        validator_canister_id = opt principal \"2f5ll-gqaaa-aaaak-qcfuq-cai\";
                        target_canister_id = opt principal \"2f5ll-gqaaa-aaaak-qcfuq-cai\";
                        validator_method_name = opt \"set_reserve_transfer_amounts_validate\";
                        target_method_name = opt \"set_reserve_transfer_amounts\"
                    }
                }
            }
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
