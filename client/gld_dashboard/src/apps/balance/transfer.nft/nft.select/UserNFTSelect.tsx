import { useEffect } from "react";
import { SetStateAction, useAtom, WritableAtom } from "jotai";

import { useAuth } from "@auth/index";
import { CollectionNFT } from "@atoms/NFTState";

import NFTSelect from "./NFTSelect";
import useFetchUserNFT from "@services/gld_nft/hooks/useFetchUserNFT";

const UserNFTSelect = ({
  className,
  collectionAtom,
}: {
  className?: string;
  collectionAtom: WritableAtom<
    CollectionNFT,
    [SetStateAction<CollectionNFT>],
    void
  >;
}) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();
  const [collectionState, setCollectionState] = useAtom(collectionAtom);

  const fetchCollection = useFetchUserNFT(
    collectionState.canisterId,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: collectionState.name,
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  useEffect(() => {
    if (fetchCollection.isSuccess) {
      setCollectionState((state: CollectionNFT) => ({
        ...state,
        isInititialized: true,
        nfts: fetchCollection.data,
        isEmpty: fetchCollection.data.length === 0,
        totalCount: fetchCollection.data.length,
      }));
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchCollection.isSuccess]);

  return <NFTSelect collectionAtom={collectionAtom} className={className} />;
};

export default UserNFTSelect;
