import clsx from "clsx";
import { useAuth } from "@auth/index";
import { NFTCollections } from "@shared/utils/nfts";
import useFetchNFTUserMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const CountNFT = ({
  count = 0,
  className,
}: {
  count: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className={clsx("flex items-center gap-2", "text-3xl xl:text-4xl")}>
        <div className="font-semibold">{count}</div>
        <div className="text-content/60 font-normal">NFTs</div>
      </div>
    </div>
  );
};

const PriceNFT = ({
  grams = 0,
  className,
}: {
  grams: number;
  className?: string;
}) => {
  const { unauthenticatedAgent, isConnected } = useAuth();

  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent && isConnected,
  });

  return (
    <div className={className}>
      <div className="text-lg">
        <span>{grams} grams of Gold </span>
        <span className="text-content/60">
          {priceGold.isSuccess ? (
            <>
              ($
              <NumberToLocaleString value={grams * priceGold.data} />)
            </>
          ) : (
            <span className="animate-pulse">($0)</span>
          )}
        </span>
      </div>
    </div>
  );
};

const TotalCountUserNFTs = ({ className }: { className?: string }) => {
  const { isConnected, unauthenticatedAgent, principalId } = useAuth();

  const nfts = useFetchNFTUserMetrics({
    owner: principalId,
    nft_collections: NFTCollections,
    enabled: !!unauthenticatedAgent && isConnected,
    agent: unauthenticatedAgent,
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
      return <PriceNFT grams={0} />;
    }
    if (nfts.isSuccess) {
      return <PriceNFT grams={nfts.data.totalGrams} />;
    }
    return <PriceNFT className="animate-pulse" grams={0} />;
  };

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        {renderCount()}
        {renderGrams()}
      </div>
    </div>
  );
};

export default TotalCountUserNFTs;
