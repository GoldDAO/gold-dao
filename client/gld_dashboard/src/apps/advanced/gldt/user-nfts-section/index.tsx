// import { Tab, TabGroup, TabList } from "@headlessui/react";
import { NFTCollections } from "@shared/utils/nfts";
import { useAuth } from "@auth/index";
// import { useSearchParams } from "react-router-dom";
// import { useEffect, useMemo } from "react";
import useFetchNFTUserCount from "@shared/hooks/useFetchNFTUserCount";
// import CardList from "./CardList";

// const validCollections = ["1G", "10G", "100G", "1KG"];

const UserNFTsSection = ({ className }: { className?: string }) => {
  // const [searchParams, setSearchParams] = useSearchParams();

  // const selectedIndex = useMemo(() => {
  //   const collection = searchParams.get("collection");
  //   if (!collection || !validCollections.includes(collection)) {
  //     return 0;
  //   }
  //   return NFTCollections.findIndex((c) => c.name === collection);
  // }, [searchParams]);

  // useEffect(() => {
  //   const collection = searchParams.get("collection");
  //   if (!collection) {
  //     setSearchParams({ collection: "1G" });
  //   } else if (!validCollections.includes(collection)) {
  //     setSearchParams({ collection: "1G" });
  //   }
  // }, [searchParams, setSearchParams]);

  // const handleTabChange = (index: number) => {
  //   const collection = NFTCollections[index];
  //   if (collection) {
  //     setSearchParams({ collection: collection.name });
  //   }
  // };

  // const selectedCollection = useMemo(() => {
  //   const collection = searchParams.get("collection");
  //   if (!collection || !validCollections.includes(collection)) {
  //     return NFTCollections[0];
  //   }
  //   return (
  //     NFTCollections.find((c) => c.name === collection) || NFTCollections[0]
  //   );
  // }, [searchParams]);

  return (
    <div className={className}>
      <div className="flex items-center justify-between mb-4">
        <div>My NFTs</div>
      </div>
      <div className="grid grid-cols-4 gap-2 xl:gap-4 mb-8 xl:mb-12">
        {NFTCollections.map((collection) => (
          <CollectionCount key={collection.name} collection={collection} />
        ))}
      </div>
      {/* <TabGroup selectedIndex={selectedIndex} onChange={handleTabChange}>
        <TabList className="flex gap-2">
          {NFTCollections.map((collection) => (
            <Tab
              key={collection.name}
              className="rounded-full py-2 px-6 border border-border text-content bg-background cursor-pointer focus:not-data-focus:outline-none data-focus:outline data-focus:outline-gold data-hover:bg-primary data-hover:text-white data-selected:bg-primary data-selected:text-white"
            >
              {collection.name}
            </Tab>
          ))}
        </TabList>
      </TabGroup>
      <CardList canisterId={selectedCollection.canisterId} className="mt-8" /> */}
    </div>
  );
};

const CollectionCount = ({
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
    <div className="border border-border p-4 rounded-xl">
      <div className="flex flex-col xl:flex-row items-center justify-center xl:justify-start gap-2">
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
    </div>
  );
};

export default UserNFTsSection;
