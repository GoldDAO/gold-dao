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
echo "Deploying icp_neuron on ${NETWORK}"
. scripts/deploy-icp-neuron.sh $NETWORK
echo "Deploying token_metrics on ${NETWORK}"
. scripts/deploy-token-metrics.sh $NETWORK
echo "Deploying sns_rewards on ${NETWORK}"
. scripts/deploy-sns-rewards.sh $NETWORK
echo "Deploying management on ${NETWORK}"
. scripts/deploy-management.sh $NETWORK
echo "Deploying frontends on ${NETWORK}"
dfx deploy --network ${NETWORK} --compute-evidence gldt_landing_page
dfx deploy --network ${NETWORK} --compute-evidence gldt_swap_app
dfx deploy --network ${NETWORK} --compute-evidence gldt_explorer
dfx deploy --network ${NETWORK} --compute-evidence gld_dashboard
dfx deploy --network ${NETWORK} gldt_landing_page
dfx deploy --network ${NETWORK} gldt_swap_app
dfx deploy --network ${NETWORK} gldt_explorer
dfx deploy --network ${NETWORK} gld_dashboard
