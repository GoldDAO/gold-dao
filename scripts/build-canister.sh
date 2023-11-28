#!/usr/bin/env bash

show_help() {
  cat << EOF
Canister build and did generation script.
Must be run from the repository's root folder.

Usage:
  scripts/build-canister.sh [options] <CANISTER>

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
  echo "Error: missing <CANISTER> argument"
  exit 1
fi

if [[ $WASMONLY == 1 ]]; then
  echo "" > canister/$1/src/${1}.did
fi

cargo build --target wasm32-unknown-unknown --target-dir canister/$1/target --release --locked -p $1

if [[ $WASMONLY == 1 ]]; then
  rm canister/$1/src/${1}.did
	echo "$1 wasm file created and read for did generation"
else
	ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm shrink
	ic-wasm canister/$1/target/wasm32-unknown-unknown/release/$1.wasm -o canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm optimize --inline-functions-with-loops O3
	gzip -9 -v -c canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm >canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	gzip -v -t canister/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	echo "$1 successfully built, optimized and compressed"
fi
