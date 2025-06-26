import { ReactNode } from "react";
import clsx from "clsx";
import { BuyCrypto, ArrangeVertical, Refresh, HuobiToken } from "iconsax-react";
import { useAuth } from "@auth/index";
import Redeem from "@assets/icons/redeem.svg";
import Govern from "@assets/icons/govern.svg";
import Earn from "@assets/icons/earn.svg";
import BuyOnBity from "@assets/icons/bity_white.svg";

type Feature =
  | "buy-gldt"
  | "transfer"
  | "swap"
  | "earn"
  | "buy-on-bity"
  | "redeem"
  | "govern"
  | "mint-nft"
  | "burn-nft";

const AppFeatureBtn = ({
  action,
  handleOnClick = () => {},
}: {
  action: Feature;
  handleOnClick?: () => void;
}) => {
  const { isConnected } = useAuth();
  const actions: Record<Feature, { icon: ReactNode; text: string }> = {
    "buy-gldt": { icon: <BuyCrypto />, text: "Buy GLDT" },
    transfer: { icon: <ArrangeVertical />, text: "Transfer" },
    swap: { icon: <Refresh />, text: "Swap" },
    earn: { icon: <img src={Earn} alt="Earn" />, text: "Earn" },
    "buy-on-bity": {
      icon: <img src={BuyOnBity} alt="Buy on BITY" />,
      text: "Buy on BITY",
    },
    redeem: { icon: <img src={Redeem} alt="Redeem" />, text: "Redeem" },
    govern: { icon: <img src={Govern} alt="Govern" />, text: "Govern" },
    "mint-nft": { icon: <BuyCrypto />, text: "Mint" },
    "burn-nft": { icon: <HuobiToken />, text: "Burn" },
  };

  const renderBtn = () => {
    return (
      <button
        onClick={handleOnClick}
        className={clsx(
          "relative rounded-xl shrink-0 cursor-pointer disabled:cursor-default",
          "bg-primary text-white hover:bg-primary/80 disabled:bg-primary/60",
          "w-[72px] xl:w-[140px]"
        )}
        disabled={!isConnected}
      >
        <div
          className={clsx(
            "flex flex-col justify-center items-center gap-1",
            "px-1 py-3"
          )}
        >
          {actions[action].icon}
          <div className="text-xs xl:text-base">{actions[action].text}</div>
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

export default AppFeatureBtn;
