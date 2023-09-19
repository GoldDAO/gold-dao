#!/usr/bin/env bash
: << EOF
=pod

=head1 NAME

Generate the candid file and declarations for the mentioned wasm canister.

=head1 SYNOPSYS

generate-did [options] <wasm>

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
Generate the candid file and declarations for the mentioned wasm canister.
Usage:
  generate-did [options] <wasm>

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

defaultpath="canister/$1/src"
did_path="${outpath:-$defaultpath}"
if [[ $dryrun -eq 1 ]]; then
  echo -e "This would be written to ${did_path}/${1}.did :\n"
  candid-extractor "canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm" 2>/dev/null
else
  candid-extractor "canister/$1/target/wasm32-unknown-unknown/release/${1}.wasm" 2>/dev/null > $did_path/$1.did
fi
