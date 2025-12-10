import { useSearchParams } from "react-router-dom";
import { useEffect, useMemo } from "react";

const NFT_TABS = ["overview", "viewer", "transactions"] as const;

type NFTTab = (typeof NFT_TABS)[number];

const isValidTab = (tab: string | null): tab is NFTTab => {
  return tab !== null && NFT_TABS.includes(tab as NFTTab);
};

const useTabNFT = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const index = useMemo(() => {
    const tab = searchParams.get("tab");
    if (!isValidTab(tab)) {
      return 0;
    }
    return NFT_TABS.findIndex((t) => t === tab);
  }, [searchParams]);

  useEffect(() => {
    const tab = searchParams.get("tab");
    if (!isValidTab(tab)) {
      setSearchParams((prev) => {
        const newParams = new URLSearchParams(prev);
        newParams.set("tab", "overview");
        return newParams;
      });
    }
  }, [searchParams, setSearchParams]);

  const handleChangeTab = (index: number) => {
    const tab = NFT_TABS[index];
    if (tab) {
      setSearchParams((prev) => {
        const newParams = new URLSearchParams(prev);
        newParams.set("tab", tab);
        return newParams;
      });
    }
  };

  const tab = useMemo(() => {
    const tab = searchParams.get("tab");
    if (!isValidTab(tab)) {
      return NFT_TABS[0];
    }
    return NFT_TABS.find((t) => t === tab) || NFT_TABS[0];
  }, [searchParams]);

  return {
    index,
    tab,
    handleChangeTab,
  };
};

export default useTabNFT;
