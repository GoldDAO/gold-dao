#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1005

export BLOB="$(didc encode --format blob "(record {
    token_list = vec {
      record {
        \"OGY\";
        record {
          fee = 200_000 : nat64;
          decimals = 8 : nat64;
          ledger_id = principal \"j5naj-nqaaa-aaaal-ajc7q-cai\";
        };
      };
    };
  }
)")"

./scripts/_local/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file ./sns_canister_ids.json --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Update reward tokens on rewards canister.\";
        url=\"https://example.com/\";
        summary=\"Updates the ledger id of the OGY token in the registered rewards definition of the rewards canister.
When this canister was launched, ORIGYN hadn't undergone their SNS yet and the ledger canister id was defined with the legacy canister.
Now that the rewards will come soon, the ledger ID has to be updated to the new ledger 'j5naj-nqaaa-aaaal-ajc7q-cai'.
\";
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
