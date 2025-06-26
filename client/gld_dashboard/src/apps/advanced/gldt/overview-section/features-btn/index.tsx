import { useAtom } from "jotai";
import clsx from "clsx";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import MintNFT from "@advanced/gldt/overview-section/mint-nft";
import BurnNFT from "@advanced/gldt/overview-section/burn-nft";
import AppFeatureBtn from "@shared/components/app-feature-btn";

const ActionBtns = () => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);

  return (
    <>
      <div className={clsx("flex justify-center gap-2")}>
        <AppFeatureBtn
          action="mint-nft"
          handleOnClick={() => {
            dispatchSwapNFT({ type: "INIT_MINT_MODE" });
          }}
        />
        <AppFeatureBtn
          action="burn-nft"
          handleOnClick={() => {
            dispatchSwapNFT({ type: "INIT_BURN_MODE" });
          }}
        />
        <AppFeatureBtn action="buy-on-bity" />
      </div>
      <MintNFT />
      <BurnNFT />
    </>
  );
};

export default ActionBtns;
