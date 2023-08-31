# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=staging

# Change the variable to the account that can mint and burn tokens.
# export MINT_ACC=$(dfx ledger account-id --network ${NETWORK} --of-canister gldt_core)
export MINT_ACC=$(dfx canister id --network ${NETWORK} gldt_core)

# Change the variable to the principal that controls archive canisters.
export ARCHIVE_CONTROLLER=$(dfx identity get-principal)

export TOKEN_NAME="Gold token"
export TOKEN_SYMBOL="GLDT"

dfx deploy --network ${NETWORK} gldt_ledger --argument "(variant {Init = record {
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
}})"



# # to deploy rosetta API
# docker run \
#     --interactive \
#     --tty \
#     --publish 8081:8080 \
#     --rm \
#     dfinity/rosetta-api:latest \
#     --canister-id 6uad6-fqaaa-aaaam-abovq-cai \
#     --ic-url "https://icp0.io" \
#     -t GLDT
