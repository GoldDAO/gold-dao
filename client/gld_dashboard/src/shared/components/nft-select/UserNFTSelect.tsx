import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { CollectionNameNFT } from "@services/nft/utils/interfaces";
import useFetchUserNFT from "@shared/hooks/useFetchNFTUser";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import NFTSelect from ".";

const UserNFTSelect = ({ collection }: { collection: CollectionNameNFT }) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();
  const [selectNFTState, dispatchSelectNFTState] = useAtom(
    SelectNFTStateReducerAtom
  );

  const fetchCollection = useFetchUserNFT(
    selectNFTState[collection].canister_id,
    {
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
      agent: unauthenticatedAgent,
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

export default UserNFTSelect;
