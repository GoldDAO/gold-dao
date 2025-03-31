import { usePagination } from "@utils/table/useTable";
import { GLDGovTransactionsTable } from "./table/GLDGovTransactionsTable";

const GLDGovTransactions = () => {
  const [pagination, setPagination] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">GLDGov</div>
        <div className="text-4xl">Transactions</div>
      </div>

      <section className="">
        <GLDGovTransactionsTable
          pagination={pagination}
          setPagination={setPagination}
        />
      </section>
    </>
  );
};

export default GLDGovTransactions;
