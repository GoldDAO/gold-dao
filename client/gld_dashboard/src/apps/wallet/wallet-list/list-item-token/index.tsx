import clsx from "clsx";
import { useAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import TokenLogo from "@shared/components/token-logo";
import { Token } from "@shared/utils/tokens";
import { amountDisplayDecimals } from "@shared/utils/tokens/custom";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListItemToken = ({ token }: { token: Token }) => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [searchParams, setSearchParams] = useSearchParams();
  const [selectedToken, setSelectedToken] = useAtom(TokenSelectedAtom);

  const balance = useFetchLedgerBalance(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const onClickToken = () => {
    setSelectedToken(token);
    searchParams.set("token", token.display_name);
    setSearchParams(searchParams);
  };

  return (
    <div
      className={clsx(
        "shrink-0",
        "rounded-xl border border-border p-2 cursor-pointer",
        "hover:border-gold hover:bg-gold/10",
        `${
          searchParams.get("token") !== "GLDNFT" &&
          selectedToken.display_name === token.display_name
            ? "border-gold bg-gold/10"
            : ""
        }`
      )}
      onClick={onClickToken}
    >
      <div className="flex justify-between items-center p-2 font-semibold">
        <div className="flex items-center gap-2 min-w-0">
          <TokenLogo token={token} className="h-9 w-9" />
          <div className="text-left min-w-0">
            <div className="truncate">{token.display_name}</div>
            <div className="text-content/60 text-sm font-normal truncate">
              {token.label}
            </div>
          </div>
        </div>
        <div className="text-end shrink-0 pl-2">
          <div>
            {balance.isSuccess ? (
              <NumberToLocaleString
                value={balance.data.balance}
                decimals={amountDisplayDecimals(token, balance.data.balance)}
              />
            ) : (
              <div>{balance.isError ? "Unavailable" : "Loading..."}</div>
            )}
          </div>
          <div className="text-content/60 text-sm">
            {balance.isSuccess ? (
              balance.data.has_price ? (
                <>
                  $
                  <NumberToLocaleString value={balance.data.balance_usd} />
                </>
              ) : (
                <div>No price</div>
              )
            ) : (
              !balance.isError && <div>Loading...</div>
            )}
          </div>
        </div>
      </div>
    </div>
  );
};

export default ListItemToken;
