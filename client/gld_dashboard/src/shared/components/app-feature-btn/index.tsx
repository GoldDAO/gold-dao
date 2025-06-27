import { ReactNode } from "react";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import Icon from "@shared/ui/icons";

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
    "buy-gldt": {
      icon: <Icon.BuyCrypto width={24} aria-label="Buy GLDT" />,
      text: "Buy GLDT",
    },
    transfer: {
      icon: <Icon.Transfer width={24} aria-label="Transfer" />,
      text: "Transfer",
    },
    swap: { icon: <Icon.Swap width={24} aria-label="Swap" />, text: "Swap" },
    earn: { icon: <Icon.Earn aria-label="Earn" />, text: "Earn" },
    "buy-on-bity": {
      icon: <Icon.BuyOnBity aria-label="Buy on BITY" />,
      text: "Buy on BITY",
    },
    redeem: { icon: <Icon.Redeem aria-label="Redeem" />, text: "Redeem" },
    govern: { icon: <Icon.Govern aria-label="Govern" />, text: "Govern" },
    "mint-nft": {
      icon: <Icon.Mint width={24} aria-label="Mint" />,
      text: "Mint",
    },
    "burn-nft": {
      icon: <Icon.Burn width={24} aria-label="Burn" />,
      text: "Burn",
    },
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
