import { useAtom, useAtomValue } from "jotai";
import { NFTCollections } from "@shared/utils/nfts";
import {
  TotalGLDTSelectedAtom,
  TotalNFTSelectedAtom,
  SelectNFTStateReducerAtom,
} from "@shared/atoms/NFTStateAtom";
import SwapNFTReducerAtom from "../../atoms/SwapNFTAtom";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import { NFTCollectionSection } from "@wallet/nft/shared/NFTSelectionGrid";

const Submit = () => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [selectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);

  const handleSubmit = () => {
    dispatchSwapNFT({ type: "SUBMIT", value: selectNFTState });
  };

  return (
    <>
      <div className="rounded-xl p-4 border border-border">
        <div className="text-copper text-sm font-semibold mb-2">From</div>
        <div>
          {NFTCollections.map((collection) => (
            <NFTCollectionSection
              key={collection.name}
              collectionName={collection.name}
              fetchType="user"
            />
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
          disabled={totalNFTSelected === 0}
          className="w-full"
        >
          Submit
        </BtnPrimary>
      </div>
    </>
  );
};

export default Submit;
