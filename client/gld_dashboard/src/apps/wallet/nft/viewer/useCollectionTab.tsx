import { useState, useMemo } from "react";
import { NFTCollections } from "@shared/utils/nfts";

const useCollectionTab = () => {
  const [index, setIndex] = useState(0);

  const handleChangeTab = (newIndex: number) => {
    if (newIndex >= 0 && newIndex < NFTCollections.length) {
      setIndex(newIndex);
    }
  };

  const collection = useMemo(() => {
    return NFTCollections[index] || NFTCollections[0];
  }, [index]);

  return {
    index,
    collection,
    handleChangeTab,
  };
};

export default useCollectionTab;
