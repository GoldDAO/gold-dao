import { useAtomValue } from "jotai";
import { useSearchParams } from "react-router-dom";
import { GLDT_VALUE_1G_NFT } from "@constants";
import { useAuth } from "@auth/index";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTokenPrice from "@hooks/useFetchTokenPrice";
import useUserNFTMetrics from "@hooks/useUserNFTMetrics";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { TokenSelectedAtom } from "@wallet/atoms/WalletAtom";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const Token = ({ className }: { className?: string }) => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const { id, name, label } = token;

  const balance = useFetchUserBalance(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const tokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: name,
    from_canister_id: token.canisterId,
    amount: balance.data ?? 0n,
    enabled: !!unauthenticatedAgent && isConnected && balance.isSuccess,
  });

  const tokenPriceOne = useFetchTokenPrice(unauthenticatedAgent, {
    from: name,
    from_canister_id: token.canisterId,
    amount: BigInt(1 * 10 ** (decimals.data ?? 0)),
    enabled: !!unauthenticatedAgent && isConnected && decimals.isSuccess,
  });

  const renderTokenUserBalance = () => {
    return (
      <div>
        {tokenPrice.isSuccess ? (
          <div className="text-2xl xl:text-4xl font-semibold flex items-center gap-2">
            <TokenValueToLocaleString
              value={tokenPrice.data.amount}
              tokenDecimals={tokenPrice.data.decimals}
            />
            <div className="text-content/60 font-normal">{name}</div>
          </div>
        ) : (
          <div>Loading...</div>
        )}
      </div>
    );
  };

  const renderTokenGLDT = () => {
    return (
      <div className="flex flex-col items-center gap-2">
        {renderTokenUserBalance()}
        <div className="text-sm text-content/60">
          {tokenPrice.isSuccess ? (
            <div>
              <TokenValueToLocaleString
                value={tokenPrice.data.amount / BigInt(GLDT_VALUE_1G_NFT)}
                tokenDecimals={tokenPrice.data.decimals}
              />{" "}
              grams of Gold ({" "}
              <span>
                $
                <NumberToLocaleString value={tokenPrice.data.amount_usd} />
              </span>
              )
            </div>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    );
  };

  const renderToken = () => {
    return (
      <div className="flex flex-col items-center gap-2">
        {renderTokenUserBalance()}
        <div className="text-sm text-content/60">
          {tokenPrice.isSuccess ? (
            <div>
              $
              <NumberToLocaleString value={tokenPrice.data.amount_usd} />
            </div>
          ) : (
            <div>Loading...</div>
          )}
        </div>
      </div>
    );
  };

  const renderTokenHeaderContent = () => {
    switch (id) {
      case "gldt":
        return renderTokenGLDT();
      default:
        return renderToken();
    }
  };

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
        <div className="py-8 xl:py-12">{renderTokenHeaderContent()}</div>
      </div>
    </div>
  );
};

const NFT = ({ className }: { className?: string }) => {
  // const { principalId, authenticatedAgent, isConnected } = useAuth();

  const { data: nfts, isSuccess: isSuccessFetchUserNFTs } = useUserNFTMetrics();

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
          {/* <div className="text-sm text-content/60">1 {name} ≈ $(todo)</div> */}
        </div>
        <div className="py-8 xl:py-12">
          <div className="flex flex-col items-center gap-2">
            <div className="text-2xl xl:text-4xl font-semibold">
              {isSuccessFetchUserNFTs ? (
                <div className="flex items-center gap-2">
                  {nfts.totalCount}
                  <div className="text-content/60 font-normal">NFTs</div>
                </div>
              ) : (
                <div>Loading...</div>
              )}
            </div>
            {isSuccessFetchUserNFTs ? (
              <div>{nfts.totalGrams} grams of Gold ($todo)</div>
            ) : (
              <div>Loading...</div>
            )}
          </div>
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
