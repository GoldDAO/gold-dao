import Icon from "@shared/ui/icons";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { ReactNode } from "react";

const BalanceAvailable = ({
  token,
  balance,
  icon = <Icon.Wallet size={14} />,
}: {
  token: string;
  balance: number | undefined;
  icon?: ReactNode;
}) => {
  const renderBalance = () => {
    if (balance !== undefined) {
      return <NumberToLocaleString value={balance} decimals={5} />;
    }
    return (
      <div className="animate-pulse">
        <NumberToLocaleString value={0} />
      </div>
    );
  };

  return (
    <div className="flex items-center gap-1">
      {icon}
      <div>{renderBalance()}</div>
      <div>{token}</div>
    </div>
  );
};

export default BalanceAvailable;
