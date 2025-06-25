import clsx from "clsx";
import { useAtom } from "jotai";
import { useSearchParams } from "react-router-dom";
import { useAuth } from "@auth/index";
import { Logo } from "@components/index";
import { Token } from "@wallet/shared/utils";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const ListItemToken = ({ token }: { token: Token }) => {
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
        "hover:border-gold hover:bg-gold/10",
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
          <div className="text-left">
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

export default ListItemToken;
