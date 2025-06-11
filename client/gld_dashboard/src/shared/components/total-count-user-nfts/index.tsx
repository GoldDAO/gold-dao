import clsx from "clsx";
import { useAuth } from "@auth/index";
import { NFTCollections } from "@shared/utils/nfts";
import useFetchNFTUserMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

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

const PriceNFT = ({
  grams = 0,
  priceUSD = 0,
  className,
}: {
  grams: number;
  priceUSD: number;
  className?: string;
}) => {
  return (
    <div className={className}>
      <div className="text-sm text-content/60">
        {grams} grams of Gold ($
        <NumberToLocaleString value={priceUSD} decimals={2} />)
      </div>
    </div>
  );
};

const TotalCountUserNFTs = () => {
  const { isConnected, authenticatedAgent, principalId } = useAuth();

  const nfts = useFetchNFTUserMetrics(authenticatedAgent, {
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
      return <PriceNFT grams={0} priceUSD={0} />;
    }
    if (nfts.isSuccess) {
      return (
        <PriceNFT grams={nfts.data.totalGrams} priceUSD={nfts.data.totalUSD} />
      );
    }
    return <PriceNFT className="animate-pulse" grams={0} priceUSD={0} />;
  };

  return (
    <div className="flex flex-col items-center gap-2">
      {renderCount()}
      {renderGrams()}
    </div>
  );
};

export default TotalCountUserNFTs;
