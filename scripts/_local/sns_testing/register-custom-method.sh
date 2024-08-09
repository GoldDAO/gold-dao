#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

CID="j2neh-vqaaa-aaaal-aduxq-cai"
METHOD_NAME="manage_reward_recipients"
VALIDATE_METHOD_NAME="manage_reward_recipients_validate"

./scripts/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Adding the manage_reward_recipients method of the icp_neuron canister to the SNS to be able to manage the reward recipients of the neurons' maturities. As described in proposal #5, the rewards of the neurons are split in 4 parts - 33% to GLDGov stakers, 33% to development team, 33% for listings and 1% for the Good DAO. This method allows to set these recipients.\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_002 : nat64);
                name = \"Manage NNS neuron maturity recipients.\";
                description = opt \"Proposal to manage the recipients of the disbursed maturity of the NNS neurons.\";
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
