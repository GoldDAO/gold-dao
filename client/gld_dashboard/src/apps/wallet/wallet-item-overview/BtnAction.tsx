import { ReactNode } from "react";
import { useAtom } from "jotai";
import { useNavigate, useSearchParams } from "react-router-dom";
import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { BuyCrypto, ArrangeVertical, Refresh } from "iconsax-react";
import BuyOnBity from "@assets/icons/buy_on_bity.svg";
import Redeem from "@assets/icons/redeem.svg";
import Govern from "@assets/icons/govern.svg";
import Earn from "@assets/icons/earn.svg";
import { TokenSelectedAtom } from "../atoms";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "../transfer.token/atoms";
import { TransferNFTStateReducerAtom } from "../transfer.nft/atoms";
import TransferDialogToken from "../transfer.token/Dialog.component";
import TransferDialogNFT from "../transfer.nft";
import SendDialogNFTDetails from "../transfer.nft/SendDialogNFTDetails";

type Action =
  | "buy-gldt"
  | "transfer"
  | "swap"
  | "earn"
  | "mint"
  | "buy-on-bity"
  | "redeem"
  | "govern";

const ButtonAction = ({
  action,
  handleOnClick = () => {},
}: {
  action: Action;
  handleOnClick?: () => void;
}) => {
  const actions: Record<Action, { icon: ReactNode; text: string }> = {
    "buy-gldt": { icon: <BuyCrypto />, text: "Buy GLDT" },
    transfer: { icon: <ArrangeVertical />, text: "Transfer" },
    swap: { icon: <Refresh />, text: "Swap" },
    earn: { icon: <img src={Earn} alt="Earn" />, text: "Earn" },
    mint: { icon: <BuyCrypto />, text: "Mint" },
    "buy-on-bity": {
      icon: <img src={BuyOnBity} alt="Buy on BITY" />,
      text: "Buy on BITY",
    },
    redeem: { icon: <img src={Redeem} alt="Redeem" />, text: "Redeem" },
    govern: { icon: <img src={Govern} alt="Govern" />, text: "Govern" },
  };

  const renderBtn = () => {
    return (
      <div
        onClick={handleOnClick}
        className={clsx(
          "flex justify-center px-3 py-4 rounded-md shrink-0",
          "bg-secondary text-white cursor-pointer",
          "xl:px-12 xl:px-8 xl:rounded-lg"
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

const BalanceBtnAction = ({ className }: { className?: string }) => {
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
          {/* <ButtonAction action="mint" /> */}
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
              action="earn"
              handleOnClick={() => navigate("/earn")}
            />
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
    </>
  );
};

export default BalanceBtnAction;
