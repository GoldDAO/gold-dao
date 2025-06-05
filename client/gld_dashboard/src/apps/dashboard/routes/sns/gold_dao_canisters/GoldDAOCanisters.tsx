import { usePagination } from "@shared/utils/table/useTable";
import { GoldDAOCanistersTable } from "./table/GoldDAOCanistersTable";

const GoldDAOCanisters = () => {
  const [pagination, setPagination] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Canisters</div>
      </div>

      <section>
        <GoldDAOCanistersTable
          pagination={pagination}
          setPagination={setPagination}
        />
      </section>
    </>
  );
};

export default GoldDAOCanisters;
