#!/usr/bin/env bash

BASE_CANISTER_PATH_EXTERNAL="backend/external_canisters"

sed -i 's/.\/gldnft_backend_1g.did.js/.\/gld_nft.did.js/g' $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/declarations/index.js
mv $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/declarations/gldnft_backend_1g.did.js $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/declarations/gld_nft.did.js
mv $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/declarations/gldnft_backend_1g.did $BASE_CANISTER_PATH_EXTERNAL/gld_nft/api/declarations/gld_nft.did
