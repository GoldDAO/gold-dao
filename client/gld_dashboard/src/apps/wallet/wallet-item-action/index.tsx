import { ReactNode } from "react";
import { useAtom } from "jotai";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { BuyCrypto, ArrangeVertical, Refresh, HuobiToken } from "iconsax-react";
import { useAuth } from "@auth/index";
import Redeem from "@assets/icons/redeem.svg";
import Govern from "@assets/icons/govern.svg";
import Earn from "@assets/icons/earn.svg";
import BuyOnBity from "@assets/icons/bity_white.svg";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "@wallet/shared/atoms/TransferTokenAtom";
import { TransferNFTStateReducerAtom } from "@wallet/shared/atoms/TransferNFTAtom";
import TransferDialogToken from "@wallet/transfer-token";
import TransferDialogNFT from "@wallet/transfer-nft";
import SendDialogNFTDetails from "@wallet/transfer-nft/Details";
import MintNFT from "@advanced/gldt/overview-section/mint-nft";
import BurnNFT from "@advanced/gldt/overview-section/burn-nft";

type Action =
  | "buy-gldt"
  | "transfer"
  | "swap"
  | "earn"
  | "buy-on-bity"
  | "redeem"
  | "govern"
  | "mint-nft"
  | "burn-nft";

const ButtonAction = ({
  action,
  handleOnClick = () => {},
}: {
  action: Action;
  handleOnClick?: () => void;
}) => {
  const { isConnected } = useAuth();
  const actions: Record<Action, { icon: ReactNode; text: string }> = {
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
          "relative rounded-md shrink-0 cursor-pointer disabled:cursor-default w-full",
          "bg-secondary text-white",
          "xl:w-[140px] xl:rounded-lg"
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

const WalletItemAction = ({ className }: { className?: string }) => {
  const [searchParams] = useSearchParams();
  const navigate = useNavigate();
  const token = useAtomValue(TokenSelectedAtom);
  const setTransferTokenState = useSetAtom(TransferTokenStateAtom);
  const setSendTokenState = useSetAtom(SendTokenStateAtom);
  const [, dispatchTransferNFT] = useAtom(TransferNFTStateReducerAtom);

  const { id } = token;

  const handleOpenTransferTokenDialog = () => {
    setSendTokenState(RESET);
    setTransferTokenState((state) => ({
      ...state,
      is_open_transfer_dialog: true,
    }));
  };

  const renderTokenAction = () => {
    if (searchParams.get("token") === "nft") {
      return (
        <>
          <ButtonAction
            action="transfer"
            handleOnClick={() =>
              dispatchTransferNFT({ type: "OPEN_TRANSFER_DIALOG" })
            }
          />
          <ButtonAction
            action="mint-nft"
            handleOnClick={() => navigate("/advanced/gldt")}
          />
          <ButtonAction action="buy-on-bity" />
          {/* <ButtonAction action="redeem" /> */}
        </>
      );
    } else {
      if (id === "gldt") {
        return (
          <>
            <ButtonAction
              action="buy-gldt"
              handleOnClick={() => navigate("/buy")}
            />
            <ButtonAction
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            {/* <ButtonAction action="swap" /> */}
            <ButtonAction
              action="burn-nft"
              handleOnClick={() => navigate("/advanced/gldt")}
            />
            {/* <ButtonAction
              action="earn"
              handleOnClick={() => navigate("/earn")}
            /> */}
          </>
        );
      } else if (id === "goldao") {
        return (
          <>
            <ButtonAction
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            {/* <ButtonAction action="swap" /> */}
            <ButtonAction
              action="govern"
              handleOnClick={() => navigate("/govern")}
            />
          </>
        );
      } else {
        return (
          <>
            <ButtonAction
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            {/* <ButtonAction action="swap" /> */}
          </>
        );
      }
    }
  };

  return (
    <>
      <div className={className}>
        <div
          className={clsx(
            "flex flex-col justify-center gap-2",
            "xl:flex-row xl:gap-4"
          )}
        >
          {renderTokenAction()}
        </div>
      </div>
      <TransferDialogToken />
      <TransferDialogNFT />
      <SendDialogNFTDetails />
      <MintNFT />
      <BurnNFT />
    </>
  );
};

export default WalletItemAction;
