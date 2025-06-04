import { usePagination } from "@shared/utils/table/useTable";
import { GoldDAONeuronsTable } from "./table/GoldDAONeuronsTable";

const GoldDAONeurons = () => {
  const [pagination, setPagination] = usePagination({
    pageIndex: 0,
    pageSize: 10,
  });
  return (
    <>
      <div className="mt-4 sm:mt-8 mb-8">
        <div className="text-4xl font-bold text-accent">Gold DAO</div>
        <div className="text-4xl">Neurons</div>
      </div>

      <section>
        <GoldDAONeuronsTable
          pagination={pagination}
          setPagination={setPagination}
        />
      </section>
    </>
  );
};

export default GoldDAONeurons;
