import { ReactNode, useEffect, useState } from "react";
// import { useNavigate } from "react-router-dom";
import { Tab, TabGroup, TabList, TabPanel, TabPanels } from "@headlessui/react";

import NavbarHome from "@components/shared/navbars/Home";
import { ExplorerTable } from "@components/explorer/ExplorerTable";
import { TopHoldersTable } from "@components/explorer/TopHoldersTable";
import { ExplorerInfo } from "@components/explorer/Info";
import { useFetchTopAccountHolders } from "@hooks/gldt_super_stats_v3";
import { useSearchParams } from "react-router-dom";

enum TAB {
  TRANSACTIONS,
  TOP_HOLDERS,
  INFO,
}

export const Explorer = () => {
  // const navigate = useNavigate();
  const [selectedTabIndex, setSelectedTabIndex] = useState(TAB.TRANSACTIONS);

  useFetchTopAccountHolders();

  const tabs: { name: string; component: ReactNode }[] = [
    { name: "Transactions", component: <ExplorerTable /> },
    {
      name: "Top 100 Holders",
      component: <TopHoldersTable />,
    },
    {
      name: "Info",
      component: <ExplorerInfo />,
    },
  ];

  const [searchParams, setSearchParams] = useSearchParams();

  useEffect(() => {
    if (!searchParams.has("tab")) {
      setSearchParams({ tab: "transactions" });
      setSelectedTabIndex(TAB.TRANSACTIONS);
    } else {
      const currentTab = searchParams.get("tab");
      if (currentTab === "transactions") setSelectedTabIndex(TAB.TRANSACTIONS);
      else if (currentTab === "top_holders")
        setSelectedTabIndex(TAB.TOP_HOLDERS);
      else if (currentTab === "info") setSelectedTabIndex(TAB.INFO);
    }
  }, [searchParams, setSearchParams]);

  const handleOnChangetab = (tab: number) => {
    let newTab = "";
    if (tab === TAB.TRANSACTIONS) newTab = "transactions";
    else if (tab === TAB.TOP_HOLDERS) newTab = "top_holders";
    else if (tab === TAB.INFO) newTab = "info";
    setSearchParams({ tab: newTab });
    setSelectedTabIndex(tab);
  };

  return (
    <>
      <div className="bg-surface-2">
        <NavbarHome />
        <section className="container mx-auto px-4 py-8 xl:py-16">
          <div className="mb-8 lg:mb-16">
            <div className="text-4xl font-semibold text-gold">GLDT</div>
            <div className="text-4xl">Explorer</div>
          </div>

          <TabGroup
            selectedIndex={selectedTabIndex}
            onChange={(tab) => handleOnChangetab(tab)}
          >
            <TabList className="inline-flex gap-4">
              {tabs.map(({ name }) => (
                <Tab
                  key={name}
                  className="rounded-full py-2 px-3 lg:px-4 text-sm lg:text-sm/6 font-semibold bg-content/5 text-content focus:outline-none data-[selected]:bg-content data-[selected]:text-background data-[hover]:bg-gold/20 data-[selected]:data-[hover]:bg-content data-[focus]:outline-1 data-[focus]:outline-white"
                >
                  {name}
                </Tab>
              ))}
            </TabList>
            <TabPanels className="mt-6">
              {tabs.map(({ name, component }) => (
                <TabPanel key={name}>{component}</TabPanel>
              ))}
            </TabPanels>
          </TabGroup>
        </section>
      </div>
    </>
  );
};
