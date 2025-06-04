import { useAtomValue } from "jotai";
import {
  Disclosure,
  DisclosureButton,
  DisclosurePanel,
} from "@headlessui/react";
import { ChevronDownIcon } from "@heroicons/react/20/solid";
import {
  TotalGLDTSelectedAtom,
  TotalGramSelectedAtom,
  TotalNFTSelectedAtom,
  CollectionSelectedAtom,
} from "@shared/atoms/NFTStateAtom";

const TransactionDetails = ({
  className = "",
  defaultOpen = false,
}: {
  className?: string;
  defaultOpen?: boolean;
}) => {
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const totalGramSelected = useAtomValue(TotalGramSelectedAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const collectionSelected = useAtomValue(CollectionSelectedAtom);

  return (
    <div className={className}>
      <Disclosure as="div" defaultOpen={defaultOpen}>
        <div className="flex items-center justify-between px-2 rounded-lg group-data-[open]:rounded-b-none">
          <div className="flex items-center gap-4">
            <div className="font-medium group-data-[hover]:text-content/80">
              Transaction details
            </div>
          </div>
          <DisclosureButton className="group">
            <ChevronDownIcon className="size-5 group-data-[hover]:fill-content/50 group-data-[open]:rotate-180" />
          </DisclosureButton>
        </div>
        <DisclosurePanel className="bg-surface text-sm/5 mt-4">
          <div className="flex flex-col gap-4 border border-border bg-surface-secondary p-6 rounded-xl">
            <div className="flex flex-col sm:flex-row items-center justify-center sm:justify-between text-content/60">
              <div className="">Amount to be minted</div>
              <div>{totalGLDTSelected} GLDT</div>
            </div>
            <div className="border-b border-border"></div>
            <div className="flex flex-col sm:flex-row items-center justify-center sm:justify-between text-content/60 font-semibold">
              <div>Total number of NFTs to be swapped</div>
              <div>{totalNFTSelected} GLD NFT</div>
            </div>
            {collectionSelected.map(
              ({ value, total_count_selected }, index) => (
                <div
                  key={index}
                  className="flex flex-col sm:flex-row items-center justify-center sm:justify-between text-content/60"
                >
                  <div>{value}g GLD NFT</div>
                  <div>{total_count_selected}x</div>
                </div>
              )
            )}
            <div className="flex flex-col sm:flex-row items-center justify-center sm:justify-between text-content/60 font-semibold">
              <div>Total grams of gold</div>
              <div>{totalGramSelected}g</div>
            </div>
            <div className="mt-4 flex flex-col sm:flex-row items-center justify-center sm:justify-between font-semibold">
              <div>Total</div>
              <div>{totalGLDTSelected} GLDT</div>
            </div>
          </div>
        </DisclosurePanel>
      </Disclosure>
    </div>
  );
};

export default TransactionDetails;
