import clsx from "clsx";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import useFetchUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { NFTCollections } from "@shared/utils/nfts";

const ListItemNFT = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { isConnected, unauthenticatedAgent, principalId } = useAuth();

  const onClickToken = () => {
    searchParams.set("token", "GLDNFT");
    setSearchParams(searchParams);
  };

  const nfts = useFetchUserNFTMetrics({
    owner: principalId,
    nft_collections: NFTCollections,
    enabled: !!unauthenticatedAgent && isConnected,
    agent: unauthenticatedAgent,
  });

  return (
    <div
      className={clsx(
        "shrink-0",
        "rounded-xl border border-border p-2 cursor-pointer",
        `${
          searchParams.get("token") === "GLDNFT" ? "border-gold bg-gold/10" : ""
        }`
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2">
          <Logo name="gld_nft" className="h-9 w-9" />
          <div>
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
                {nfts.data.totalGrams} grams - $
                <NumberToLocaleString value={nfts.data.totalUSD} />
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
