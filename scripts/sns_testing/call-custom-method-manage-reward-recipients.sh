#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1003

export BLOB="$(didc encode --format blob "(record {
    list = vec {
        record {
            tag = \"SNS rewards canister\";
            reward_weight = 5000: nat16;
            account = record {
                owner = principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\";
                subaccount = null
            }
        };
        record {
            tag = \"Team wallet\";
            reward_weight = 5000: nat16;
            account = record {
                owner = principal \"lu74a-37wds-n53ie-rlaal-7jcu2-fksa5-4lzgc-4lkaa-xt2vr-hr3db-rqe\";
                subaccount = null
            }
        };
    }
})")"

./scripts/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Define the NNS maturity recipient addresses.\";
        url=\"https://example.com/\";
        summary=\"Define the wallets to receive the NNS neuron rewards.\";
        action= opt variant {
            ExecuteGenericNervousSystemFunction = record {
                function_id= ${FID}:nat64;
                payload = ${BLOB}
            }
        }
    }
)" > message.json


quill send message.json

rm tmp.pem && rm message.json && rm sns_canister_ids.json
