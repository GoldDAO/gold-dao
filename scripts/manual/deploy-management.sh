#!/usr/bin/env bash

./scripts/build-canister.sh management &&
./scripts/generate-did.sh management &&
./scripts/build-canister.sh management &&
# dfx deploy --network staging management --argument '(record {test_mode = true})' --mode reinstall
dfx deploy --network staging management --argument "(record {
  test_mode = true;
 authorized_principals = vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\" };
})" --mode reinstall


 # authorized_principals = vec { principal \"465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae\" };
# 