#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

CID="j2neh-vqaaa-aaaal-aduxq-cai"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

./scripts/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Adding the manage_reward_recipients method to allow to set the accounts to receive the maturity from the NNS neurons.\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_003 : nat64);
                name = \"Manage NNS neuron reward recipients\";
                description = opt \"Proposal to update the recipients of the NNS neuron maturity.\";
                function_type = opt variant {
                    GenericNervousSystemFunction = record {
                        validator_canister_id = opt principal \"j2neh-vqaaa-aaaal-aduxq-cai\";
                        target_canister_id = opt principal \"j2neh-vqaaa-aaaal-aduxq-cai\";
                        validator_method_name = opt \"manage_reward_recipients_validate\";
                        target_method_name = opt \"manage_reward_recipients\"
                    }
                }
            }
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
