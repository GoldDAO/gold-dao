import clsx from "clsx";
import { useAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { TokensList, Token, GLDT_INDEX } from "@wallet/shared/utils";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import useFetchTokenPrice from "@shared/hooks/useFetchTokenPrice";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import { NFTCollections } from "@shared/utils/nfts";

const TokenItem = ({ token }: { token: Token }) => {
  const { id, name, label } = token;
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const [selectedToken, setSelectedToken] = useAtom(TokenSelectedAtom);

  const balance = useFetchUserBalance(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const tokenPrice = useFetchTokenPrice(unauthenticatedAgent, {
    from: name,
    from_canister_id: token.canisterId,
    amount: balance.data ?? 0n,
    enabled: !!unauthenticatedAgent && isConnected && balance.isSuccess,
  });

  const onClickToken = () => {
    setSelectedToken(token);
    searchParams.set("token", token.id);
    setSearchParams(searchParams);
  };

  return (
    <div
      className={clsx(
        "shrink-0",
        "rounded-xl border border-border p-2 cursor-pointer",
        `${
          searchParams.get("token") !== "nft" && selectedToken.id === id
            ? "border-primary bg-primary/10"
            : ""
        }`
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2">
          <Logo name={id} className="h-9 w-9" />
          <div>
            <div>{name}</div>
            <div className="text-content/60 text-sm font-normal">{label}</div>
          </div>
        </div>
        <div className="text-end">
          <div>
            {tokenPrice.isSuccess ? (
              <TokenValueToLocaleString
                value={tokenPrice.data.amount}
                tokenDecimals={tokenPrice.data.decimals}
              />
            ) : (
              <div>Loading...</div>
            )}
          </div>
          <div className="text-content/60 text-sm">
            {tokenPrice.isSuccess ? (
              <>
                $
                <NumberToLocaleString value={tokenPrice.data.amount_usd} />
              </>
            ) : (
              <div>Loading...</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

const NFTItem = () => {
  const [searchParams, setSearchParams] = useSearchParams();
  const { isConnected, authenticatedAgent, principalId } = useAuth();

  const onClickToken = () => {
    searchParams.set("token", "nft");
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
          searchParams.get("token") === "nft"
            ? "border-primary bg-primary/10"
            : ""
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
                <NumberToLocaleString value={nfts.data.totalUSD} decimals={2} />
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

const DisconnectedPlaceholder = () => {
  return (
    <div className="flex xl:flex-grow flex-row xl:flex-col xl:h-100 pb-4 overflow-hidden gap-4 relative">
      {[...Array(4)].map((_, index) => (
        <div key={index}>
          <div
            className={clsx(
              "lg:@container",
              "shrink-0",
              "rounded-xl border border-surface-secondary p-4 cursor-pointer"
            )}
          >
            <div className="flex justify-between items-center p-2">
              <div className="flex items-center gap-2">
                <div className="h-10 w-10 bg-surface-secondary rounded-full" />
                <div className="flex flex-col gap-1">
                  <div className="h-5 w-[16cqw] bg-surface-secondary rounded-sm" />
                  <div className="h-4 w-[20cqw] bg-surface-secondary rounded-sm" />
                </div>
              </div>
              <div className="flex flex-col gap-1 items-end">
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
                <div className="h-4 w-[16cqw] bg-surface-secondary rounded-sm" />
              </div>
            </div>
          </div>
        </div>
      ))}
      <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-surface-primary to-transparent" />
    </div>
  );
};

const WalletList = () => {
  const { isConnected } = useAuth();

  return (
    <div className="flex xl:flex-grow flex-row xl:flex-col xl:h-100 pb-4 xl:overflow-y-auto overflow-x-auto xl:overflow-x-hidden gap-4 xl:pr-4">
      {isConnected ? (
        <>
          <TokenItem
            token={TokensList[GLDT_INDEX]}
            key={TokensList[GLDT_INDEX].id}
          />
          <NFTItem />
          {TokensList.slice(1).map((token) => (
            <TokenItem token={token} key={token.id} />
          ))}
        </>
      ) : (
        <DisconnectedPlaceholder />
      )}
    </div>
  );
};

export default WalletList;
