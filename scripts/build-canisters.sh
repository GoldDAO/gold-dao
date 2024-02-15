#!/usr/bin/env bash

show_help() {
  cat << EOF
Wrapper script to build all project canisters.

Usage:
  scripts/build-canisters.sh [options]

Options:
  -h, --help        Show this message and exit
EOF
}


$PROJECT_CANISTERS=("gldt_core" "gldt_fee_compensation" "sns_rewards")

for canister in "${PROJECT_CANISTERS[@]}"
do
  ./build-canister.sh "$canister"
done
