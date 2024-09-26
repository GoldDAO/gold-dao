#!/usr/bin/env bash

show_help() {
  cat << EOF
Canister build and did generation script.
Must be run from the repository's root folder.

Usage:
  scripts/build_canister.sh [options] <CANISTER>

Options:
  -h, --help        Show this message and exit
  -w, --wasmonly    Only produce a non-optimized wasm file (used for did generation)
  --checksum        Compute sha256 evidence
  --verify          Verify sha256 evidence of existing build
EOF
}

BASE_CANISTER_PATH="backend/canisters"

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
      -it | --integration-test )
        echo "Building for integration testing"
        INTTEST="--features inttest"
        ;;
      --checksum )
        EVIDENCE=1
        ;;
      --verify )
        VERIFY=1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <CANISTER> argument"
  exit 1
fi

CHECKSUM_PATH=$BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/$1.checksum

if [[ -v VERIFY ]]; then
  sha256sum -c $CHECKSUM_PATH
  exit $?
fi

if [[ $WASMONLY == 1 ]]; then
  echo "" > $BASE_CANISTER_PATH/$1/api/can.did
fi

./scripts/canister_prebuilds.sh $1 $BASE_CANISTER_PATH $INTTEST

cargo build --target wasm32-unknown-unknown --target-dir $BASE_CANISTER_PATH/$1/target --release --locked $INTTEST -p $1

if [[ -v $WASMONLY ]]; then
  rm -f $BASE_CANISTER_PATH/$1/api/can.did
	echo "$1 wasm file created and ready for did generation"
else
	ic-wasm $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/$1.wasm -o $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}.wasm shrink
	ic-wasm $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/$1.wasm -o $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm optimize --inline-functions-with-loops O3
	gzip --no-name -9 -v -c $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm > $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	gzip -v -t $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz &&
	echo "$1 successfully built, optimized and compressed"
fi


if [[ -v $EVIDENCE ]]; then
  SUM=$(sha256sum $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/$1.wasm)
  echo $SUM
  echo $SUM > $CHECKSUM_PATH
  if [[ -z $WASMONLY ]]; then
    SUM=$(sha256sum $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm)
    echo $SUM
    echo $SUM >> $CHECKSUM_PATH
    SUM=$(sha256sum $BASE_CANISTER_PATH/$1/target/wasm32-unknown-unknown/release/${1}_canister.wasm.gz)
    echo $SUM
    echo $SUM >> $CHECKSUM_PATH
  fi
fi
