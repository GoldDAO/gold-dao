import { LoaderSpin } from "@components/ui";
import { useTransactionDetails } from "@context/index";

import TransactionStatus from "@components/transactions/badge/TransactionStatus";

const HeaderDetails = ({ className }: { className?: string }) => {
  const { isSuccess, data, isLoading, isError } = useTransactionDetails();

  return (
    <div className={`${className}`}>
      {isSuccess && data && (
        <div className="flex justify-between items-center">
          <div>
            <div className="text-4xl font-semibold text-gold">Transaction</div>
            <div className="text-4xl">Details</div>
          </div>
          <div>
            <TransactionStatus status={data.status.label} />
          </div>
        </div>
      )}
      {(isLoading || isError) && (
        <div className="flex justify-center">
          <LoaderSpin />
        </div>
      )}
    </div>
  );
};

export default HeaderDetails;
