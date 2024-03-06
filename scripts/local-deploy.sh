#!/usr/bin/env bash

. scripts/pre-deploy.sh --network ${NETWORK} && \
if [[ $? != 0 ]]; then
	exit 1;
fi

echo "Deploying gldt_ledger on ${NETWORK}"
. scripts/deploy-ledger.sh $NETWORK && \
echo "Deploying gldt_ledger_indexer on ${NETWORK}" && \
. scripts/deploy-ledger-indexer.sh $NETWORK
echo "Deploying gldt_core on ${NETWORK}"
. scripts/deploy-gldt-core.sh $NETWORK
echo "Deploying gldt_fee_compensation on ${NETWORK}"
. scripts/deploy-gldt-fee-compensation.sh $NETWORK
echo "Deploying frontends on ${NETWORK}"
dfx deploy --network ${NETWORK} --compute-evidence gldt_landing_page
dfx deploy --network ${NETWORK} --compute-evidence gldt_swap_app
dfx deploy --network ${NETWORK} --compute-evidence gldt_explorer
dfx deploy --network ${NETWORK} gldt_landing_page
dfx deploy --network ${NETWORK} gldt_swap_app
dfx deploy --network ${NETWORK} gldt_explorer
