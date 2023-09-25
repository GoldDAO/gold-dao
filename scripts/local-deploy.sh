#!/usr/bin/env bash

. scripts/pre-deploy.sh local && \
echo "Deploying gldt-ledger on ${NETWORK}" && \
. scripts/deploy-ledger.sh $NETWORK && \
echo "Deploying gldt-core on ${NETWORK}" && \
. scripts/deploy-gldt-core.sh $NETWORK
