#!/usr/bin/env bash

sed -i 's/.\/gldnft_backend_1g.did.js/.\/gld_nft.did.js/g' canister/gld_nft/declarations/index.js
mv canister/gld_nft/declarations/gldnft_backend_1g.did.js canister/gld_nft/declarations/gld_nft.did.js
mv canister/gld_nft/declarations/gldnft_backend_1g.did canister/gld_nft/declarations/gld_nft.did
