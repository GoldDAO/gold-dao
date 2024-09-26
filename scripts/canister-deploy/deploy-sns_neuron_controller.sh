#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE=true
  SNS_REWARDS_CANISTER_ID=2f5ll-gqaaa-aaaak-qcfuq-cai
  OGY_SNS_GOVERNANCE_CANISTER_ID=jtpnb-waaaa-aaaal-ajc6q-cai
  OGY_SNS_LEDGER_CANISTER_ID=j5naj-nqaaa-aaaal-ajc7q-cai
  OGY_SNS_REWARDS_CANISTER_ID=fpmqz-aaaaa-aaaag-qjvua-cai
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE=false
  SNS_REWARDS_CANISTER_ID=iyehc-lqaaa-aaaap-ab25a-cai
  OGY_SNS_GOVERNANCE_CANISTER_ID=lnxxh-yaaaa-aaaaq-aadha-cai
  OGY_SNS_LEDGER_CANISTER_ID=lkwrt-vyaaa-aaaaq-aadhq-cai
  OGY_SNS_REWARDS_CANISTER_ID=yuijc-oiaaa-aaaap-ahezq-cai
  AUTHORIZED_PRINCIPAL=tr3th-kiaaa-aaaaq-aab6q-cai
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

# ARGUMENTS="(
#   variant {
#     Init = record {
#       test_mode = $TESTMODE;
#       version = record {
#         major = 0 : nat32;
#         minor = 0 : nat32;
#         patch = 0 : nat32;
#       };
#       commit_hash = \"$COMMIT_HASH\";
#       authorized_principals = vec {
#         principal \"$AUTHORIZED_PRINCIPAL\";
#       };
#       sns_rewards_canister_id = principal \"$SNS_REWARDS_CANISTER_ID\";
#       ogy_sns_governance_canister_id = principal \"$OGY_SNS_GOVERNANCE_CANISTER_ID\";
#       ogy_sns_ledger_canister_id = principal \"$OGY_SNS_LEDGER_CANISTER_ID\";
#       ogy_sns_rewards_canister_id = principal \"$OGY_SNS_REWARDS_CANISTER_ID\";
#     }
#   }
# )"

  ARGUMENTS="(
    variant {
      Upgrade = record {
        version = record {
          major = 1 : nat32;
          minor = 0 : nat32;
          patch = 0 : nat32;
        };
        commit_hash = \"$COMMIT_HASH\";
      }
    }
  )"

. ./scripts/deploy_backend_canister.sh sns_neuron_controller $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA
