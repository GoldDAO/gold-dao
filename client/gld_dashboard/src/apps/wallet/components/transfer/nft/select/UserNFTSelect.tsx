import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import useFetchUserNFT from "@services/gld_nft/hooks/useFetchUserNFT";
import { SelectNFTStateReducerAtom } from "@wallet/atoms/NFTStateAtom";
import NFTSelect from "./NFTSelect";

const UserNFTSelect = ({
  className,
  collection,
}: {
  className?: string;
  collection: CollectionNameNFT;
}) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  const fetchCollection = useFetchUserNFT(
    selectNFTState[collection].canister_id,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: selectNFTState[collection].name,
      enabled: !!authenticatedAgent && isConnected,
    }
  );

  useEffect(() => {
    if (fetchCollection.isSuccess) {
      dispatchSelectNFTState({
        type: "SET_COLLECTION_NFT",
        value: { name: collection, nfts: fetchCollection.data },
      });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [fetchCollection.isSuccess]);

  return <NFTSelect collection={collection} className={className} />;
};

export default UserNFTSelect;
