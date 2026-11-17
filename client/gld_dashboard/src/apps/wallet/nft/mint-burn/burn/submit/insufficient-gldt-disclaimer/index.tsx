import { useAtom } from "jotai";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import BtnPrimary from "@shared/ui/button/HorizontalButton";
import SwapNFTReducerAtom from "../../../atoms/SwapNFTAtom";

const InsufficientGLDTDisclaimer = ({
  totalGLDTSelected,
  totalNFTSelected,
  totalCollectionSelected,
  balance,
}: {
  totalGLDTSelected: number;
  totalNFTSelected: number;
  totalCollectionSelected: number;
  balance: number;
}) => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);
  const totalNeeded =
    totalGLDTSelected + totalNFTSelected + totalCollectionSelected * 0.1;

  return (
    <div className="border border-warning bg-warning/5 p-4 flex flex-col justify-center items-center rounded-xl text-center">
      <div className="mb-6 text-warning">
        <div className="font-semibold mb-2">
          You don't have enough GLDT to process.
        </div>
        <div className="text-sm">
          To burn the{" "}
          <span className="font-semibold">{totalNFTSelected} GLD NFT</span>{" "}
          selected you need to have at least{" "}
          <span className="font-semibold">
            <NumberToLocaleString value={totalNeeded} /> GLDT
          </span>
          .
          <br />
          Includes{" "}
          <span className="font-semibold">
            {totalCollectionSelected} x 0.1 GLDT
          </span>{" "}
          approvals.
          <br />
          Your current balance is{" "}
          <span className="font-semibold">
            <NumberToLocaleString value={balance} /> GLDT.
          </span>
        </div>
      </div>
      <div>
        <a
          href="https://gldt.org/"
          target="_blank"
          rel="noopener noreferrer"
          onClick={() => {
            dispatchSwapNFT({ type: "RESET" });
            dispatchSelectNFTState({ type: "RESET" });
          }}
        >
          <BtnPrimary>Get GLDT on gldt.org</BtnPrimary>
        </a>
      </div>
    </div>
  );
};

export default InsufficientGLDTDisclaimer;
