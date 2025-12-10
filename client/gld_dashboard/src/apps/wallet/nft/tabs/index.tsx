import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";
import { useAuth } from "@auth/index";
import TxHistoryNFT from "@wallet/nft/tx-history";
import TxHistoryDisconnected from "@wallet/shared/components/tx-history-disconnected";
import ViewerNFT from "@wallet/nft/viewer";
import useTabNFT from "@wallet/nft/useTabNFT";

const TabTitle = ({ children }: { children: React.ReactNode }) => {
  return (
    <Tab className="py-2 cursor-pointer text-lg text-content/40 data-hover:text-content data-selected:text-content  outline-none focus:outline-none">
      {children}
    </Tab>
  );
};

const Tabs = ({ className }: { className?: string }) => {
  const { isConnected } = useAuth();
  const { index: nftTabIndex, handleChangeTab: handleChangeNFTTab } =
    useTabNFT();

  return (
    <div className={className}>
      <TabGroup selectedIndex={nftTabIndex} onChange={handleChangeNFTTab}>
        <TabList className="flex gap-8 mb-4">
          <TabTitle>My NFTs</TabTitle>
          <TabTitle>Transactions</TabTitle>
        </TabList>
        <TabPanels>
          <TabPanel>
            <ViewerNFT />
          </TabPanel>
          <TabPanel>
            {isConnected ? <TxHistoryNFT /> : <TxHistoryDisconnected />}
          </TabPanel>
        </TabPanels>
      </TabGroup>
    </div>
  );
};

export default Tabs;
