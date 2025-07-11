import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";

const HeaderNFT = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent } = useAuth();
  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex items-center gap-1">
          <Logo name="gld_nft" className="h-6 w-6" />
          <div className="font-semibold text-xl">GLD NFT</div>
        </div>
        <div className="text-xs xl:text-sm text-content/60">
          1 gram Gold ≈{" "}
          <span>
            {priceGold.isSuccess ? (
              <>
                $
                <NumberToLocaleString value={priceGold.data} />
              </>
            ) : (
              <span className="animate-pulse">($0)</span>
            )}
          </span>
        </div>
      </div>
    </div>
  );
};

export default HeaderNFT;
