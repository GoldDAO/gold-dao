import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
import { useQueryClient } from "@tanstack/react-query";
import clsx from "clsx";

import { useAuth } from "@auth/index";

import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";

import { TokenSelectedAtom } from "../balance.atoms";
import { SendTokenStateAtom, TransferTokenStateAtom } from "./atoms";

import useTransfer from "@services/ledger/hooks/useTransfer";

const TransferTokenConfirm = ({ className }: { className?: string }) => {
  const queryClient = useQueryClient();
  const { principalId, authenticatedAgent } = useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const sendState = useAtomValue(SendTokenStateAtom);
  const setTransferState = useSetAtom(TransferTokenStateAtom);
  const { amount, receive_address, decimals, fee } = sendState;

  const transfer = useTransfer(token.canisterId, authenticatedAgent);

  if (
    decimals === null ||
    fee === null ||
    amount === null ||
    receive_address === null
  ) {
    return (
      <div className="flex justify-center items-center px-4 py-16 lg:py-32">
        Loading...
      </div>
    );
  }

  const handleConfirm = () => {
    transfer.mutate(
      { amount, to: receive_address },
      {
        onSuccess: () => {
          console.log("transfered");
          queryClient.invalidateQueries({
            queryKey: [`USER_FETCH_LEDGER_BALANCE_${token.name}`],
          });
        },
      }
    );
  };

  const handleRetry = () => {
    transfer.reset();
    transfer.mutate(
      {
        to: receive_address,
        amount,
      },
      {
        onSuccess: () => {
          console.log("transfered");
        },
      }
    );
  };

  const handleClose = () => {
    setTransferState(RESET);
  };

  return (
    <div className={className}>
      {transfer.isIdle && (
        <>
          <div className="border border-border rounded-md lg:rounded-lg p-4">
            <div className="mb-2">Sending Account</div>
            <div className="text-content/60 text-sm">{principalId}</div>

            <div className="my-4 lg:my-6 text-content/20 border-b border-dashed" />

            <div>Receiver Account</div>
            <div className="text-content/60 text-sm mt-2">
              {receive_address}
            </div>

            <div className="my-4 lg:my-6 text-content/20 border-b border-dashed" />

            <div className="mb-2">Total</div>

            <div className={clsx("flex flex-col gap-1 lg:gap-2")}>
              <div className="flex flex-col lg:flex-row lg:justify-between gap-2 text-sm">
                <div className="text-content/60">
                  Amount deducted (including fee)
                </div>
                <TokenValueToLocaleString
                  value={amount + fee}
                  decimals={decimals}
                />{" "}
                {token.name}
              </div>

              <div className="flex flex-col lg:flex-row lg:justify-between gap-2 text-content/60 text-sm">
                <div>Fee</div>
                <TokenValueToLocaleString
                  value={fee}
                  decimals={decimals}
                />{" "}
                {token.name}
              </div>

              <div className="flex flex-col lg:flex-row lg:justify-between gap-2 text-sm">
                <div className="text-content/60">
                  Amount received on new wallet
                </div>
                <TokenValueToLocaleString value={amount} decimals={decimals} />{" "}
                {token.name}
              </div>
            </div>
          </div>
          <div className="mt-8">
            <Button
              type="button"
              onClick={handleConfirm}
              className="w-full px-6 py-3 bg-secondary text-white lg:text-lg font-medium rounded-md"
            >
              Confirm Transfer
            </Button>
          </div>
        </>
      )}
      {!transfer.isIdle && (
        <>
          <div className="flex flex-col gap-4 mt-4 lg:mt-8">
            <div className="flex items-center gap-4">
              <MutationStatusIcons status={transfer.status} />
              <div>1. Transfer</div>
            </div>
          </div>

          <div className="flex justify-center items-center gap-2 mt-4 lg:mt-8">
            {transfer.isError && (
              <Button
                onClick={handleRetry}
                className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
              >
                Retry
              </Button>
            )}
            {!transfer.isPending && (
              <Button
                onClick={handleClose}
                className="px-6 py-2 bg-secondary text-white lg:text-lg font-medium rounded-md"
              >
                Close
              </Button>
            )}
          </div>
        </>
      )}
    </div>
  );
};

export default TransferTokenConfirm;
