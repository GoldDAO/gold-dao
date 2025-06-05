import { useAtom, useAtomValue } from "jotai";
import { Button } from "@components/index";
import UserNFTSelect from "@shared/components/nft-select/UserNFTSelect";
import { NFTCollections } from "@shared/utils/nfts";
import {
  IsOneOrMoreSelectedNFTAtom,
  TotalGLDTSelectedAtom,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";

const Submit = () => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [selectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const IsOneOrMoreSelectedNFT = useAtomValue(IsOneOrMoreSelectedNFTAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);

  const handleSubmit = () => {
    dispatchSwapNFT({ type: "SUBMIT", value: selectNFTState });
  };

  return (
    <div>
      <div className="rounded-xl p-4 border border-border">
        <div className="text-primary mb-4">From</div>
        <div className="flex flex-col gap-2">
          {NFTCollections.map((collection) => (
            <UserNFTSelect key={collection.name} collection={collection.name} />
          ))}
        </div>
      </div>

      <div className="mt-8 rounded-xl p-4 border border-border">
        <div className="text-primary mb-4">To</div>
        <div className="flex justify-center items-center p-4 border border-border rounded-xl bg-surface-secondary">
          <div>{totalGLDTSelected} GLDT</div>
        </div>
      </div>

      <div className="mt-8">
        <Button
          onClick={handleSubmit}
          disabled={!IsOneOrMoreSelectedNFT}
          className="w-full px-6 py-3 bg-secondary text-white font-medium rounded-md"
        >
          Submit
        </Button>
      </div>
    </div>
  );
};

export default Submit;
