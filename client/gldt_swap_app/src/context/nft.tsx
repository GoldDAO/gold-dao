import {
  createContext,
  useContext,
  ReactNode,
  useState,
  useCallback,
  useMemo,
  useEffect,
} from "react";

import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
  REVERSE_GLDT_TX_FEE,
  GLDT_DECIMAL,
} from "@constants";

import { useAuth } from "@context/auth";

export type TokenId = {
  id_string: string;
  id_bigint: bigint;
  selected?: boolean;
  id_byte_array?: Uint8Array | [];
};

export interface Nft {
  tokenIds: TokenId[];
}

export type CollectionName = "1g" | "10g" | "100g" | "1000g";

// eslint-disable-next-line react-refresh/only-export-components
export enum CollectionIndex {
  GLD_NFT_1G,
  GLD_NFT_10G,
  GLD_NFT_100G,
  GLD_NFT_1000G,
}

export interface NftCollection {
  name: CollectionName;
  value: number;
  index: CollectionIndex;
  tokenIds: Array<TokenId>;
  isEmpty: boolean;
  canisterId: string;
  canister: string;
  totalSelected: number;
  totalSelectedGram: number;
  totalSelectedGLDT: number;
}

export interface NftState {
  isEmpty: boolean;
  nfts: Array<NftCollection>;
  isLoadingInit: boolean;
}

const initialState: NftState = {
  isEmpty: false,
  nfts: [
    {
      name: "1g",
      value: 1,
      index: CollectionIndex.GLD_NFT_1G,
      tokenIds: [],
      isEmpty: true,
      canisterId: GLD_NFT_1G_CANISTER_ID,
      canister: "gld_nft_1g",
      totalSelected: 0,
      totalSelectedGram: 0,
      totalSelectedGLDT: 0,
    },
    {
      name: "10g",
      value: 10,
      index: CollectionIndex.GLD_NFT_10G,
      tokenIds: [],
      isEmpty: true,
      canisterId: GLD_NFT_10G_CANISTER_ID,
      canister: "gld_nft_10g",
      totalSelected: 0,
      totalSelectedGram: 0,
      totalSelectedGLDT: 0,
    },
    {
      name: "100g",
      value: 100,
      index: CollectionIndex.GLD_NFT_100G,
      tokenIds: [],
      isEmpty: true,
      canisterId: GLD_NFT_100G_CANISTER_ID,
      canister: "gld_nft_100g",
      totalSelected: 0,
      totalSelectedGram: 0,
      totalSelectedGLDT: 0,
    },
    {
      name: "1000g",
      value: 1000,
      index: CollectionIndex.GLD_NFT_1000G,
      tokenIds: [],
      isEmpty: true,
      canisterId: GLD_NFT_1000G_CANISTER_ID,
      canister: "gld_nft_1000g",
      totalSelected: 0,
      totalSelectedGram: 0,
      totalSelectedGLDT: 0,
    },
  ],
  isLoadingInit: true,
};

const NftContext = createContext<ReturnType<typeof useNftProviderValue> | null>(
  null
);

// eslint-disable-next-line react-refresh/only-export-components
export const useNft = () => {
  const context = useContext(NftContext);
  if (!context) {
    throw new Error("useNft must be used within a NftProvider");
  }
  return context;
};

