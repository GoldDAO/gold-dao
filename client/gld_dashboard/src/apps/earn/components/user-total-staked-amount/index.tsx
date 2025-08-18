import { UseQueryResult } from "@tanstack/react-query";
import { HTMLAttributes, PropsWithChildren } from "react";
import { useAuth } from "@auth/index";
import TotalCountToken from "@shared/components/total-count-token";
import { TOKEN_GLDT } from "@shared/utils/tokens";

import { Position } from "@earn/interfaces";

interface UserTotalStakedAmountProps
  extends PropsWithChildren<HTMLAttributes<HTMLDivElement>> {
  position: UseQueryResult<Position, Error>;
}

const UserTotalStakedAmount = ({
  position,
  ...props
}: UserTotalStakedAmountProps) => {
  const { isConnected } = useAuth();

  const renderUserTotalStakedAmount = () => {
    if (!isConnected) {
      return <TotalCountToken token={TOKEN_GLDT} amount={0} amountUSD={0} />;
    }
    if (position.isLoading || position.isError) {
      return (
        <TotalCountToken
          token={TOKEN_GLDT}
          isFetching={true}
          amount={0}
          amountUSD={0}
        />
      );
    }
    if (position.isSuccess && position.data) {
      return (
        <TotalCountToken
          token={TOKEN_GLDT}
          amount={position.data.staked_amount}
          amountUSD={position.data.staked_amount_usd}
        />
      );
    }
    return <TotalCountToken token={TOKEN_GLDT} amount={0} amountUSD={0} />;
  };

  return (
    <div {...props}>
      <div className="flex flex-col items-center gap-2">
        <div className="text-2xl font-semibold">Total active stakes</div>
        {renderUserTotalStakedAmount()}
      </div>
    </div>
  );
};

export default UserTotalStakedAmount;
