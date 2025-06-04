import { CollectionNFT } from "@shared/atoms/NFTStateAtom";
import CollectionItem from "./CollectionItem";

const Collection = ({ collection }: { collection: CollectionNFT }) => {
  return (
    <div className="p-4 border border-border rounded-md">
      <div className="text-xl p-2 mb-3">{collection.label} collection</div>
      <div className="flex flex-col gap-2">
        {collection.nfts_selected.map((nft) => (
          <CollectionItem
            key={nft.id_string}
            nft={nft}
            canisterId={collection.canister_id}
            value={collection.value}
            name={collection.name}
          />
        ))}
      </div>
    </div>
  );
};

export default Collection;
