#!/usr/bin/env bash

. scripts/pre-deploy.sh local && \
echo "Deploying gldt_ledger on ${NETWORK}" && \
. scripts/deploy-ledger.sh $NETWORK && \
echo "Deploying gldt_ledger_indexer on ${NETWORK}" && \
. scripts/deploy-ledger-indexer.sh $NETWORK && \
echo "Deploying gldt_fee_compensation on ${NETWORK}" && \
. scripts/deploy-gldt-fee-compensation.sh $NETWORK && \
echo "Deploying gldt_core on ${NETWORK}" && \
. scripts/deploy-gldt-core.sh $NETWORK
