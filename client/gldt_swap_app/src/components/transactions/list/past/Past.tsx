import { useState } from "react";
import { useNavigate } from "react-router-dom";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import clsx from "clsx";

import { usePagination } from "@utils/table/useTable";
import TxList from "./List";
import { useGetUserHistoricCountSwap } from "@hooks/gldt_swap";

const PastTransactions = () => {
  const navigate = useNavigate();
  const [pagination] = usePagination({ pageIndex: 0, pageSize: 5 });
  const [open, setOpen] = useState(true);

  const handleShowAllTxs = () => {
    navigate("/swap/account/transactions");
  };

  const count = useGetUserHistoricCountSwap({ refetchInterval: 10000 });

  return (
    <div className="w-full">
      <div
        className={clsx(
          "flex items-center justify-between bg-surface-2 border border-border px-6 py-4 rounded-xl",
          { "rounded-b-none": open }
        )}
      >
        <div className="flex items-center gap-2">
          <div className="font-semibold">Past</div>
          <div className="bg-gold px-2 text-xs text-white font-semibold py-1 rounded-md text-center">
            {count.data}
          </div>
          <div className="flex justify-center items-center shrink-0 rounded-md text-gold border border-gold bg-surface-2 hover:px-4 ml-4 px-3 transition-all duration-500">
            <button onClick={handleShowAllTxs} className="font-semibold">
              Show all
            </button>
          </div>
        </div>
        <button onClick={() => setOpen(!open)}>
          <ChevronDownIcon
            className={clsx("w-5 h-5 transition-transform duration-300", {
              "rotate-180": open,
            })}
          />
        </button>
      </div>
      {open && (
        <div
          className={clsx(
            "bg-surface p-4 text-sm border-x border-b border-t-none border-border rounded-b-xl"
          )}
        >
          <TxList pagination={pagination} />
        </div>
      )}
    </div>
  );
};

export default PastTransactions;
