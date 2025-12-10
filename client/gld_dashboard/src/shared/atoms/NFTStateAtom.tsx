import { atomWithReducer } from "jotai/utils";
import { atom } from "jotai";
import { CollectionNameNFT, NFT } from "@services/nft/utils/interfaces";
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
  nfts: NFT[];
  nfts_selected: NFT[];
  all_nfts_original_order: NFT[];
  is_empty: boolean;
  is_initialized: boolean;
  canister_id: string;
  total_count: number;
  total_count_selected: number;
  total_grams_selected: number;
  total_gldt_selected: number;
}

export type SelectNFTState = {
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
    all_nfts_original_order: [],
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
    all_nfts_original_order: [],
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
    all_nfts_original_order: [],
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
    all_nfts_original_order: [],
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
        value: { name: CollectionNameNFT; nfts: NFT[] };
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
        type: "TOGGLE_NFT_BY_ID";
        value: { name: CollectionNameNFT; nft_id: bigint };
      }
    | {
        type: "SELECT_ALL_COLLECTION";
        value: CollectionNameNFT;
      }
    | {
        type: "DESELECT_ALL_COLLECTION";
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
          nfts: [...nfts],
          all_nfts_original_order: [...nfts],
          is_empty: !nfts.length,
          is_initialized: true,
          total_count: nfts.length,
        },
      };
    }
    case "SET_ADD_NFT": {
      const name = action.value;
      const nfts = prev[name].nfts;
      const nft = nfts.shift() as NFT;
      const nfts_selected = [...prev[name].nfts_selected, nft];
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
    case "SET_REMOVE_NFT": {
      const name = action.value;
      const nfts_selected = prev[name].nfts_selected;
      const nft = nfts_selected.pop() as NFT;
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
    case "TOGGLE_NFT_BY_ID": {
      const { name, nft_id } = action.value;
      const collection = prev[name];

      const isSelected = collection.nfts_selected.some(
        (nft) => nft.id === nft_id
      );

      if (isSelected) {
        const nfts_selected = collection.nfts_selected.filter(
          (nft) => nft.id !== nft_id
        );
        const nft = collection.nfts_selected.find((nft) => nft.id === nft_id);
        const nfts = nft ? [...collection.nfts, nft] : collection.nfts;
        const total_count_selected = nfts_selected.length;
        const total_grams_selected = total_count_selected * collection.value;
        const total_gldt_selected = total_grams_selected * GLDT_VALUE_1G_NFT;

        return {
          ...prev,
          [name]: {
            ...collection,
            nfts,
            nfts_selected,
            total_count_selected,
            total_grams_selected,
            total_gldt_selected,
          },
        };
      } else {
        const nft = collection.nfts.find((nft) => nft.id === nft_id);
        if (!nft) return prev;

        const nfts = collection.nfts.filter((nft) => nft.id !== nft_id);
        const nfts_selected = [...collection.nfts_selected, nft];
        const total_count_selected = nfts_selected.length;
        const total_grams_selected = total_count_selected * collection.value;
        const total_gldt_selected = total_grams_selected * GLDT_VALUE_1G_NFT;

        return {
          ...prev,
          [name]: {
            ...collection,
            nfts,
            nfts_selected,
            total_count_selected,
            total_grams_selected,
            total_gldt_selected,
          },
        };
      }
    }
    case "SELECT_ALL_COLLECTION": {
      const name = action.value;
      const collection = prev[name];

      const allNFTs = [...collection.nfts, ...collection.nfts_selected];

      const total_count_selected = allNFTs.length;
      const total_grams_selected = total_count_selected * collection.value;
      const total_gldt_selected = total_grams_selected * GLDT_VALUE_1G_NFT;

      return {
        ...prev,
        [name]: {
          ...collection,
          nfts: [],
          nfts_selected: allNFTs,
          total_count_selected,
          total_grams_selected,
          total_gldt_selected,
        },
      };
    }
    case "DESELECT_ALL_COLLECTION": {
      const name = action.value;
      const collection = prev[name];

      const allNFTs = [...collection.nfts, ...collection.nfts_selected];

      return {
        ...prev,
        [name]: {
          ...collection,
          nfts: allNFTs,
          nfts_selected: [],
          total_count_selected: 0,
          total_grams_selected: 0,
          total_gldt_selected: 0,
        },
      };
    }
    case "RESET": {
      return initialState;
    }
  }
};

export const SelectNFTStateReducerAtom = atomWithReducer(initialState, reducer);

export const TotalGLDTSelectedAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return (
    state["1G"].total_gldt_selected +
    state["10G"].total_gldt_selected +
    state["100G"].total_gldt_selected +
    state["1KG"].total_gldt_selected
  );
});

export const TotalGramSelectedAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return (
    state["1G"].total_grams_selected +
    state["10G"].total_grams_selected +
    state["100G"].total_grams_selected +
    state["1KG"].total_grams_selected
  );
});

export const TotalNFTSelectedAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return (
    state["1G"].total_count_selected +
    state["10G"].total_count_selected +
    state["100G"].total_count_selected +
    state["1KG"].total_count_selected
  );
});

export const CollectionSelectedAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return [state["1G"], state["10G"], state["100G"], state["1KG"]].filter(
    (collection) => collection.total_count_selected > 0
  );
});

export const TotalCollectionSelectedAtom = atom((get) => {
  const state = get(SelectNFTStateReducerAtom);
  return [state["1G"], state["10G"], state["100G"], state["1KG"]].filter(
    (collection) => collection.total_count_selected > 0
  ).length;
});
