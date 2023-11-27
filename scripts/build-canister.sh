#!/usr/bin/env bash

show_help() {
  cat << EOF
gldt_core canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

The canister will always be reinstalled locally, and only upgraded in staging and production (ic).

Usage:
  scripts/deploy-gldt-fee-compensation.sh [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
  -w, --wasmonly		Only produce a non-optimized wasm file (used for did generation)
EOF
}

if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      -w | --wasmonly )
        WASMONLY=1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

cargo build --target wasm32-unknown-unknown --target-dir canister/$1/target --release --locked -p $1

if [[ $WASMONLY == 1 ]]; then
	echo "$1 wasm file created and read for did generation"
else
	ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm shrink
	ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm optimize --inline-functions-with-loops O3
	gzip -9 -v -c canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm >canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	gzip -v -t canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	echo "$1 successfully built, optimized and compressed"
fi
