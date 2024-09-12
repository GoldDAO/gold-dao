import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
} from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";

import TxList from "./List";
import { useGetUserActiveSwaps } from "@hooks/gldt_swap";

const PastTransactions = () => {


  const active_swap = useGetUserActiveSwaps({
    refetchInterval: 10000,
  });

  return (
    <div>
      <div className="w-full divide-y divide-white/5 border border-border rounded-xl">
        <Disclosure as="div" defaultOpen={true}>
          <DisclosureButton className="group w-full">
            <div className="flex items-center justify-between bg-surface-2 px-6 py-4 rounded-xl group-data-[open]:rounded-b-none">
              <div className="font-medium group-data-[hover]:text-content/80 align-center flex">
                <span className="mr-2">Ongoing transactions</span> <span className="bg-gold pl-2 pr-2 text-xs py-1 rounded-md text-center">{active_swap?.data?.rows.length}</span>
              </div>
              <ChevronDownIcon className="size-5 group-data-[hover]:fill-content/50 group-data-[open]:rotate-180" />
            </div>
          </DisclosureButton>
          <DisclosurePanel className="bg-surface p-4 text-sm/5 border-t border-border rounded-b-xl">
          <div id="active-swaps">
            <TxList />
            </div>
          </DisclosurePanel>
        </Disclosure>
      </div>
    </div>
  );
};

export default PastTransactions;
