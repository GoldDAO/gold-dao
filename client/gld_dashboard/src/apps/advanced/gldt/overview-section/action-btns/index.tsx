import { ReactNode } from "react";
import { useAtom } from "jotai";
import clsx from "clsx";
import { BuyCrypto, HuobiToken } from "iconsax-react";
import { useAuth } from "@auth/index";
import BuyOnBity from "@assets/icons/bity_white.svg";
import SwapNFTReducerAtom from "@advanced/gldt/overview-section/shared/atoms/SwapNFTAtom";
import MintNFT from "@advanced/gldt/overview-section/mint-nft";
import BurnNFT from "@advanced/gldt/overview-section/burn-nft";

type Action = "buy-on-bity" | "mint-nft" | "burn-nft";

const Btn = ({
  action,
  handleOnClick = () => {},
}: {
  action: Action;
  handleOnClick?: () => void;
}) => {
  const { isConnected } = useAuth();

  const actions: Record<Action, { icon: ReactNode; text: string }> = {
    "mint-nft": { icon: <BuyCrypto />, text: "Mint" },
    "burn-nft": { icon: <HuobiToken />, text: "Burn" },
    "buy-on-bity": {
      icon: <img src={BuyOnBity} alt="Buy on BITY" />,
      text: "Buy on BITY",
    },
  };

  const renderBtn = () => {
    return (
      <button
        onClick={handleOnClick}
        className={clsx(
          "relative rounded-xl shrink-0 cursor-pointer disabled:cursor-default w-full",
          "bg-primary text-white hover:bg-primary/80 disabled:bg-primary/60",
          "xl:w-[140px] xl:rounded-xl"
        )}
        disabled={!isConnected}
      >
        <div
          className={clsx(
            "flex justify-center items-center gap-2",
            "xl:flex-col xl:gap-1",
            "px-1 py-3"
          )}
        >
          {actions[action].icon}
          <div>{actions[action].text}</div>
        </div>
        {!isConnected && (
          <div className="absolute rounded-[inherit] top-0 w-full h-full bg-white/30" />
        )}
      </button>
    );
  };

  if (action === "buy-on-bity") {
    return (
      <a
        href="https://gold.bity.com/"
        target="_blank"
        rel="noopener noreferrer"
        className="xl:flex shrink-0"
      >
        {renderBtn()}
      </a>
    );
  }

  return renderBtn();
};

const ActionBtns = () => {
  const [, dispatchSwapNFT] = useAtom(SwapNFTReducerAtom);

  return (
    <>
      <div
        className={clsx(
          "flex flex-col justify-center gap-2",
          "xl:flex-row xl:gap-4"
        )}
      >
        <Btn
          action="mint-nft"
          handleOnClick={() => {
            dispatchSwapNFT({ type: "INIT_MINT_MODE" });
          }}
        />
        <Btn
          action="burn-nft"
          handleOnClick={() => {
            dispatchSwapNFT({ type: "INIT_BURN_MODE" });
          }}
        />
        <Btn action="buy-on-bity" />
      </div>
      <MintNFT />
      <BurnNFT />
    </>
  );
};

export default ActionBtns;
