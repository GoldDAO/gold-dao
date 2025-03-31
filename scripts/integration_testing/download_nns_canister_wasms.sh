#!/bin/bash

SCRIPT_DIR=$(dirname "$0")
cd "$SCRIPT_DIR/.."

# Create directory for wasms
mkdir -p wasms
cd wasms

# Check if the command was successful
if [ $? -ne 0 ]; then
    echo "Failed to fetch SNS version information using dfx"
    exit 1
fi

# Function to download a specific canister WASM
download_wasm() {
    CANISTER_NAME=$1
    FILE_NAME=$2
    
    echo "Downloading $CANISTER_NAME"
    
    # Use a recent commit ID that should have the latest versions
    COMMIT_ID=$(git ls-remote https://github.com/dfinity/ic.git HEAD | awk '{ print $1 }')
    echo "Commit ID: $COMMIT_ID"
    
    HTTP_CODE=$(curl -so "${CANISTER_NAME}.wasm.gz" "https://download.dfinity.systems/ic/$COMMIT_ID/canisters/$FILE_NAME.wasm.gz" --write-out "%{http_code}")
    
    if [[ ${HTTP_CODE} -ne 200 ]] ; then
        echo "Failed to download $CANISTER_NAME wasm. Response code: ${HTTP_CODE}"
        return 1
    fi
    
    echo "$CANISTER_NAME wasm downloaded successfully"
    return 0
}

# Parse version_info.txt to extract canister names and hashes
echo "Parsing version information..."

# Map canister types to their file names
GOVERNANCE_FILE="nns-governance-canister"
LEDGER_FILE="ic-icrc1-ledger"
ROOT_FILE="nns-root-canister"
SWAP_FILE="nns-swap-canister"
LEDGER_ARCHIVE_FILE="ic-icrc1-archive"
LEDGER_INDEX_FILE="ic-icrc1-index-ng"

# Extract hashes for each canister type
GOVERNANCE_HASH=$(grep -A1 "\"Governance\"" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')
LEDGER_HASH=$(grep -A1 "\"Ledger\";" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')
ROOT_HASH=$(grep -A1 "\"Root\"" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')
SWAP_HASH=$(grep -A1 "\"Swap\"" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')
LEDGER_ARCHIVE_HASH=$(grep -A1 "\"Ledger Archive\"" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')
LEDGER_INDEX_HASH=$(grep -A1 "\"Ledger Index\"" version_info.txt | grep -o '"[a-f0-9]\{64\}"' | tr -d '"')

# Download each canister using the specified names
download_wasm "nns_governance_canister" "$LEDGER_INDEX_FILE"
download_wasm "nns_root_canister" "$LEDGER_INDEX_FILE"
download_wasm "nns_swap_canister" "$LEDGER_INDEX_FILE"
download_wasm "ic_icrc1_ledger" "$LEDGER_INDEX_FILE"
download_wasm "ic_icrc1_archive" "$LEDGER_INDEX_FILE"
download_wasm "ic_icrc1_index" "$LEDGER_INDEX_FILE"

echo "Download process completed"
