import clsx from "clsx";
import { useSetAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import { TokensList, Token, GLDT_INDEX } from "../utils";
import { TokenSelectedAtom } from "../atoms";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useUserNFTMetrics from "../../../hooks/useUserNFTMetrics";

const TokenItem = ({ token }: { token: Token }) => {
  const { id, name, label } = token;
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const setSelectedToken = useSetAtom(TokenSelectedAtom);

  const balance = useFetchUserBalance(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    owner: principalId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const decimals = useFetchDecimals(token.canisterId, unauthenticatedAgent, {
    ledger: id,
    enabled: !!unauthenticatedAgent && isConnected,
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
        "rounded-xl border border-border p-4 cursor-pointer"
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2">
          <Logo name={id} className="h-10 w-10" />
          <div>
            <div>{name}</div>
            <div className="text-content/60 text-sm">{label}</div>
          </div>
        </div>
        <div className="text-end">
          <div>
            {balance.isSuccess && decimals.isSuccess ? (
              <TokenValueToLocaleString
                value={balance.data}
                tokenDecimals={decimals.data}
              />
            ) : (
              <div>Loading...</div>
            )}
          </div>
          <div className="text-content/60 text-sm">$todo</div>
        </div>
      </div>
    </div>
  );
};

const NFTItem = () => {
  const [searchParams, setSearchParams] = useSearchParams();

  const onClickToken = () => {
    searchParams.set("token", "nft");
    setSearchParams(searchParams);
  };

  const { data: nfts, isSuccess: isSuccessFetchUserNFTs } = useUserNFTMetrics();

  return (
    <div
      className={clsx(
        "shrink-0",
        "rounded-xl border border-border p-4 cursor-pointer"
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2">
          <Logo name="gld_nft" className="h-10 w-10" />
          <div>
            <div>GLD NFT</div>
            <div className="text-content/60 text-sm">GLD NFT</div>
          </div>
        </div>
        <div className="text-end">
          <div>
            {isSuccessFetchUserNFTs ? nfts.totalCount : <div>Loading...</div>}
          </div>
          <div className="text-content/60 text-sm flex items-center justify-end gap-1">
            {isSuccessFetchUserNFTs ? (
              <div>({nfts.totalGrams} grams) - $todo</div>
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
              "@container",
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
    <>
      {isConnected ? (
        <div className="flex lg:flex-grow flex-row lg:flex-col lg:h-100 pb-4 lg:overflow-y-auto overflow-x-auto lg:overflow-x-hidden gap-4 lg:pr-4">
          <TokenItem
            token={TokensList[GLDT_INDEX]}
            key={TokensList[GLDT_INDEX].id}
          />
          <NFTItem />
          {TokensList.slice(1).map((token) => (
            <TokenItem token={token} key={token.id} />
          ))}
        </div>
      ) : (
        <DisconnectedPlaceholder />
      )}
    </>
  );
};

export default WalletList;
