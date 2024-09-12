import { useNavigate } from "react-router-dom";
import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
} from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";

import { usePagination } from "@utils/table/useTable";
import TxList from "./List";
import { useGetUserHistoricCountSwap } from "@hooks/gldt_swap";

const PastTransactions = () => {
  const navigate = useNavigate();
  const [pagination] = usePagination({ pageIndex: 0, pageSize: 5 });

  const handleShowAllTxs = () => {
    navigate("/swap/account/transactions");
  };

  const count = useGetUserHistoricCountSwap({refetchInterval: 2000});

  return (
    <div>
      <div className="w-full divide-y divide-white/5 border border-border rounded-xl">
        <Disclosure as="div" defaultOpen={true}>
          <div className="flex items-center justify-between bg-surface-2 px-6 py-4 rounded-xl group-data-[open]:rounded-b-none">
            <div className="flex items-center gap-4">
              <div className="font-medium group-data-[hover]:text-content/80 align-center flex">
              <span className="mr-2">Past transactions</span> <span className="bg-gold pl-2 pr-2 text-xs py-1 rounded-md text-center">{count.data}</span>
              </div>
              <button onClick={handleShowAllTxs}>Show all</button>
            </div>
            <DisclosureButton className="group">
              <ChevronDownIcon className="size-5 group-data-[hover]:fill-content/50 group-data-[open]:rotate-180" />
            </DisclosureButton>
          </div>
          <DisclosurePanel className="bg-surface p-4 text-sm/5 border-t border-border rounded-b-xl">
            <TxList pagination={pagination} />
          </DisclosurePanel>
        </Disclosure>
      </div>
    </div>
  );
};

export default PastTransactions;
