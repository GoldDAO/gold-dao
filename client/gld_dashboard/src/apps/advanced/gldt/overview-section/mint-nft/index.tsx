import { useAtom } from "jotai";
import Dialog from "@components/dialogs/Dialog";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import { SelectNFTStateReducerAtom } from "@shared/atoms/NFTStateAtom";
import MintSubmit from "@advanced/gldt/overview-section/mint-nft/submit";
import MintConfirm from "@advanced/gldt/overview-section/mint-nft/confirm";
import MintDetails from "@advanced/gldt/overview-section/mint-nft/details";
import Switch from "@shared/components/ui/switch/SwitchWithLabel";

const MintNFT = () => {
  const [swapNFT, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);
  const [, dispatchSelectNFTState] = useAtom(SelectNFTStateReducerAtom);

  const handleCloseSwapNFTDialog = () => {
    dispatchSelectNFTState({ type: "RESET" });
    dispatchSwapNFT({ type: "RESET" });
  };

  return (
    <>
      <Dialog
        open={swapNFT.mode === "mint" && swapNFT.step === "submit"}
        handleOnClose={handleCloseSwapNFTDialog}
      >
        <div className="flex justify-center mb-8">
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
