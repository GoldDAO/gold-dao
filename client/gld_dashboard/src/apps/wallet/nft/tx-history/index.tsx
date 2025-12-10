import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";
import { NFTCollections } from "@shared/utils/nfts";
import List from "./list";

const TxHistoryNFT = ({ className }: { className?: string }) => {
  return (
    <div className={className}>
      <TabGroup>
        <TabList className="flex gap-2 mb-4">
          {NFTCollections.map((collection) => (
            <Tab
              key={collection.name}
              className="rounded-full py-2 px-6 border border-border text-content bg-background cursor-pointer focus:not-data-focus:outline-none data-focus:outline data-focus:outline-gold data-hover:text-white data-hover:bg-primary data-selected:bg-primary data-selected:text-white"
            >
              {collection.name}
            </Tab>
          ))}
        </TabList>
        <TabPanels className="mt-3">
          {NFTCollections.map((collection) => (
            <TabPanel key={collection.name}>
              <List collection={collection} />
            </TabPanel>
          ))}
        </TabPanels>
      </TabGroup>
    </div>
  );
};

export default TxHistoryNFT;
