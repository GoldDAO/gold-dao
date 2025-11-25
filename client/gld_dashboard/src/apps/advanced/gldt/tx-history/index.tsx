import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";
import { useAuth } from "@auth/index";
import { NFTCollections } from "@shared/utils/nfts";
import List from "./list";
import ListDisconnected from "./list-disconnected";

const TxHistory = ({ className }: { className?: string }) => {
  const { isConnected } = useAuth();

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
              {isConnected ? (
                <List collection={collection} />
              ) : (
                <ListDisconnected />
              )}
            </TabPanel>
          ))}
        </TabPanels>
      </TabGroup>
    </div>
  );
};

export default TxHistory;
