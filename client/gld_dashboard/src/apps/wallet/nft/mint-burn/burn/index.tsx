import { useAtom } from "jotai";
import Dialog from "@shared/ui/dialog/Dialog";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import SwitchMintBurn from "@shared/components/switch/SwitchMintBurn";
import SwapNFTReducerAtom from "../atoms/SwapNFTAtom";
import BurnSubmit from "./submit";
import BurnConfirm from "./confirm";
import BurnDetails from "./details";

const BurnNFT = () => {
  const [swapNFT, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  const handleCloseSwapNFTDialog = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchSwapNFT({ type: "RESET" });
  };

  const handleChangeTab = (value: "mint" | "burn") => {
    dispatchSelectNFTState({ type: "RESET" });
    if (value === "mint") {
      dispatchSwapNFT({ type: "INIT_MINT_MODE" });
    } else {
      dispatchSwapNFT({ type: "INIT_BURN_MODE" });
    }
  };

  return (
    <>
      <Dialog
        open={swapNFT.mode === "burn" && swapNFT.step === "submit"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="flex justify-center items-center mb-8">
          <SwitchMintBurn value={swapNFT.mode} handleChange={handleChangeTab} />
        </div>
        <BurnSubmit />
      </Dialog>

      <Dialog
        open={swapNFT.mode === "burn" && swapNFT.step === "confirm"}
        handleOnClose={handleCloseSwapNFTDialog}
        handlePreviousStep={() =>
          dispatchSwapNFT({ type: "SET_STEP", value: "submit" })
        }
      >
        <div className="mt-4">
          <BurnConfirm />
        </div>
      </Dialog>

      <Dialog
        open={swapNFT.mode === "burn" && swapNFT.step === "details"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="mt-4">
          <BurnDetails />
        </div>
      </Dialog>
    </>
  );
};

export default BurnNFT;
