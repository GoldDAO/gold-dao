import { useAtom } from "jotai";
import Dialog from "@shared/ui/dialog/Dialog";
import SwapNFTReducerAtom from "../atoms/SwapNFTAtom";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import MintSubmit from "./submit";
import MintConfirm from "./confirm";
import MintDetails from "./details";
import SwitchMintBurn from "@shared/components/switch/SwitchMintBurn";

const MintNFT = () => {
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
        open={swapNFT.mode === "mint" && swapNFT.step === "submit"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="flex justify-center mb-8">
          <SwitchMintBurn value={swapNFT.mode} handleChange={handleChangeTab} />
        </div>
        <MintSubmit />
      </Dialog>

      <Dialog
        open={swapNFT.mode === "mint" && swapNFT.step === "confirm"}
        handleOnClose={handleCloseSwapNFTDialog}
        handlePreviousStep={() =>
          dispatchSwapNFT({ type: "SET_STEP", value: "submit" })
        }
      >
        <div className="mt-4">
          <MintConfirm />
        </div>
      </Dialog>

      <Dialog
        open={swapNFT.mode === "mint" && swapNFT.step === "details"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="mt-4">
          <MintDetails />
        </div>
      </Dialog>
    </>
  );
};

export default MintNFT;
