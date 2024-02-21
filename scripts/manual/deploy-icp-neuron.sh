#!/usr/bin/env bash

./scripts/build-canister.sh icp_neuron &&
./scripts/generate-did.sh icp_neuron &&
# dfx deploy --network staging icp_neuron --argument '(record {test_mode = true; rewards_recipients = vec {
#   record { tag = "Team"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
#   record { tag = "GLDGov voters"; reward_weight = 3300; account = record { owner = principal "2f5ll-gqaaa-aaaak-qcfuq-cai"; subaccount = null } };
#   record { tag = "Listing funds"; reward_weight = 3300; account = record { owner = principal "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe"; subaccount = null } };
#   record { tag = "Good DAO"; reward_weight = 100; account = record { owner = principal "lu74a-37wds-n53ie-rlaal-7jcu2-fksa5-4lzgc-4lkaa-xt2vr-hr3db-rqe"; subaccount = null } }
# }})' --mode reinstall
dfx deploy --network staging icp_neuron --argument '(record {test_mode = true; rewards_recipients = vec {
  record { tag = "Team"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "GLDGov voters"; reward_weight = 3300; account = record { owner = principal "2f5ll-gqaaa-aaaak-qcfuq-cai"; subaccount = null } };
  record { tag = "Listing funds"; reward_weight = 3300; account = record { owner = principal "thrhh-hnmzu-kjquw-6ebmf-vdhed-yf2ry-avwy7-2jrrm-byg34-zoqaz-wqe"; subaccount = null } };
  record { tag = "Good DAO"; reward_weight = 100; account = record { owner = principal "lu74a-37wds-n53ie-rlaal-7jcu2-fksa5-4lzgc-4lkaa-xt2vr-hr3db-rqe"; subaccount = null } }
}})'
