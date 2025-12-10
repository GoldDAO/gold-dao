import { useEffect, useState } from "react";
import { useAtom } from "jotai";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import { CollectionNameNFT, NFT } from "@services/nft/utils/interfaces";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import useFetchUserNFT from "@shared/hooks/useFetchNFTUser";
import useFetchNFTAvailable from "@shared/hooks/useFetchNFTAvailable";
import Icon from "@shared/ui/icons";

export type NFTFetchType = "user" | "available";

const NFTItem = ({
  nft,
  isSelected,
  onToggle,
}: {
  nft: NFT;
  isSelected: boolean;
  onToggle: () => void;
}) => {
  return (
    <button
      type="button"
      onClick={onToggle}
      className={clsx(
        "p-3 border rounded-lg text-left transition-colors flex items-center gap-2 cursor-pointer",
        isSelected
          ? "border-content bg-content/10"
          : "border-border hover:border-primary/50"
      )}
    >
      <div className="">
        <div className="font-semibold text-sm">Serial: {nft.serial_number}</div>
        <div className="text-xs text-content/60">ID: {Number(nft.id)}</div>
      </div>
      <Icon.Check
        width={20}
        height={20}
        className={clsx(
          "flex-shrink-0",
          isSelected ? "text-content" : "opacity-0"
        )}
      />
    </button>
  );
};

export const NFTCollectionSection = ({
  collectionName,
  fetchType = "user",
}: {
  collectionName: CollectionNameNFT;
  fetchType?: NFTFetchType;
}) => {
  const Header = ({
    isOpen,
    onToggle,
  }: {
    isOpen: boolean;
    onToggle: () => void;
  }) => (
    <button
      type="button"
      onClick={onToggle}
      className="flex items-center justify-between mb-3 w-full cursor-pointer hover:opacity-80 transition-opacity text-left"
    >
      <div className="flex items-center gap-2">
        <img
          className="flex-none h-8"
          src={`/gold-bars/${collection.name}.png`}
          alt={collection.label}
        />
        <div className="font-semibold">{collection.label}</div>
        <div className="text-sm text-content/60">
          ({collection.total_count_selected} / {collection.total_count}{" "}
          selected)
        </div>
      </div>
      <Icon.Chevron
        width={16}
        height={16}
        className={clsx("transition-transform", isOpen ? "" : "rotate-180")}
      />
    </button>
  );

  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();
  const collection = selectNFTState[collectionName];
  const [isExpanded, setIsExpanded] = useState(false);

  const fetchUserCollection = useFetchUserNFT(collection.canister_id, {
    owner: principalId,
    enabled: fetchType === "user" && !!unauthenticatedAgent && isConnected,
    agent: unauthenticatedAgent,
  });

  const fetchAvailableCollection = useFetchNFTAvailable(
    collection.canister_id,
    {
      enabled:
        fetchType === "available" && !!unauthenticatedAgent && isConnected,
      agent: unauthenticatedAgent,
    }
  );

  const fetchCollection =
    fetchType === "user" ? fetchUserCollection : fetchAvailableCollection;

  useEffect(() => {
    if (fetchCollection.isSuccess) {
      dispatchSelectNFTState({
        type: "SET_COLLECTION_NFT",
        value: { name: collectionName, nfts: fetchCollection.data },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchCollection.isSuccess]);

  const [isExpandedLoading, setIsExpandedLoading] = useState(false);
  const [isExpandedEmpty, setIsExpandedEmpty] = useState(false);
  const [isExpandedInit, setIsExpandedInit] = useState(false);

  if (fetchCollection.isLoading) {
    return (
      <div className="mb-6 border border-border rounded-lg p-4">
        <Header
          isOpen={isExpandedLoading}
          onToggle={() => setIsExpandedLoading(!isExpandedLoading)}
        />
        {isExpandedLoading && (
          <div className="text-sm text-content/60 p-4 border border-border rounded-lg">
            Loading NFTs...
          </div>
        )}
      </div>
    );
  }

  if (collection.is_initialized && collection.is_empty) {
    return (
      <div className="mb-6 border border-border rounded-lg p-4">
        <Header
          isOpen={isExpandedEmpty}
          onToggle={() => setIsExpandedEmpty(!isExpandedEmpty)}
        />
        {isExpandedEmpty && (
          <div className="text-sm text-content/60 p-4 border border-border rounded-lg bg-surface-secondary">
            No NFTs available in this collection
          </div>
        )}
      </div>
    );
  }

  if (!collection.is_initialized) {
    return (
      <div className="mb-6 border border-border rounded-lg p-4">
        <Header
          isOpen={isExpandedInit}
          onToggle={() => setIsExpandedInit(!isExpandedInit)}
        />
      </div>
    );
  }

  const selectedIds = new Set(
    collection.nfts_selected.map((nft) => nft.id.toString())
  );
  const uniqueAvailableNfts = collection.nfts.filter(
    (nft) => !selectedIds.has(nft.id.toString())
  );
  const allNFTs = [...collection.nfts_selected, ...uniqueAvailableNfts].sort(
    (a, b) => {
      const aSerial = a.serial_number ?? Number(a.id);
      const bSerial = b.serial_number ?? Number(b.id);
      return aSerial - bSerial;
    }
  );

  const handleSelectAll = () => {
    dispatchSelectNFTState({
      type: "SELECT_ALL_COLLECTION",
      value: collectionName,
    });
  };

  const handleDeselectAll = () => {
    dispatchSelectNFTState({
      type: "DESELECT_ALL_COLLECTION",
      value: collectionName,
    });
  };

  const allSelected = collection.nfts.length === 0;
  const hasSelected = collection.nfts_selected.length > 0;

  return (
    <div className="mb-6 border border-border rounded-lg p-4 @container">
      <Header isOpen={isExpanded} onToggle={() => setIsExpanded(!isExpanded)} />
      {isExpanded && (
        <>
          <div className="flex justify-end gap-2 mb-3">
            {!allSelected && (
              <button
                type="button"
                onClick={handleSelectAll}
                className="px-3 py-1.5 text-sm border border-border rounded-md hover:border-content/60 hover:text-content transition-colors"
              >
                Select All
              </button>
            )}
            {hasSelected && (
              <button
                type="button"
                onClick={handleDeselectAll}
                className="px-3 py-1.5 text-sm border border-border rounded-md hover:border-content/60 hover:text-content transition-colors"
              >
                Deselect All
              </button>
            )}
          </div>
          <div className="grid gap-2 grid-cols-1 @sm:grid-cols-2 @lg:grid-cols-3">
            {allNFTs.map((nft) => {
              const isSelected = collection.nfts_selected.some(
                (selected) => selected.id.toString() === nft.id.toString()
              );
              return (
                <NFTItem
                  key={Number(nft.id)}
                  nft={nft}
                  isSelected={isSelected}
                  onToggle={() =>
                    dispatchSelectNFTState({
                      type: "TOGGLE_NFT_BY_ID",
                      value: { name: collectionName, nft_id: nft.id },
                    })
                  }
                />
              );
            })}
          </div>
        </>
      )}
    </div>
  );
};

export default NFTCollectionSection;
