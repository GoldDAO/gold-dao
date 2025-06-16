import { NFTCollection } from "@services/gld_nft/utils/interfaces";
import useFetchUserNFT from "@shared/hooks/useFetchNFTUser";
import { NFTCollections } from "@shared/utils/nfts";
import { useAuth } from "@auth/index";

const CountCollectionNFT = ({
  count = 0,
  name,
  className,
}: {
  count: number;
  name: "1G" | "10G" | "100G" | "1KG";
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="flex items-center justify-center xl:justify-start gap-2">
        <div className="px-2 bg-gold text-white flex items-center justify-center rounded">
          {count}
        </div>
        <div className="w-[64px] flex justify-center items-center">
          <img className="flex-none h-12" src={`/gold-bars/${name}.svg`} />
        </div>
        <div>{name}</div>
      </div>
    </div>
  );
};

const CollectionItem = ({ collection }: { collection: NFTCollection }) => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();

  const fetchCollection = useFetchUserNFT(
    collection.canisterId,
    authenticatedAgent,
    {
      owner: principalId,
      collection_name: collection.name,
      enabled: !!authenticatedAgent && isConnected,
    }
  );

  const renderCountCollectionNFT = () => {
    if (!isConnected) {
      return <CountCollectionNFT count={0} name={collection.name} />;
    }
    if (fetchCollection.isSuccess) {
      return (
        <CountCollectionNFT
          count={fetchCollection.data.length}
          name={collection.name}
        />
      );
    }
    return (
      <CountCollectionNFT
        className="animate-pulse"
        count={0}
        name={collection.name}
      />
    );
  };

  return (
    <div className="border border-border p-4 rounded-xl">
      {renderCountCollectionNFT()}
    </div>
  );
};

const UserNFTsSection = ({ className }: { className?: string }) => {
  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-4 xl:mb-8">
        <div>My NFTs</div>
      </div>
      <div className="grid grid-cols-1 xl:grid-cols-4 gap-4">
        {NFTCollections.map((collection) => (
          <CollectionItem key={collection.name} collection={collection} />
        ))}
      </div>
    </div>
  );
};

export default UserNFTsSection;
