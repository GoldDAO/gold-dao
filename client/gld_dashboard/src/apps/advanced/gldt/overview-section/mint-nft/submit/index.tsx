import { useAtom, useAtomValue } from "jotai";
import UserNFTSelect from "@shared/components/nft-select/UserNFTSelect";
import { NFTCollections } from "@shared/utils/nfts";
import {
  IsOneOrMoreSelectedNFTAtom,
  TotalGLDTSelectedAtom,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Submit = () => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [selectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const IsOneOrMoreSelectedNFT = useAtomValue(IsOneOrMoreSelectedNFTAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);

  const handleSubmit = () => {
    dispatchSwapNFT({ type: "SUBMIT", value: selectNFTState });
  };

  return (
    <div className="max-h-[80vh] overflow-y-auto pr-2 xl:pr-0">
      <div className="rounded-xl p-4 border border-border">
        <div className="text-copper text-sm font-semibold mb-2">From</div>
        <div className="flex flex-col gap-2">
          {NFTCollections.map((collection) => (
            <UserNFTSelect key={collection.name} collection={collection.name} />
          ))}
        </div>
      </div>

      <div className="mt-4 rounded-xl p-4 border border-border">
        <div className="text-copper text-sm font-semibold mb-2">To</div>
        <div className="flex justify-center items-center p-4 border border-border rounded-xl bg-surface-secondary">
          <div>{totalGLDTSelected} GLDT</div>
        </div>
      </div>

      <div className="mt-8">
        <BtnPrimary
          onClick={handleSubmit}
          disabled={!IsOneOrMoreSelectedNFT}
          className="w-full"
        >
          Submit
        </BtnPrimary>
      </div>
    </div>
  );
};

export default Submit;
