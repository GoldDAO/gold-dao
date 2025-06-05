import {
  GLD_NFT_1G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_1000G_CANISTER_ID,
} from "@constants";
import { CollectionNameNFT } from "@services/gld_nft/utils/interfaces";
import useFetchUserNFT from "@shared/hooks/useFetchNFTUser";
import { useAuth } from "@auth/index";
import { LoaderSpin } from "@components/index";

interface NFTCollectionState {
  canister_id: string;
  name: CollectionNameNFT;
}

const CollectionItem = ({ collection }: { collection: NFTCollectionState }) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();

  const fetchCollection = useFetchUserNFT(
    collection.canister_id,
    authenticatedAgent,
    {
      owner: principalId,
      collection_name: collection.name,
      enabled: !!authenticatedAgent && isConnected,
    }
  );
  return (
    <div className="border border-border p-4 rounded-xl">
      {fetchCollection.isSuccess ? (
        <div className="flex items-center justify-center xl:justify-start gap-2">
          <div className="px-2 bg-primary text-white flex items-center justify-center rounded">
            {fetchCollection.data.length}
          </div>
          <div className="w-[64px] flex justify-center items-center">
            <img
              className="flex-none h-12"
              src={`/gold-bars/${collection.name}.svg`}
            />
          </div>
          <div>{collection.name}</div>
        </div>
      ) : (
        <div className="flex justify-center items-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};

const UserNFTsSection = ({ className }: { className?: string }) => {
  const collections: NFTCollectionState[] = [
    {
      canister_id: GLD_NFT_1G_CANISTER_ID,
      name: "1G",
    },
    {
      canister_id: GLD_NFT_10G_CANISTER_ID,
      name: "10G",
    },
    {
      canister_id: GLD_NFT_100G_CANISTER_ID,
      name: "100G",
    },
    {
      canister_id: GLD_NFT_1000G_CANISTER_ID,
      name: "1KG",
    },
  ];

  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-4 xl:mb-8">
        <div>My NFTs</div>
      </div>
      <div className="grid grid-cols-1 xl:grid-cols-4 gap-4">
        {collections.map((collection) => (
          <CollectionItem key={collection.name} collection={collection} />
        ))}
      </div>
    </div>
  );
};

export default UserNFTsSection;
