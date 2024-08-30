import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
} from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";

import { usePagination } from "@utils/table/useTable";
import TxList from "./List";

const PastTransactions = () => {
  const [pagination] = usePagination({ pageIndex: 0, pageSize: 5 });

  return (
    <div>
      <div className="w-full divide-y divide-white/5 border border-border rounded-xl">
        <Disclosure as="div" defaultOpen={true}>
          <div className="flex items-center justify-between bg-surface-2 px-6 py-4 rounded-xl group-data-[open]:rounded-b-none">
            <div className="font-medium group-data-[hover]:text-content/80">
              Past transactions
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
