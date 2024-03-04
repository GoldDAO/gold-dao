#!/usr/bin/env bash

./scripts/build-canister.sh icp_neuron &&
./scripts/generate-did.sh icp_neuron &&
./scripts/build-canister.sh icp_neuron &&

dfx deploy --network staging icp_neuron --argument '(opt record {test_mode = true; rewards_recipients = vec {
  record { tag = "Team development"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "GLDGov voters"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "Listing funds"; reward_weight = 3300; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } };
  record { tag = "Good DAO"; reward_weight = 100; account = record { owner = principal "465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae"; subaccount = null } }
}})' --mode reinstall
# dfx deploy --network staging icp_neuron --argument '(null)'
