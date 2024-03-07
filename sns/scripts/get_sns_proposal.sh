#!/usr/bin/env bash

set -euo pipefail

cd -- "$(dirname -- "${BASH_SOURCE[0]}")"

export ID="${1:-1}"

# ls

# . ./constants.sh normal

# dfx canister \
#     --network "${NETWORK}" \
#     call "${SNS_GOVERNANCE_CANISTER_ID}" \
#     get_proposal "(record {proposal_id = opt record {id = (${ID}:nat64)}})"

dfx canister \
    --network ic \
    call sns_governance \
    --candid candid/sns_governance.did \
    get_proposal "(record {proposal_id = opt record {id = (${ID}:nat64)}})"
