import { usePagination } from "@utils/table/useTable";

import PastTransactions from "@components/transactions/list/past/List";

const TransactionHistoryList = () => {
  const [pagination, setPagination] = usePagination({
    pageSize: 20,
    pageIndex: 0,
  });

  return (
    <div className="container mx-auto mt-4 sm:mt-8 flex flex-col gap-4">
      <div>
        <div className="text-4xl font-semibold text-gold">Transactions</div>
        <div className="text-4xl">History</div>
      </div>

      <div className="border border-border rounded-xl bg-surface p-4 mt-8 mb-16">
        <PastTransactions
          pagination={pagination}
          setPagination={setPagination}
        />
      </div>
    </div>
  );
};

export default TransactionHistoryList;
