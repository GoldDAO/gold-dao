import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";

const BalanceAvailable = ({ token }: { token: Token }) => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.token_from.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_from.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  const renderBalance = () => {
    if (balance.isSuccess) {
      return <NumberToLocaleString value={balance.data.balance} />;
    }
    return (
      <div className="animate-pulse">
        <NumberToLocaleString value={0} />
      </div>
    );
  };

  return (
    <div className="flex items-center gap-2 text-sm text-content/80">
      <Icon.Wallet size={14} />
      <div>{renderBalance()}</div>
      <div>{token.name}</div>
    </div>
  );
};

export default BalanceAvailable;
