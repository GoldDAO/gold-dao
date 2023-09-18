#!/usr/bin/env bash
: << EOF
=pod

=head1 NAME

Generate the candid file and declarations for the mentioned canister.

=head1 SYNOPSYS

generate-did [options] canister

=head1 OPTIONS

-h, --help
  Show this message and exit

-o, --output PATH
  The path where to write the resulting candid file (Must be a folder)

-d, --dry-run
  Only output the result, without actually writing on disk

=cut

EOF

show_help() {
  cat << EOF
Generate the candid file and declarations for the mentioned canister.
Usage:
  generate-did [options] canister

Options:
  -h, --help        Show this message and exit
  -o, --output PATH The path where to write the resulting candid file (Must be a folder)
  -d, --dry-run     Only output the result, without actually writing on disk
EOF
}

if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      -o | --output )
        shift; outpath=$1
        ;;
      -d | --dry-run )
        dryrun=1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: not enough arguments."
  show_help
  exit 1
fi

cargo build --target wasm32-unknown-unknown --target-dir canister/$1/target \
  --release --locked --features "ic-cdk/wasi" -p $1

defaultpath="canister/$1/src"
did_path="${outpath:-$defaultpath}"
if [[ $dryrun -eq 1 ]]; then
  echo "This would be written to ${did_path}/${1}.did :"
  wasmtime "canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm"
else
  wasmtime "canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm" > "$did_path/$1.did" &&
  dfx generate $1
fi
