# Deploy scripts for staging deployment

# 1. ONLY NEEDED FOR LOCAL DEVELOPMENT, FOR STAGING / PROD, start with step 2
# create gldt core canister. Needed to deploy gldt_ledger to assign the minting account.
dfx canister create gldt_core

# Replace "--network staging" with "--network local" for local development
# 2. deploy gldt ledger
dfx deploy gldt_ledger --network local --argument '(record{minting_account="'"$(dfx ledger account-id --network local --of-canister gldt_core)"'"; send_whitelist=vec{}; initial_values=vec{}})'
dfx deploy gldt_ledger --network staging --argument '(record{minting_account="'"$(dfx ledger account-id --network staging --of-canister gldt_core)"'"; send_whitelist=vec{}; initial_values=vec{}})'

# 3. deploy gldt core
dfx deploy gldt_core --network local --argument '(opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network local gldt_ledger)"'"; gldt_nft_canister_ids=vec{record { principal "obapm-2iaaa-aaaak-qcgca-cai"; record { grams=1}};record { principal "xyo2o-gyaaa-aaaal-qb55a-cai"; record { grams=10}}}})'
dfx deploy gldt_core --network staging --argument '(opt record {gldt_ledger_canister_id=principal "'"$(dfx canister id --network staging gldt_ledger)"'"; gldt_nft_canister_ids=vec{record { principal "obapm-2iaaa-aaaak-qcgca-cai"; record { grams=1}};record { principal "xyo2o-gyaaa-aaaal-qb55a-cai"; record { grams=10}}}})'

# 4. deploy gldt frontend
dfx deploy gldt_frontend --network local
dfx deploy gldt_frontend --network staging
