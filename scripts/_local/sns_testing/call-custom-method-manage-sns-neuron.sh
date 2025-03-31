#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1010

export BLOB="$(didc encode --format blob "()")"

dfx identity export gitlab_ci_gldt_staging > tmp.pem

./scripts/_local/sns_testing/prepare_scripts.sh staging

[ -e message.json ] && rm message.json

export BLOB="$(didc encode --format blob "(
  record {
    neuron_type = variant { WTN };
    command = variant {
      Configure = record {
        operation = opt variant { StartDissolving = record {} };
      }
    };
    neuron_id = blob \"\fe\13\c2\a2\fa\29\a5\bf\d9\e4\69\31\de\25\03\c1\c3\a6\fe\23\1f\79\bd\80\ab\dd\0f\0a\18\9f\ec\87\";
  },
)")"


quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Manage WTN neuron.\";
        url=\"https://example.com/\";
        summary=\"Start dissolving WTN neuron.\";
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
