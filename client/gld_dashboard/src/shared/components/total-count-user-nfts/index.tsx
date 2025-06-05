import clsx from "clsx";
import { useAuth } from "@auth/index";
import { NFTCollections } from "@shared/utils/nfts";
import useUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";

const CountNFT = ({
  count = 0,
  className,
}: {
  count: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className={clsx("flex items-center gap-2", "text-2xl xl:text-4xl")}>
        <div className="font-semibold">{count}</div>
        <div className="text-content/60 font-normal">NFTs</div>
      </div>
    </div>
  );
};

const GramsNFT = ({
  grams = 0,
  className,
}: {
  grams: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div>{grams} grams of Gold ($todo)</div>
    </div>
  );
};

const TotalCountUserNFTs = () => {
  const { isConnected, authenticatedAgent, principalId } = useAuth();

  const nfts = useUserNFTMetrics(authenticatedAgent, {
    owner: principalId,
    nft_collections: NFTCollections,
    enabled: !!authenticatedAgent && isConnected,
  });

  const renderCount = () => {
    if (!isConnected) {
      return <CountNFT count={0} />;
    }
    if (nfts.isSuccess) {
      return <CountNFT count={nfts.data.totalCount} />;
    }
    return <CountNFT className="animate-pulse" count={0} />;
  };

  const renderGrams = () => {
    if (!isConnected) {
      return <GramsNFT grams={0} />;
    }
    if (nfts.isSuccess) {
      return <GramsNFT grams={nfts.data.totalGrams} />;
    }
    return <GramsNFT className="animate-pulse" grams={0} />;
  };

  return (
    <div className="flex flex-col items-center gap-2">
      {renderCount()}
      {renderGrams()}
    </div>
  );
};

export default TotalCountUserNFTs;
