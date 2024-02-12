#!/usr/bin/env bash

BASE_CANISTER_PATH_EXTERNAL="backend/external_canisters"

sed -i 's/.\/gldnft_backend_1g.did.js/.\/gld_nft.did.js/g' $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/index.js
mv $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/gldnft_backend_1g.did.js $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/gld_nft.did.js
mv $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/gldnft_backend_1g.did $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/gld_nft.did
