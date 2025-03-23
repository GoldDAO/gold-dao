import { ReactNode } from "react";
import { useSearchParams } from "react-router-dom";
import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";

import { TokenSelectedAtom } from "./balance.atoms";
import {
  TransferTokenStateAtom,
  SendTokenStateAtom,
} from "./transfer.token/atoms";

import {
  TransferStateAtom as TransferNFTStateAtom,
  SendStateAtom as SendNFTStateAtom,
} from "./transfer.nft/atoms";

import TransferTokenDialog from "./transfer.token/Dialog.component";
import TransferNFTDIalog from "./transfer.nft/Dialog.component";

const ButtonAction = ({
  action,
  handleOnClick = () => {},
}: {
  action: string;
  handleOnClick?: () => void;
}) => {
  const ButtonContent = ({ children }: { children: ReactNode }) => {
    return (
      <div
        className={clsx(
          "flex justify-center items-center gap-2",
          "lg:flex-col lg:gap-1"
        )}
      >
        {children}
      </div>
    );
  };

  return (
    <div
      onClick={handleOnClick}
      className={clsx(
        "flex justify-center px-4 py-3 rounded-md shrink-0",
        "bg-secondary text-white cursor-pointer",
        "xl:px-12 lg:px-8 lg:rounded-lg"
      )}
    >
      {action === "transfer" && (
        <ButtonContent>
          <div>*</div>
          <div>Transfer</div>
        </ButtonContent>
      )}
      {action === "swap" && (
        <ButtonContent>
          <div>*</div>
          <div>Swap</div>
        </ButtonContent>
      )}
      {action === "stake" && (
        <ButtonContent>
          <div>*</div>
          <div>Stake</div>
        </ButtonContent>
      )}
      {action === "mint" && (
        <ButtonContent>
          <div>*</div>
          <div>Mint</div>
        </ButtonContent>
      )}
      {action === "buy-on-bity" && (
        <ButtonContent>
          <div>*</div>
          <div>Buy on BITY</div>
        </ButtonContent>
      )}
      {action === "redeem" && (
        <ButtonContent>
          <div>*</div>
          <div>Redeem</div>
        </ButtonContent>
      )}
    </div>
  );
};

const BalanceBtnAction = ({ className }: { className?: string }) => {
  const [searchParams] = useSearchParams();
  const token = useAtomValue(TokenSelectedAtom);
  const setTransferTokenState = useSetAtom(TransferTokenStateAtom);
  const setSendTokenState = useSetAtom(SendTokenStateAtom);
  const setTransferNFTState = useSetAtom(TransferNFTStateAtom);
  const setSendNFTState = useSetAtom(SendNFTStateAtom);

  const { id } = token;

  const handleOpenTransferTokenDialog = () => {
    setSendTokenState(RESET);
    setTransferTokenState((state) => ({
      ...state,
      is_open_transfer_dialog: true,
    }));
  };

  const handleOpenTransferNFTDialog = () => {
    setSendNFTState(RESET);
    setTransferNFTState((state) => ({
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
            handleOnClick={handleOpenTransferNFTDialog}
          />
          <ButtonAction action="mint" />
          <ButtonAction action="buy-on-bity" />
          <ButtonAction action="redeem" />
        </>
      );
    } else {
      if (id === "gldt") {
        return (
          <>
            <ButtonAction
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            <ButtonAction action="swap" />
            <ButtonAction action="stake" />
            <ButtonAction action="mint" />
          </>
        );
      } else if (id === "goldao") {
        return (
          <>
            <ButtonAction
              action="transfer"
              handleOnClick={handleOpenTransferTokenDialog}
            />
            <ButtonAction action="stake" />
          </>
        );
      } else {
        return (
          <ButtonAction
            action="transfer"
            handleOnClick={handleOpenTransferTokenDialog}
          />
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
            "lg:flex-row lg:gap-4"
          )}
        >
          {renderTokenAction()}
        </div>
      </div>
      <TransferTokenDialog />
      <TransferNFTDIalog />
    </>
  );
};

export default BalanceBtnAction;
