import { useAtom } from "jotai";
import { Link } from "react-router-dom";
import Button from "@components/buttons/Button";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";

const InsufficientGLDTDisclaimer = ({
  totalGLDTSelected,
  totalNFTSelected,
  balance,
}: {
  totalGLDTSelected: number;
  totalNFTSelected: number;
  balance: number;
}) => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  return (
    <div className="border border-orange-500 bg-orange-500/5 p-4 flex flex-col justify-center items-center rounded-xl text-center">
      <div className="mb-6 text-orange-500">
        <div className="font-semibold mb-2">
          You don't have enough GLDT to process.
        </div>
        <div className="text-sm">
          To burn the{" "}
          <span className="font-semibold">{totalNFTSelected} GLD NFT</span>{" "}
          selected you need to have at least{" "}
          <span className="font-semibold">{totalGLDTSelected} GLDT</span>.
          <br />
          Your current balance is{" "}
          <span className="font-semibold">
            <NumberToLocaleString value={balance} /> GLDT.
          </span>
        </div>
      </div>
      <div>
        <Link
          to={"/buy"}
          className="px-6 py-2 rounded-full bg-secondary text-white"
          onClick={() => {
            dispatchSwapNFT({ type: "RESET" });
            dispatchSelectNFTState({ type: "RESET" });
          }}
        >
          <Button>Buy GLDT</Button>
        </Link>
      </div>
    </div>
  );
};

export default InsufficientGLDTDisclaimer;
