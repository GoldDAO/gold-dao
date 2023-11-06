#!/usr/bin/env bash

show_help() {
  cat << EOF
Pre-deployment commands.
Must be run from the repository's root folder, and with a running replica for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.

Usage:
  scripts/pre-deploy [options]

Options:
  -h, --help        Show this message and exit
  --network NETWORK The network where the commands will be executed (default is 'local')
EOF
}

export NETWORK="local"

if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      --network )
        shift; export NETWORK=$1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
fi

create_canister () {
	if [[ $2 == "local" ]]; then
		dfx canister create $1
		CANISTER_ID=$(dfx canister id $1)
	elif [[ ($2 == "staging" || $2 == "ic") && $CI ]]; then
		dfx canister create $1 --network $2
		CANISTER_ID=$(dfx canister id $1 --network $2)
	else
		echo "Error during canister creation: unknown network ${2}"
		exit 1
	fi
	echo $CANISTER_ID
}

check_and_create_canister () {
	if [[ $2 == "local" ]]; then
		echo $(dfx canister id $1 2>/dev/null || echo $(create_canister $1 $2))
	elif [[ ($2 == "staging" && $CI) || ($2 == "ic" && $CI_COMMIT_TAG =~ ^(ledger|core|swap_app)-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$) ]]; then
		if [[ $(cat canister_ids.json | jq -r .$1.$2) == "" ]]; then
			echo $(create_canister $1 $2)
		else
			echo $(cat canister_ids.json | jq -r .$1.$2)
		fi
	else
		echo "Error: unknown network ${2}"
		exit 1
	fi
}

echo -e "\nCanisters IDs on $NETWORK:\n"
IMPORTANT_MSG=""
export GLDT_CORE_ID=$(check_and_create_canister gldt_core $NETWORK)
echo -e "gldt_core      \033[1m${GLDT_CORE_ID}\033[0m${IMPORTANT_MSG}"
export GLDT_LEDGER_ID=$(check_and_create_canister gldt_ledger $NETWORK)
echo -e "gldt_ledger    \033[1m${GLDT_LEDGER_ID}\033[0m${IMPORTANT_MSG}"
export GLDT_FRONT_ID=$(check_and_create_canister gldt_swap_app $NETWORK)
echo -e "gldt_swap_app  \033[1m${GLDT_FRONT_ID}\033[0m${IMPORTANT_MSG}\n"
export GLDT_COMPENSATION_ID=$(check_and_create_canister gldt_fee_compensation $NETWORK)
echo -e "gldt_fee_compensation  \033[1m${GLDT_COMPENSATION_ID}\033[0m${IMPORTANT_MSG}\n"

if [[ ($NETWORK == "staging" || $NETWORK == "ic") && $CI ]]; then
	echo -e "\n  \033[1;5;31mIMPORTANT\033[0m  If a canister id has just been created on \033[7m${NETWORK}\033[0m. Please update and version \033[4m'canister_ids.json'\033[0m on both the \033[7m'master'\033[0m and \033[7m'develop'\033[0m branches ASAP.\n"
fi
