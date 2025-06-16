import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import useFetchNFTAvailable from "@shared/hooks/useFetchNFTAvailable";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import NFTSelect from ".";

const AvailableNFTSelect = ({
  collection,
}: {
  collection: CollectionNameNFT;
}) => {
  const { authenticatedAgent, isConnected } = useAuth();
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  const fetchCollection = useFetchNFTAvailable(
    selectNFTState[collection].canister_id,
    authenticatedAgent,
    {
      collection_name: selectNFTState[collection].name,
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

  return <NFTSelect collection={collection} />;
};

export default AvailableNFTSelect;
