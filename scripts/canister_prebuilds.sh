#!/usr/bin/env bash

CANISTER=$1
BASE_CANISTER_PATH=$2
INTTEST=$3  # $4 should be changed to $3 since it's the third argument passed

echo "Running prebuild scripts for $CANISTER"
echo "Flags passed $INTTEST"