const useNftProviderValue = () => {
  const [state, setState] = useState<NftState>(initialState);
  const { state: authState } = useAuth();
  const { isConnected } = authState;

  const setNfts = useCallback((nfts: Nft[]): void => {
    setState((prevState) => {
      const newNfts = nfts.map((nft, index) => {
        const newIds = nft.tokenIds.map((tokenId) => ({
          id_string: tokenId.id_string,
          id_bigint: tokenId.id_bigint,
          id_byte_array: tokenId?.id_byte_array ?? [],
          selected: false,
        }));
        return {
          ...prevState.nfts[index],
          tokenIds: newIds,
          isEmpty: !newIds.length,
        };
      });

      const isEmpty = nfts.every((nft) => !nft.tokenIds.length);

      return {
        ...prevState,
        nfts: newNfts,
        isEmpty,
      };
    });
  }, []);

  const getNftById = ({
    collectionIndex: i,
    nftId,
  }: {
    collectionIndex: CollectionIndex;
    nftId: string;
  }): void => {
    setState((prevState) => {
      const newNfts = [...prevState.nfts];
      const indexId = newNfts[i].tokenIds.findIndex(
        (e: TokenId) => e.id_string === nftId
      );

      if (indexId !== -1) {
        const newTokenIds = [...newNfts[i].tokenIds];
        newTokenIds[indexId] = {
          ...newTokenIds[indexId],
          selected: !newTokenIds[indexId].selected,
        };
        newNfts[i] = {
          ...newNfts[i],
          tokenIds: newTokenIds,
        };
      }

      return {
        ...prevState,
        nfts: newNfts,
      };
    });
  };

  const canBuyNft = (
    collectionIndex: CollectionIndex,
    user_balance: number
  ): boolean => {
    const totalGLDTtoSwap = getSelectedTotalGLDT();
    const singlePrice = state.nfts[collectionIndex].value * 100;
    const remaining =
      user_balance -
      totalGLDTtoSwap -
      singlePrice -
      REVERSE_GLDT_TX_FEE / GLDT_DECIMAL;
    return remaining > 0;
  };

  // selects a random NFT
  const selectNft = (collectionIndex: CollectionIndex): void => {
    setState((prevState) => {
      const newNfts = [...prevState.nfts];
      const collectionMeta = newNfts[collectionIndex];
      const collection = newNfts[collectionIndex].tokenIds;
      const collectionLength = collection.length;

      let randomIndex = getRandomNumber(collectionLength - 1);
      if (collectionMeta.totalSelected === collectionLength) {
        return {
          ...prevState,
        };
      }
      while (
        collection.find((token, index) => {
          return index === randomIndex && token.selected === true;
        })
      ) {
        randomIndex = getRandomNumber(collectionLength - 1);
      }
      collection[randomIndex] = {
        ...collection[randomIndex],
        selected: true,
      };
      newNfts[collectionIndex] = {
        ...newNfts[collectionIndex],
        tokenIds: collection,
        totalSelected: newNfts[collectionIndex].totalSelected + 1,
        totalSelectedGram:
          newNfts[collectionIndex].totalSelectedGram +
          newNfts[collectionIndex].value,
        totalSelectedGLDT:
          newNfts[collectionIndex].totalSelectedGLDT +
          newNfts[collectionIndex].value * GLDT_VALUE_1G_NFT,
      };

      const newState = {
        ...prevState,
        nfts: newNfts,
      };
      return newState;
    });
  };

  const unselectNft = (collectionIndex: CollectionIndex): void => {
    const i = collectionIndex;
    setState((prevState) => {
      const newNfts = [...prevState.nfts];
      const indexId = newNfts[i].tokenIds.findIndex(
        (e: TokenId) => e.selected === true
      );

      if (indexId !== -1) {
        const newTokenIds = [...newNfts[i].tokenIds];
        newTokenIds[indexId] = {
          ...newTokenIds[indexId],
          selected: false,
        };
        newNfts[i] = {
          ...newNfts[i],
          tokenIds: newTokenIds,
          totalSelected: newNfts[i].totalSelected - 1,
          totalSelectedGram: newNfts[i].totalSelectedGram - newNfts[i].value,
          totalSelectedGLDT:
            newNfts[i].totalSelectedGLDT - newNfts[i].value * GLDT_VALUE_1G_NFT,
        };
      }

      return {
        ...prevState,
        nfts: newNfts,
      };
    });
  };

  const getCountNfts = () => {
    return state.nfts.map((nft) => ({
      selected: nft.tokenIds.filter((e) => e.selected).length,
      total: nft.tokenIds.length,
    }));
  };

  const getCollectionSelectedNFTs = () => {
    const selected = state.nfts.map((nft: NftCollection) => ({
      ...nft,
      tokenIds: nft.tokenIds.filter((e: TokenId) => e.selected === true),
    }));
    return selected.filter((c) => c.tokenIds.length !== 0);
  };

  const getSelectedCollectionGLDTNFTs = (collectionIndex: CollectionIndex) => {
    const i = collectionIndex;
    const indexId = state.nfts[i].tokenIds.findIndex(
      (e: TokenId) => e.selected === true
    );
    return state.nfts[indexId].totalSelectedGLDT;
  };

  const getSelectedTotal = () => {
    return state.nfts.reduce(
      (acc, nft: NftCollection) => acc + nft.totalSelected,
      0
    );
  };

  const getSelectedTotalGram = () => {
    return state.nfts.reduce(
      (acc, nft: NftCollection) => acc + nft.totalSelectedGram,
      0
    );
  };

  const getSelectedTotalGLDT = () => {
    return state.nfts.reduce(
      (acc, nft: NftCollection) => acc + nft.totalSelectedGLDT,
      0
    );
  };

  const resetState = (): void => {
    setState(initialState);
  };

  useEffect(() => {
    if (!isConnected) {
      resetState();
    }
  }, [isConnected]);

  const value = useMemo(
    () => ({
      state,
      setNfts,
      getNftById,
      selectNft,
      unselectNft,
      getCountNfts,
      getCollectionSelectedNFTs,
      getSelectedCollectionGLDTNFTs,
      getSelectedTotal,
      getSelectedTotalGram,
      getSelectedTotalGLDT,
      resetState,
      canBuyNft,
    }),
    // eslint-disable-next-line react-hooks/exhaustive-deps
    [state]
  );
  return value;
};

export const NftProvider = ({ children }: { children: ReactNode }) => {
  const contextValue = useNftProviderValue();

  return (
    <NftContext.Provider value={contextValue}>{children}</NftContext.Provider>
  );
};

function getRandomNumber(n: number): number {
  return Math.floor(Math.random() * (n + 1));
}
