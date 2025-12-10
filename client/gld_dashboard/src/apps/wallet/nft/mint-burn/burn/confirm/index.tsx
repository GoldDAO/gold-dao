import { useAtom, useAtomValue } from "jotai";
import {
  SelectNFTStateReducerAtom,
  TotalGLDTSelectedAtom,
  TotalNFTSelectedAtom,
  TotalGramSelectedAtom,
  TotalCollectionSelectedAtom,
} from "@shared/atoms/NFTStateAtom";
import SwapNFTReducerAtom from "../../atoms/SwapNFTAtom";
import TransactionDetails from "../transaction-details";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import Icon from "@shared/ui/icons";

const Confirm = () => {
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const totalGLDTSelected = useAtomValue(TotalGLDTSelectedAtom);
  const totalNFTSelected = useAtomValue(TotalNFTSelectedAtom);
  const totalGramSelectedAtom = useAtomValue(TotalGramSelectedAtom);
  const totalCollectionSelected = useAtomValue(TotalCollectionSelectedAtom);
  const totalApproveFees = totalCollectionSelected * 0.1;
  const totalToSend = totalGLDTSelected + totalNFTSelected + totalApproveFees;

  const handleConfirm = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchSwapNFT({ type: "CONFIRM" });
  };

  return (
    <div>
      <div className="mt-8 flex flex-col gap-8">
        <div className="text-center">
          You are sending{" "}
          <span className="text-copper font-semibold">
            <NumberToLocaleString value={totalToSend} /> GLDT
          </span>{" "}
          and will receive{" "}
          <span className="text-copper font-semibold">
            {totalGramSelectedAtom}g GLD NFTs
          </span>
          . <br />
          <span className="text-copper font-semibold">
            <NumberToLocaleString value={totalGLDTSelected} /> GLDT
          </span>{" "}
          will be burned and{" "}
          <span className="text-copper font-semibold">
            {totalNFTSelected} GLDT
          </span>{" "}
          fee are charged.{" "}
          <span className="text-copper font-semibold">
            {totalCollectionSelected} x 0.1 GLDT
          </span>{" "}
          approvals are required.
        </div>
        <div className="flex flex-col items-center gap-6 border border-border bg-surface-secondary p-6 rounded-xl">
          <div className="font-semibold">
            <NumberToLocaleString value={totalToSend} /> GLDT
          </div>
          <div className="w-full flex justify-center items-center py-4">
            <div className="relative w-full">
              <div className="border-t border-border w-full"></div>
              <div className="absolute inset-x-0 top-0 flex justify-center transform -translate-y-1/2">
                <button className="bg-content text-background rounded-full p-2 cursor-default">
                  <Icon.Arrow height={24} width={24} className="text-gold" />
                </button>
              </div>
            </div>
          </div>
          <div className="font-semibold">{totalGramSelectedAtom}g of gold</div>
        </div>
        <TransactionDetails defaultOpen={true} />
        <BtnPrimary onClick={handleConfirm} disabled={false} className="w-full">
          Confirm
        </BtnPrimary>
      </div>
    </div>
  );
};

export default Confirm;
