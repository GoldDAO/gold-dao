import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import { Logo } from "@components/index";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import TotalCountToken from "@shared/components/total-count-token";
import TotalCountUserNFTs from "@shared/components/total-count-user-nfts";

const Token = ({ className }: { className?: string }) => {
  const { unauthenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const { id, name, label } = token;

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const tokenPriceOne = useFetchTokenPrice(unauthenticatedAgent, {
    from: name,
    from_canister_id: token.canisterId,
    amount: BigInt(1 * 10 ** (decimals.data ?? 0)),
    enabled: !!unauthenticatedAgent && isConnected && decimals.isSuccess,
  });

  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name={id} className="h-10 w-10" />
            <div>
              <div>{name}</div>
              <div className="text-content/60 text-sm">{label}</div>
            </div>
          </div>
          <div className="text-sm text-content/60">
            {tokenPriceOne.isSuccess ? (
              <>
                1 {name} ≈ $
                <NumberToLocaleString value={tokenPriceOne.data.amount_usd} />
              </>
            ) : (
              <span>Loading...</span>
            )}
          </div>
        </div>
        <div className="py-8 xl:py-12">
          <TotalCountToken token={token} />
        </div>
      </div>
    </div>
  );
};

const NFT = ({ className }: { className?: string }) => {
  return (
    <div className={className}>
      <div className="flex flex-col items-center">
        <div className="flex flex-col gap-2 items-center">
          <div className="flex items-center gap-2">
            <Logo name="gld_nft" className="h-10 w-10" />
            <div>
              <div>GLD NFT</div>
              <div className="text-content/60 text-sm">GLD NFT</div>
            </div>
          </div>
        </div>
        <div className="py-8 xl:py-12">
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
