import { CollectionNameNFT, NFTCollection } from "@services/gld_nft/utils/interfaces";
import {
    GLD_NFT_1G_CANISTER_ID,
    GLD_NFT_10G_CANISTER_ID,
    GLD_NFT_100G_CANISTER_ID,
    GLD_NFT_1000G_CANISTER_ID,
} from "@constants";

export const NFTCollectionNames: CollectionNameNFT[] = ["1G", "10G", "100G", "1KG"];

export const NFTCollections: NFTCollection[] = [
    { canisterId: GLD_NFT_1G_CANISTER_ID, grams: 1, name: "1G" },
    { canisterId: GLD_NFT_10G_CANISTER_ID, grams: 10, name: "10G" },
    { canisterId: GLD_NFT_100G_CANISTER_ID, grams: 100, name: "100G" },
    { canisterId: GLD_NFT_1000G_CANISTER_ID, grams: 1000, name: "1KG" },
];