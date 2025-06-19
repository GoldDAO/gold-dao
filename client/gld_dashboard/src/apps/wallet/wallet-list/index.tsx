import clsx from "clsx";
import { useAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import { TokensList, Token, GLDT_INDEX } from "@wallet/shared/utils";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import useFetchUserNFTMetrics from "@shared/hooks/useFetchNFTUserMetrics";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { NFTCollections } from "@shared/utils/nfts";

const TokenItem = ({ token }: { token: Token }) => {
  const { id, name, label } = token;
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const [selectedToken, setSelectedToken] = useAtom(TokenSelectedAtom);

  const balance = useFetchLedgerBalance(
    token.canisterId,
    unauthenticatedAgent,
    {
      ledger: name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

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
            ? "border-gold bg-gold/10"
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
            {balance.isSuccess ? (
              <NumberToLocaleString value={balance.data.balance} />
            ) : (
              <div>Loading...</div>
            )}
          </div>
          <div className="text-content/60 text-sm">
            {balance.isSuccess ? (
              <>
                $
                <NumberToLocaleString value={balance.data.balance_usd} />
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
        `${searchParams.get("token") === "nft" ? "border-gold bg-gold/10" : ""}`
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

const DisconnectedPlaceholder = () => {
  return (
    <div className="flex flex-col gap-4 relative">
      {[...Array(2)].map((_, index) => (
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
    <div className="flex flex-col gap-4 pb-4">
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
