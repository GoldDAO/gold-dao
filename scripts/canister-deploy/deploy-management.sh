#!/usr/bin/env bash

NETWORK=$1
DEPLOYMENT_VIA="direct"

if [[ $NETWORK =~ ^(local|staging)$ ]]; then
  TESTMODE=true
  AUTHORIZED_PRINCIPAL=465sx-szz6o-idcax-nrjhv-hprrp-qqx5e-7mqwr-wadib-uo7ap-lofbe-dae
elif [[ $NETWORK =~ ^(ic)$ ]]; then
  TESTMODE=false
  AUTHORIZED_PRINCIPAL=2we4k-xim55-asne3-m7o22-fliz6-lmu6q-5pwc5-evfit-4scxr-itg7g-xae
else
  echo "Error: unknown network for deployment. Found $NETWORK."
  exit 2
fi

ARGUMENTS="(record {
  test_mode = $TESTMODE;
  authorized_principals = vec { principal \"$AUTHORIZED_PRINCIPAL\"; };
})"

. ./scripts/deploy_backend_canister.sh management $NETWORK "$ARGUMENTS" $DEPLOYMENT_VIA
