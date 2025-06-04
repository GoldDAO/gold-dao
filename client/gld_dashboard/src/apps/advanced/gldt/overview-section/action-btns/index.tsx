import { ReactNode } from "react";
import { useAtom } from "jotai";
import clsx from "clsx";
import { BuyCrypto, HuobiToken } from "iconsax-react";
import BuyOnBity from "@assets/icons/buy_on_bity.svg";
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
      <div
        onClick={handleOnClick}
        className={clsx(
          "flex justify-center px-1 py-3 rounded-md shrink-0",
          "bg-secondary text-white cursor-pointer",
          "xl:w-[140px] xl:rounded-lg"
        )}
      >
        <div
          className={clsx(
            "flex justify-center items-center gap-2",
            "xl:flex-col xl:gap-1"
          )}
        >
          {actions[action].icon}
          <div>{actions[action].text}</div>
        </div>
      </div>
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
