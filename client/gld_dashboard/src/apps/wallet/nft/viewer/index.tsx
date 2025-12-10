import { Tab, TabGroup, TabList } from "@headlessui/react";
import { NFTCollections } from "@shared/utils/nfts";
import CardList from "./CardList";
import useCollectionTab from "./useCollectionTab";
import { useAuth } from "@auth/index";
import useFetchNFTUserCount from "@shared/hooks/useFetchNFTUserCount";

const TabCollection = ({
  collection,
}: {
  collection: { name: string; canisterId: string };
}) => {
  const { unauthenticatedAgent, principalId, isConnected } = useAuth();

  const count = useFetchNFTUserCount(collection.canisterId, {
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
    agent: unauthenticatedAgent,
  });

  const displayCount = isConnected && count.isSuccess ? count.data : 0;

  return (
    <div className="flex items-center justify-center xl:justify-start gap-2">
      <div className="px-2 border border-border text-sm flex items-center justify-center rounded-lg order-2 xl:order-0">
        {displayCount}
      </div>
      <div className="w-[54px] flex justify-center items-center">
        <img
          className="flex-none h-12"
          src={`/gold-bars/${collection.name}.png`}
        />
      </div>
      <div>{collection.name}</div>
    </div>
  );
};

const NFTsViewer = ({ className }: { className?: string }) => {
  const {
    index: selectedIndex,
    collection: selectedCollection,
    handleChangeTab,
  } = useCollectionTab();

  return (
    <div className={className}>
      <TabGroup selectedIndex={selectedIndex} onChange={handleChangeTab}>
        <TabList className="flex gap-2">
          {NFTCollections.map((collection) => (
            <Tab
              key={collection.name}
              className="cursor-pointer w-full border border-border p-4 rounded-xl focus:not-data-focus:outline-none data-focus:outline data-focus:outline-gold data-hover:border-gold data-selected:border-gold"
            >
              <TabCollection key={collection.name} collection={collection} />
            </Tab>
          ))}
        </TabList>
      </TabGroup>
      <CardList canisterId={selectedCollection.canisterId} className="mt-8" />
    </div>
  );
};

export default NFTsViewer;
