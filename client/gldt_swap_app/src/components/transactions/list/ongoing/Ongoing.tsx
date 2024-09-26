import { useState } from "react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import clsx from "clsx";

import TxList from "./List";
import { useGetUserActiveSwaps } from "@hooks/gldt_swap";

const PastTransactions = () => {
  const [open, setOpen] = useState(true);

  const active_swap = useGetUserActiveSwaps({
    refetchInterval: 10000,
  });

  return (
    <div className="w-full">
      <div
        className={clsx(
          "flex items-center justify-between bg-surface-2 border border-border px-6 py-4 rounded-xl",
          { "rounded-b-none": open }
        )}
      >
        <div className="flex items-center gap-2">
          <div className="font-semibold">Ongoing</div>
          <div className="bg-gold px-2 text-xs font-semibold text-white py-1 rounded-md text-center">
            {active_swap?.data?.rows.length ?? 0}
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
          <TxList />
        </div>
      )}
    </div>
  );
};

export default PastTransactions;
