import { useAtom, useAtomValue } from "jotai";
import { Button } from "@components/index";
import {
  SelectNFTStateReducerAtom,
  TotalGLDTSelectedAtom,
  TotalNFTSelectedAtom,
  TotalGramSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import TransactionDetails from "@advanced/gldt/overview-section/burn-nft/transaction-details";
import { ArrowDownIcon } from "@heroicons/react/20/solid";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";

const Confirm = () => {
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const totalGramSelectedAtom = useAtomValue(TotalGramSelectedAtom);

  const handleConfirm = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchSwapNFT({ type: "CONFIRM" });
  };

  return (
    <div>
      <div className="mt-8 flex flex-col gap-8">
        <div className="text-center">
          You are sending{" "}
          <span className="font-semibold text-primary">
            <NumberToLocaleString
              value={totalGLDTSelected + totalNFTSelected}
            />{" "}
            GLDT
          </span>{" "}
          and will receive{" "}
          <span className="font-semibold text-primary">
            {totalGramSelectedAtom}g GLD NFTs
          </span>
          . <br />
          <span className="font-semibold text-primary">
            <NumberToLocaleString value={totalGLDTSelected} /> GLDT
          </span>{" "}
          will be burned and{" "}
          <span className="font-semibold text-primary">
            {totalNFTSelected} GLDT
          </span>{" "}
          fee are charged.
        </div>
        <div className="flex flex-col items-center gap-6 border border-border bg-surface-secondary p-6 rounded-xl">
          <div className="font-semibold">
            <NumberToLocaleString
              value={totalGLDTSelected + totalNFTSelected}
            />{" "}
            GLDT
          </div>

          <div className="w-full flex justify-center items-center py-4">
            <div className="relative w-full">
              <div className="border-t border-border w-full"></div>
              <div className="absolute inset-x-0 top-0 flex justify-center transform -translate-y-1/2">
                <button className="bg-content text-background rounded-full p-2 cursor-default">
                  <ArrowDownIcon height={24} width={24} className="text-gold" />
                </button>
              </div>
            </div>
          </div>
          <div className="font-semibold">{totalGramSelectedAtom}g of gold</div>
        </div>
        <TransactionDetails defaultOpen={true} />

        <Button
          onClick={handleConfirm}
          disabled={false}
          className="w-full px-6 py-3 bg-secondary text-white font-medium rounded-md"
        >
          Confirm
        </Button>
      </div>
    </div>
  );
};

export default Confirm;
