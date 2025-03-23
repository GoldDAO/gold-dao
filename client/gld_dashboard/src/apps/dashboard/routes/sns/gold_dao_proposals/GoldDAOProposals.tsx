import { usePagination } from "@utils/table/useTable";
import { GoldDAOProposalsTable } from "./table/GoldDAOProposalsTable";

const GoldDAOProposals = () => {
  const [pagination, setPagination] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Proposals</div>
      </div>

      <section>
        <GoldDAOProposalsTable
          pagination={pagination}
          setPagination={setPagination}
        />
      </section>
    </>
  );
};

export default GoldDAOProposals;
