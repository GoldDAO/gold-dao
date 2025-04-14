import { atomWithReducer } from "jotai/utils";
import { atom } from "jotai";
import { CollectionNameNFT, IdNFT } from "@services/gld_nft/utils/interfaces";
import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
} from "@constants";

export interface CollectionNFT {
  name: CollectionNameNFT;
  label: string;
  value: number;
  index: number;
  nfts: IdNFT[];
  nfts_selected: IdNFT[];
  is_empty: boolean;
  is_initialized: boolean;
  canister_id: string;
  total_count: number;
  total_count_selected: number;
  total_grams_selected: number;
  total_gldt_selected: number;
}

type SelectNFTState = {
  "1G": CollectionNFT;
  "10G": CollectionNFT;
  "100G": CollectionNFT;
  "1KG": CollectionNFT;
};

const initialState: SelectNFTState = {
  "1G": {
    name: "1G",
    label: "1 gram",
    value: 1,
    index: 0,
    is_initialized: false,
    nfts: [],
    nfts_selected: [],
    is_empty: true,
    canister_id: GLD_NFT_1G_CANISTER_ID,
    total_count: 0,
    total_count_selected: 0,
    total_grams_selected: 0,
    total_gldt_selected: 0,
  },
  "10G": {
    name: "10G",
    label: "10 grams",
    value: 10,
    index: 1,
    is_initialized: false,
    nfts: [],
    nfts_selected: [],
    is_empty: true,
    canister_id: GLD_NFT_10G_CANISTER_ID,
    total_count: 0,
    total_count_selected: 0,
    total_grams_selected: 0,
    total_gldt_selected: 0,
  },
  "100G": {
    name: "100G",
    label: "100 grams",
    value: 100,
    index: 2,
    is_initialized: false,
    nfts: [],
    nfts_selected: [],
    is_empty: true,
    canister_id: GLD_NFT_100G_CANISTER_ID,
    total_count: 0,
    total_count_selected: 0,
    total_grams_selected: 0,
    total_gldt_selected: 0,
  },
  "1KG": {
    name: "1KG",
    label: "1 kilogram",
    value: 1000,
    index: 3,
    is_initialized: false,
    nfts: [],
    nfts_selected: [],
    is_empty: true,
    canister_id: GLD_NFT_1000G_CANISTER_ID,
    total_count: 0,
    total_count_selected: 0,
    total_grams_selected: 0,
    total_gldt_selected: 0,
  },
};

const reducer = (
  prev: SelectNFTState,
  action:
    | {
        type: "SET_COLLECTION_NFT";
        value: { name: CollectionNameNFT; nfts: IdNFT[] };
      }
    | {
        type: "SET_ADD_NFT";
        value: CollectionNameNFT;
      }
    | {
        type: "SET_REMOVE_NFT";
        value: CollectionNameNFT;
      }
    | {
        type: "RESET";
      }
) => {
  switch (action.type) {
    case "SET_COLLECTION_NFT": {
      const { name, nfts } = action.value;
      return {
        ...prev,
        [name]: {
          ...prev[name],
          nfts,
          is_empty: !nfts.length,
          is_initialized: true,
          total_count: nfts.length,
        },
      };
    }
    case "SET_ADD_NFT": {
      const name = action.value;
      const nfts = prev[name].nfts;
      const nft = nfts.shift() as IdNFT;
      const nfts_selected = [...prev[name].nfts_selected, nft];
      const total_count_selected = nfts_selected.length;
      const total_grams_selected = total_count_selected * prev[name].value;
      const total_gldt_selected = total_grams_selected * GLDT_VALUE_1G_NFT; // Assuming GLDT_VALUE_1G_NFT is 1 for simplicity
      return {
        ...prev,
        [name]: {
          ...prev[name],
          nfts,
          nfts_selected,
          total_count_selected,
          total_grams_selected,
          total_gldt_selected,
        },
      };
    }
    case "SET_REMOVE_NFT": {
      const name = action.value;
      const nfts_selected = prev[name].nfts_selected;
      const nft = nfts_selected.pop() as IdNFT;
      const nfts = [...prev[name].nfts, nft];
      const total_count_selected = nfts_selected.length;
      const total_grams_selected = total_count_selected * prev[name].value;
      const total_gldt_selected = total_grams_selected * GLDT_VALUE_1G_NFT;
      return {
        ...prev,
        [name]: {
          ...prev[name],
          nfts,
          nfts_selected,
          total_count_selected,
          total_grams_selected,
          total_gldt_selected,
        },
      };
    }
    case "RESET": {
      return initialState;
    }
  }
};

export const SelectNFTStateReducerAtom = atomWithReducer(initialState, reducer);

export const IsOneOrMoreSelectedNFTAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return (
    state["1G"].total_count_selected ||
    state["10G"].total_count_selected ||
    state["100G"].total_count_selected ||
    state["1KG"].total_count_selected
  );
});
