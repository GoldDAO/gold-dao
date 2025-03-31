#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1009

export BLOB="$(didc encode --format blob "()")"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

export BLOB="$(didc encode --format blob "(
  record {
    neuron_type = variant { WTN };
    amount = 10_100_000_000 : nat64;
    add_disolve_delay = opt (60 : nat32);
  },
)")"


quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Stake new WTN neuron.\";
        url=\"https://example.com/\";
        summary=\"Staking 101 WTN in SNS neuron to initiate neuron.\";
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
