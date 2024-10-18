#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

CID="rbv23-fqaaa-aaaam-qbfma-cai"
METHOD_NAME="set_reward_token_types"
VALIDATE_METHOD_NAME="set_reward_token_types_validate"

./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE \
    make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Register new method with SNS.\";
        url=\"https://example.com/\";
        summary=\"Register the method \`set_reward_token_types\` of the sns_rewards canister to the SNS.
This method allows to define the tokens in which rewards are paid to stakers.
\";
        action= opt variant {
            AddGenericNervousSystemFunction = record {
                id = (1_006 : nat64);
                name = \"Set reward token types.\";
                description = opt \"Proposal to define the tokens in which rewards are paid to GLD neurons.\";
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
