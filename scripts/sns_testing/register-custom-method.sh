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
        summary=\"Adding the stake_nns_neuron method to the SNS to be able to stake ICP NNS neurons via the Gold DAO SNS.\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_002 : nat64);
                name = \"Stake NNS neuron\";
                description = \"Proposal to stake an ICP neuron under control of the Gold DAO.\";
                function_type = opt variant {
                    GenericNervousSystemFunction = record {
                        validator_canister_id = opt principal \"j2neh-vqaaa-aaaal-aduxq-cai\";
                        target_canister_id = opt principal \"j2neh-vqaaa-aaaal-aduxq-cai\";
                        validator_method_name = \"stake_nns_neuron_validate\";
                        target_method_name = \"stake_nns_neuron\"
                    }
                }
            }
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json && rm sns_canister_ids.json
