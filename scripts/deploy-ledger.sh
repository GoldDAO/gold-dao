#!/usr/bin/env bash

show_help() {
  cat << EOF
gldt_ledger canister deployment script.
Must be run from the repository's root folder, and with a running replica if for local deployment.
'staging' and 'ic' networks can only be selected from a Gitlab CI/CD environment.
The NETWORK argument should preferably be passed from the env variable that was previously defined
by the pre-deploy script (using the dot notation, or inside a macro deploy script).

Usage:
  scripts/deploy-ledger [options] <NETWORK>

Options:
  -h, --help        Show this message and exit
  --upgrade         Upgrade running canister (whithout this option, it won't be deployed if already running)
EOF
}

# TODO: add a --identity option ?? (See dfx deploy --identity)
if [[ $# -gt 0 ]]; then
  while [[ "$1" =~ ^- && ! "$1" == "--" ]]; do
    case $1 in
      -h | --help )
        show_help
        exit
        ;;
      --upgrade )
        upgrade_me=1
        ;;
    esac;
    shift;
  done
  if [[ "$1" == '--' ]]; then shift; fi
else
  echo "Error: missing <NETWORK> argument"
  exit 1
fi

if [[ ! $1 =~ ^(local|staging|ic)$ ]]; then
  echo "Error: unknown network for deployment"
  exit 2
fi

if [[ upgrade_me -eq 1 ]]; then
  echo -e "\n\t\033[1;5;31mWARNING\033[0;31m  This script could re-deploy \033[1mgldt_ledger\033[0;31m on \033[7m${1}\033[0;31m !!\033[0m"
  echo -e "\tIf this is NOT desirable, you are lucky, it did not start yet, and \033[1myou have now 20 seconds to abort (CTRL+C locally, or cancel the CI job) !\033[0m\n"
  sleep 20s
fi

# Change the variable to the account that can mint and burn tokens.
# export MINT_ACC=$(dfx ledger account-id --network ${1} --of-canister gldt_core)
export MINT_ACC=$(dfx canister id gldt_core --network $1)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="Gold token"
export TOKEN_SYMBOL="GLDT"

# The defx deploy commands below could have been regrouped into a single conditional group with variables as arguments
# But for security and clarity reasons, they have been kept distinct, to avoid any potential unwanted execution.

if [[ $1 == "local" ]]; then
  dfx canister call --network $1 gldt_ledger get_data_certificate 2>/dev/null > /dev/null
  if [[  $? -ne 0 || upgrade_me -eq 1 ]]; then
    dfx deploy --network $1 gldt_ledger --argument "(variant {Init = record {
      token_name = \"${TOKEN_NAME}\";
      token_symbol = \"${TOKEN_SYMBOL}\";
      minting_account = record {owner = principal \"${MINT_ACC}\"};
      metadata = vec {};
      transfer_fee = 10000;
      initial_balances = vec {};
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        cycles_for_archive_creation = opt 10_000_000_000_000;
      }
    }})" -y
  else
    echo -e "\033[31mgldt_ledger is already deployed and running on \033[7m${1}\033[0;31m with id \033[1m${GLDT_LEDGER_ID}\033[0;31m. To upgrade it, use the --upgrade option.\033[0m"
  fi  
elif [[ $1 == "staging" && $CI_COMMIT_REF_NAME == "develop" ]]; then
  dfx canister call --network $1 gldt_ledger get_data_certificate 2>/dev/null > /dev/null
  if [[ $? -ne 0 || upgrade_me -eq 1 ]]; then
    dfx deploy --network $1 gldt_ledger --argument "(variant {Init = record {
      token_name = \"${TOKEN_NAME}\";
      token_symbol = \"${TOKEN_SYMBOL}\";
      minting_account = record {owner = principal \"${MINT_ACC}\"};
      metadata = vec {};
      transfer_fee = 10000;
      initial_balances = vec {};
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        cycles_for_archive_creation = opt 10_000_000_000_000;
      }
    }})" --no-wallet -y
  else
    echo -e "\033[31mgldt_ledger is already deployed and running on \033[7m${1}\033[0;31m with id \033[1m${GLDT_LEDGER_ID}\033[0;31m. To upgrade it, use the --upgrade option.\033[0m"
  fi
elif [[ $1 == "ic" && $CI_COMMIT_TAG =~ ^ledger-v{1}[[:digit:]]{1,2}.[[:digit:]]{1,2}.[[:digit:]]{1,3}$ ]]; then
  dfx canister call --network $1 gldt_ledger get_data_certificate 2>/dev/null > /dev/null
  if [[ $? -ne 0 ]]; then
    dfx deploy --network $1 gldt_ledger --argument "(variant {Init = record {
      token_name = \"${TOKEN_NAME}\";
      token_symbol = \"${TOKEN_SYMBOL}\";
      minting_account = record {owner = principal \"${MINT_ACC}\"};
      metadata = vec {};
      transfer_fee = 10000;
      initial_balances = vec {};
      archive_options = record {
        trigger_threshold = 2000;
        num_blocks_to_archive = 1000;
        controller_id = principal \"${ARCHIVE_CONTROLLER}\";
        cycles_for_archive_creation = opt 10_000_000_000_000;
      }
    }})" --no-wallet -y
  elif [[ upgrade_me -eq 1  ]]; then
    ###########################################################################################################
    echo "TODO: implement script to retrieve existing balance and reinstall passing values to initial_balances"
    ###########################################################################################################
  else
    echo -e "\033[31mgldt_ledger is already deployed and running on \033[7m${1}\033[0;31m with id \033[1m${GLDT_LEDGER_ID}\033[0;31m. To upgrade it, use the --upgrade option.\033[0m"
  fi
else
  echo "Error: no valid deployment conditions found."
  exit 3
fi
