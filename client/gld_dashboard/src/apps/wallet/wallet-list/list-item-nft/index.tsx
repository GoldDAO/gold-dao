import clsx from "clsx";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useFetchUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { NFTCollections } from "@shared/utils/nfts";

const PriceNFT = ({ grams = 0 }: { grams: number }) => {
  const { unauthenticatedAgent, isConnected } = useAuth();

  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent && isConnected,
  });

  return (
    <span className="text-content/60">
      {priceGold.isSuccess ? (
        <>
          $
          <NumberToLocaleString value={grams * priceGold.data} />
        </>
      ) : (
        <span className="animate-pulse">($0)</span>
      )}
    </span>
  );
};

const ListItemNFT = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { isConnected, authenticatedAgent, principalId } = useAuth();

  const onClickToken = () => {
    searchParams.set("token", "gldnft");
    setSearchParams(searchParams);
  };

  const nfts = useFetchUserNFTMetrics(authenticatedAgent, {
    owner: principalId,
    nft_collections: NFTCollections,
    enabled: !!authenticatedAgent && isConnected,
  });

  return (
    <div
      className={clsx(
        "shrink-0",
        "rounded-xl border border-border p-2 cursor-pointer",
        `${
          searchParams.get("token") === "gldnft" ? "border-gold bg-gold/10" : ""
        }`
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2">
          <Logo name="gld_nft" className="h-9 w-9" />
          <div className="text-left">
            <div>GLD NFT</div>
            <div className="text-content/60 text-sm font-normal">Gold NFT</div>
          </div>
        </div>
        <div className="text-end">
          <div>
            {nfts.isSuccess ? nfts.data.totalCount : <div>Loading...</div>}
          </div>
          <div className="text-content/60 text-sm flex items-center justify-end gap-1">
            {nfts.isSuccess ? (
              <div>
                {nfts.data.totalGrams} grams -{" "}
                <PriceNFT grams={nfts.data.totalGrams} />
              </div>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ListItemNFT;
