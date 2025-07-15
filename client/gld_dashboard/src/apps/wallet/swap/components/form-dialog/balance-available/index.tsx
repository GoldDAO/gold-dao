import { useAuth } from "@auth/index";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { Token } from "@shared/utils/tokens";

const BalanceAvailable = ({ token }: { token: Token }) => {
  const { unauthenticatedAgent, principalId } = useAuth();

  const balance = useFetchLedgerBalance(
    token.canister_id,
    unauthenticatedAgent,
    {
      ledger: token.name,
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
