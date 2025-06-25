import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import { Logo } from "@components/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import TotalCountToken from "@shared/components/total-count-token";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";
import useFetchPriceGold from "@shared/hooks/useFetchPriceGold";

const Token = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const { id, name } = token;

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    enabled: !!unauthenticatedAgent,
  });

  const tokenPriceOne = useFetchTokenPrice(unauthenticatedAgent, {
    from: name,
    from_canister_id: token.canisterId,
    amount: BigInt(1 * 10 ** (decimals.data ?? 0)),
    enabled: !!unauthenticatedAgent && decimals.isSuccess,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col items-center">
          <div className="flex items-center gap-1">
            <Logo name={id} className="h-6 w-6" />
            <div className="font-semibold text-xl">{name}</div>
          </div>
          <div className="text-xs xl:text-sm text-content/60">
            {tokenPriceOne.isSuccess ? (
              <>
                1 {name} ≈ $
                <NumberToLocaleString value={tokenPriceOne.data.amount_usd} />
              </>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>
        <div className="mt-8 xl:mt-12">
          <TotalCountToken token={token} />
        </div>
      </div>
    </div>
  );
};

const NFT = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent } = useAuth();
  const priceGold = useFetchPriceGold({
    enabled: !!unauthenticatedAgent,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col items-center">
          <div className="flex items-center gap-1">
            <Logo name="gld_nft" className="h-6 w-6" />
            <div className="font-semibold text-xl">GLD NFT</div>
          </div>
          <div className="text-xs xl:text-sm text-content/60">
            {priceGold.isSuccess ? (
              <>
                1 gram Gold ≈ $
                <NumberToLocaleString value={priceGold.data} />
              </>
            ) : (
              <span>Loading...</span>
            )}
          </div>
        </div>
        <div className="mt-8 xl:mt-12">
          <TotalCountUserNFTs />
        </div>
      </div>
    </div>
  );
};

const BalanceHeader = ({ className }: { className?: string }) => {
  const [searchParams] = useSearchParams();

  return (
    <div className={className}>
      {searchParams.get("token") === "nft" ? <NFT /> : <Token />}
    </div>
  );
};

export default BalanceHeader;
