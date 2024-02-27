#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1001
ADDITIONAL_SECONDS=$((3600*24*365*1))

export BLOB="$(didc encode --format blob "(record {
    command = variant {
        Configure = record {
            operation = opt variant {
                IncreaseDissolveDelay = record {
                    additional_dissolve_delay_seconds = ${ADDITIONAL_SECONDS}:nat32
                }
            }
        }
    };
    neuron_id = 17481076647658761488:nat64
})")"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Update neuron's dissolve delay.\";
        url=\"https://example.com/\";
        summary=\"Adjusting the dissolve delay of one neuron to 6 months.\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json

quill send message.json -y

rm tmp.pem && rm message.json
