import { usePagination } from "@utils/table/useTable";

import GoBack from "@components/shared/button/GoBack";
import PastTransactions from "@components/transactions/list/past/List";

export const TransactionHistoryList = () => {
  const [pagination, setPagination] = usePagination({
    pageSize: 20,
    pageIndex: 0,
  });

  return (
    <div className="container mx-auto mt-4 sm:mt-8 flex flex-col gap-4">
      <GoBack />
      <div className="my-4">
        <div className="text-4xl font-semibold text-gold">Transactions</div>
        <div className="text-4xl">History</div>
      </div>
      <div className="mt-4 mb-16">
        <PastTransactions
          pagination={pagination}
          setPagination={setPagination}
        />
      </div>
    </div>
  );
};
