import { useEffect, useState } from "react";

import { useAuth } from "@auth/index";
import useFetchUserNFT from "@services/gld_nft/hooks/useFetchUserNFT";
import {
  GLD_NFT_1000G_CANISTER_ID,
  GLD_NFT_100G_CANISTER_ID,
  GLD_NFT_10G_CANISTER_ID,
  GLD_NFT_1G_CANISTER_ID,
} from "@constants";

const useFetchUserNFTs = () => {
  const { principalId, authenticatedAgent, isConnected } = useAuth();

  const [totalGrams, setTotalGrams] = useState<number | undefined>(undefined);
  const [totalCount, setTotalCount] = useState<number | undefined>(undefined);
  const [isSuccess, setIsSuccess] = useState<boolean>(false);

  const collectionNFT1G = useFetchUserNFT(
    GLD_NFT_1G_CANISTER_ID,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: "1G",
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const collectionNFT10G = useFetchUserNFT(
    GLD_NFT_10G_CANISTER_ID,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: "10G",
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const collectionNFT100G = useFetchUserNFT(
    GLD_NFT_100G_CANISTER_ID,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: "100G",
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  const collectionNFT1KG = useFetchUserNFT(
    GLD_NFT_1000G_CANISTER_ID,
    authenticatedAgent,
    {
      owner: principalId,
      collectionName: "1KG",
      enabled: !!authenticatedAgent && !!isConnected,
    }
  );

  useEffect(() => {
    if (
      collectionNFT1G.isFetched &&
      collectionNFT10G.isFetched &&
      collectionNFT100G.isFetched &&
      collectionNFT1KG.isFetched
    ) {
      const totalGrams1G = collectionNFT1G.isSuccess
        ? collectionNFT1G.data.length * 1
        : 0;
      const totalGrams10G = collectionNFT10G.isSuccess
        ? collectionNFT10G.data.length * 10
        : 0;
      const totalGrams100G = collectionNFT100G.isSuccess
        ? collectionNFT100G.data.length * 100
        : 0;
      const totalGrams1KG = collectionNFT1KG.isSuccess
        ? collectionNFT1KG.data.length * 1000
        : 0;

      const totalCount1G = collectionNFT1G.isSuccess
        ? collectionNFT1G.data.length
        : 0;
      const totalCount10G = collectionNFT10G.isSuccess
        ? collectionNFT10G.data.length
        : 0;
      const totalCount100G = collectionNFT100G.isSuccess
        ? collectionNFT100G.data.length
        : 0;
      const totalCount1KG = collectionNFT1KG.isSuccess
        ? collectionNFT1KG.data.length
        : 0;

      setTotalGrams(
        totalGrams1G + totalGrams10G + totalGrams100G + totalGrams1KG
      );
      setTotalCount(
        totalCount1G + totalCount10G + totalCount100G + totalCount1KG
      );
      setIsSuccess(true);
    }
  }, [
    collectionNFT1G.isSuccess,
    collectionNFT10G.isSuccess,
    collectionNFT100G.isSuccess,
    collectionNFT1KG.isSuccess,
    collectionNFT1G.data,
    collectionNFT10G.data,
    collectionNFT100G.data,
    collectionNFT1KG.data,
    collectionNFT1G.isFetched,
    collectionNFT10G.isFetched,
    collectionNFT100G.isFetched,
    collectionNFT1KG.isFetched,
  ]);

  return {
    data: {
      totalCount,
      totalGrams,
    },
    isSuccess,
  };
};

export default useFetchUserNFTs;
