#!/bin/bash

PEM_FILE="tmp.pem"
DEVELOPER_NEURON_ID="2c21f2deae7502b97d63bf871381e0fdde5c9c68d499344eb2231d109bb9ffc9"
CANISTER_IDS="sns_canister_ids.json"

FID=1002

REWARDS_WALLET="iyehc-lqaaa-aaaap-ab25a-cai"
LIQUIDITY_WALLET="5aybl-v7aii-duvsu-ztemq-litdi-ly42r-iyf35-2k46p-ovynj-amtow-rae"
BUYBACK_WALLET="p7m6l-xwkr6-y46va-zm7ap-dv36u-de6cr-afpgn-fjouf-x7fkk-kftcc-4qe"
GOODDAO_WALLET="w4buy-lgwzr-pccs7-huzhh-qqnws-rns75-iaoox-jolrm-xs2ra-vdu3o-2qe"

export BLOB="$(didc encode --format blob "(record {
    list = vec {
        record {
            tag = \"Gold DAO rewards canister\";
            reward_weight = 3300: nat16;
            account = record {
                owner = principal \"$REWARDS_WALLET\";
                subaccount = null
            }
        };
        record {
            tag = \"Gold DAO liquidity wallet\";
            reward_weight = 3300: nat16;
            account = record {
                owner = principal \"$LIQUIDITY_WALLET\";
                subaccount = null
            }
        };
        record {
            tag = \"Gold DAO buyback and burn wallet\";
            reward_weight = 3300: nat16;
            account = record {
                owner = principal \"$BUYBACK_WALLET\";
                subaccount = null
            }
        };
        record {
            tag = \"The Good DAO wallet\";
            reward_weight = 100: nat16;
            account = record {
                owner = principal \"$GOODDAO_WALLET\";
                subaccount = null
            }
        }
    }
})")"

./scripts/sns_testing/prepare_scripts.sh staging

dfx identity export gitlab_ci_gldt_staging > tmp.pem

[ -e message.json ] && rm message.json

quill sns --canister-ids-file $CANISTER_IDS --pem-file $PEM_FILE make-proposal $DEVELOPER_NEURON_ID --proposal "(
    record {
        title=\"Set reward recipients on the icp_neuron canister.\";
        url=\"https://gold-dao.org/\";
        summary=\"The Gold DAO holds seven NNS neurons. Every time a neuron reaches 1'000 ICP in maturity, a new neuron is spawned of this maturity and disbursed after 1 week to different recipients. This proposal sets the recipients of these rewards. As described in proposal 5 and 26, the recipients are as follows

* 33% to the Gold DAO governance participants (Principal $REWARDS_WALLET, [View on ICP explorer](https://dashboard.internetcomputer.org/account/6dc2515bbb9b0a97b8d977ebac3eba643a1fb4b6da8b33455e0dba957f0ce7da))
* 33% to a wallet for liquidity providing (Principal $LIQUIDITY_WALLET, [View on ICP explorer](https://dashboard.internetcomputer.org/account/4fe98a124c29830fc6aca41f07326d0a917888478d4d22f1f30ed4612fc067dd))
* (new) 33% to a wallet for buying back & burning GLDGov (Principal $BUYBACK_WALLET, [View on ICP explorer](https://dashboard.internetcomputer.org/account/45c5a2744b041dc5101b30dd21979c76b1c24de36dcfd6080c7d5fe1b47aa3d8))
* 1% to the Good DAO for humanitarian causes (Principal $GOODDAO_WALLET, [View on ICP explorer](https://dashboard.internetcomputer.org/account/6826718e36a82485cfd2897c3b246980033a920ec1c562865d491e05a7dc7549))
\";
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
