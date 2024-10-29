import TransactionStatus from "@components/transactions/badge/TransactionStatus";
import { SwapData } from "@canisters/gldt_swap/interfaces";

const HeaderDetails = ({
  data,
  className,
}: {
  className?: string;
  data: SwapData;
}) => {
  return (
    <div className={`${className}`}>
      <div className="flex justify-between items-center">
        <div>
          <div className="text-4xl font-semibold text-gold">Transaction</div>
          <div className="text-4xl">Details</div>
        </div>
        <div>
          <TransactionStatus status={data.status.label} />
        </div>
      </div>
    </div>
  );
};

export default HeaderDetails;
