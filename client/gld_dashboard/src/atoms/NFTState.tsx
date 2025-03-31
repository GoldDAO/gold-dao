import { atomWithReset } from "jotai/utils";

import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
} from "@constants";

import { CollectionNameNFT, IdNFT } from "@services/gld_nft/utils/interfaces";

export interface CollectionNFT {
  name: CollectionNameNFT;
  label: string;
  value: number;
  index: number;
  nfts: IdNFT[];
  nftsSelected: IdNFT[];
  isEmpty: boolean;
  isInititialized: boolean;
  canisterId: string;
  totalCount: number;
  totalCountSelected: number;
  totalGramsSelected: number;
  totalGLDTSelected: number;
}

export const CollectionNFT1GAtom = atomWithReset<CollectionNFT>({
  name: "1G",
  label: "1 gram",
  value: 1,
  index: 0,
  isInititialized: false,
  nfts: [],
  nftsSelected: [],
  isEmpty: true,
  canisterId: GLD_NFT_1G_CANISTER_ID,
  totalCount: 0,
  totalCountSelected: 0,
  totalGramsSelected: 0,
  totalGLDTSelected: 0,
});

export const CollectionNFT10GAtom = atomWithReset<CollectionNFT>({
  name: "10G",
  label: "10 grams",
  value: 10,
  index: 1,
  isInititialized: false,
  nfts: [],
  nftsSelected: [],
  isEmpty: true,
  canisterId: GLD_NFT_10G_CANISTER_ID,
  totalCount: 0,
  totalCountSelected: 0,
  totalGramsSelected: 0,
  totalGLDTSelected: 0,
});

export const CollectionNFT100GAtom = atomWithReset<CollectionNFT>({
  name: "100G",
  label: "100 grams",
  value: 100,
  index: 2,
  isInititialized: false,
  nfts: [],
  nftsSelected: [],
  isEmpty: true,
  canisterId: GLD_NFT_100G_CANISTER_ID,
  totalCount: 0,
  totalCountSelected: 0,
  totalGramsSelected: 0,
  totalGLDTSelected: 0,
});

export const CollectionNFT1KGAtom = atomWithReset<CollectionNFT>({
  name: "1KG",
  label: "1 kilogram",
  value: 1000,
  index: 3,
  isInititialized: false,
  nfts: [],
  nftsSelected: [],
  isEmpty: true,
  canisterId: GLD_NFT_1000G_CANISTER_ID,
  totalCount: 0,
  totalCountSelected: 0,
  totalGramsSelected: 0,
  totalGLDTSelected: 0,
});
