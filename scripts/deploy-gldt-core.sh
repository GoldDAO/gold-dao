# Change the variable to "ic" to deploy the ledger on the mainnet.
export NETWORK=staging


dfx deploy gldt_core --network ${NETWORK} --argument '(
  opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network ${NETWORK} gldt_ledger)"'";
  gld_nft_canister_ids=vec{
    record { principal "'"$(dfx canister id --network ${NETWORK} gldnft_backend_1g)"'"; record { grams=1}};
    record { principal "'"$(dfx canister id --network ${NETWORK} gldnft_backend_10g)"'"; record { grams=10}}
    }})' --mode reinstall

    # record { principal "'"$(dfx identity get-principal)"'"; record { grams=100}};
