import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";

const BalanceAvailable = ({
  token,
  balance,
}: {
  token: string;
  balance: number | undefined;
}) => {
  const renderBalance = () => {
    if (balance !== undefined) {
      return <NumberToLocaleString value={balance} />;
    }
    return (
      <div className="animate-pulse">
        <NumberToLocaleString value={0} />
      </div>
    );
  };

  return (
    <div className="flex items-center gap-1">
      <Icon.Wallet size={14} />
      <div>{renderBalance()}</div>
      <div>{token}</div>
    </div>
  );
};

export default BalanceAvailable;
