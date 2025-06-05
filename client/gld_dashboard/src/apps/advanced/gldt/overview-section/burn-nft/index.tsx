import { useAtom } from "jotai";
import Dialog from "@components/dialogs/Dialog";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import BurnSubmit from "@advanced/gldt/overview-section/burn-nft/submit";
import BurnConfirm from "@advanced/gldt/overview-section/burn-nft/confirm";
import BurnDetails from "@advanced/gldt/overview-section/burn-nft/details";
import Switch from "@shared/components/ui/switch/SwitchWithLabel";

const BurnNFT = () => {
  const [swapNFT, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  const handleCloseSwapNFTDialog = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchSwapNFT({ type: "RESET" });
  };

  return (
    <>
      <Dialog
        open={swapNFT.mode === "burn" && swapNFT.step === "submit"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="flex justify-center items-center mb-8">
          <Switch
            value={swapNFT.mode}
            labelLeft="Mint"
            labelRight="Burn"
            handleClickLeft={() => {
              dispatchSelectNFTState({ type: "RESET" });
              dispatchSwapNFT({ type: "INIT_MINT_MODE" });
            }}
            handleClickRight={() => {
              dispatchSelectNFTState({ type: "RESET" });
              dispatchSwapNFT({ type: "INIT_BURN_MODE" });
            }}
          />
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
