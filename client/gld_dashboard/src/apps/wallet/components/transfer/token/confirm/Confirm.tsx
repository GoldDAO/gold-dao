import { useAtomValue, useSetAtom } from "jotai";
import { RESET } from "jotai/utils";
// import { Link } from "react-router-dom";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import { Button } from "@components/index";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import NumberToLocaleString from "@components/numbers/NumberToLocaleString";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { TokenSelectedAtom } from "@wallet/atoms/WalletAtom";
import {
  SendTokenStateAtom,
  TransferTokenStateAtom,
} from "@wallet/atoms/TransferTokenAtom";
import useTransfer from "@services/ledger/hooks/useTransfer";
import useFetchTokenData from "@hooks/useFetchTokenData";

const Confirm = ({ className }: { className?: string }) => {
  const { principalId, authenticatedAgent, unauthenticatedAgent, isConnected } =
    useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const sendState = useAtomValue(SendTokenStateAtom);
  const setTransferState = useSetAtom(TransferTokenStateAtom);
  const { amount, receive_account, is_principal_standard } = sendState;

  const transfer = useTransfer(token.canisterId, authenticatedAgent, {
    ledger: token.id,
    is_principal_standard,
  });

  const tokenData = useFetchTokenData(unauthenticatedAgent, {
    token: token.id,
    token_canister_id: token.canisterId,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  if (amount === null || receive_account === "" || !tokenData.isSuccess) {
    return (
      <div className="flex justify-center items-center px-4 py-16 xl:py-32">
        Loading...
      </div>
    );
  }

  const handleTransfer = () => {
    transfer.mutate({
      amount,
      account: receive_account,
      fee: tokenData.data.fee_e8s,
    });
  };

  const handleRetry = () => {
    transfer.reset();
    handleTransfer();
  };

  const handleClose = () => {
    setTransferState(RESET);
  };

  return (
    <div className={className}>
      {transfer.isIdle && (
        <>
          <div className="text-4xl text-center mb-8">You are sending</div>
          <div className="border border-border rounded-md xl:rounded-lg p-4">
            <div className="mb-2">Sending Account</div>
            <div className="text-content/60 text-sm">{principalId}</div>

            <div className="my-4 xl:my-6 text-content/20 border-b border-dashed" />

            <div>Receiver Account</div>
            <div className="text-content/60 text-sm mt-2">
              {receive_account}
            </div>

            <div className="my-4 xl:my-6 text-content/20 border-b border-dashed" />

            <div className="mb-2">Total</div>

            <div className={clsx("flex flex-col gap-1 xl:gap-2")}>
              <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-sm">
                <div className="text-content/60">
                  Amount deducted (including fee)
                </div>
                <div className="flex flex-col items-end">
                  <div>
                    <TokenValueToLocaleString
                      value={amount + tokenData.data.fee_e8s}
                      decimals={tokenData.data.decimals}
                    />{" "}
                    {token.name}
                  </div>
                  <div className="text-content/60">
                    ≈ $
                    <NumberToLocaleString
                      value={
                        (Number(amount + tokenData.data.fee_e8s) /
                          10 ** tokenData.data.decimals) *
                        tokenData.data.price_usd
                      }
                    />
                  </div>
                </div>
              </div>

              <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-content/60 text-sm">
                <div>Fee</div>
                <div className="flex flex-col items-end">
                  <div>
                    <NumberToLocaleString
                      value={tokenData.data.fee}
                      decimals={4}
                    />{" "}
                    {token.name}
                  </div>
                  <div>
                    ≈ $
                    <NumberToLocaleString value={tokenData.data.fee_usd} />
                  </div>
                </div>
              </div>

              <div className="flex flex-col xl:flex-row xl:justify-between gap-2 text-sm">
                <div className="text-content/60">
                  Amount received on new wallet
                </div>
                <div className="flex flex-col items-end">
                  <div>
                    <TokenValueToLocaleString
                      value={amount}
                      decimals={tokenData.data.decimals}
                    />{" "}
                    {token.name}
                  </div>
                  <div className="text-content/60">
                    ≈ $
                    <NumberToLocaleString
                      value={
                        (Number(amount) / 10 ** tokenData.data.decimals) *
                        tokenData.data.price_usd
                      }
                    />
                  </div>
                </div>
              </div>
            </div>
          </div>
          <div className="mt-8">
            <Button
              type="button"
              onClick={handleTransfer}
              className="w-full px-6 py-3 bg-secondary text-white xl:text-lg font-medium rounded-md"
            >
              Confirm Transfer
            </Button>
          </div>
        </>
      )}
      {!transfer.isIdle && (
        <>
          {transfer.isPending && (
            <div className="flex flex-col gap-1 items-center text-4xl pb-12">
              <MutationStatusIcons status={transfer.status} className="mb-4" />
              <div>Transfer</div>
              <div className="text-primary">in progress...</div>
            </div>
          )}

          {transfer.isError && (
            <div className="flex flex-col items-center gap-12">
              <div className="flex flex-col gap-1 items-center text-4xl">
                <MutationStatusIcons
                  status={transfer.status}
                  className="mb-4"
                />
                <div>Transfer has</div>
                <div className="text-primary">failed</div>
              </div>
              <div>
                <Button
                  onClick={handleRetry}
                  className="px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
                >
                  Retry
                </Button>
                <Button
                  onClick={handleClose}
                  className="ml-4 px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
                >
                  Back to wallet
                </Button>
              </div>
            </div>
          )}

          {transfer.isSuccess && (
            <div className="flex flex-col items-center gap-12">
              <div className="flex flex-col gap-1 items-center text-4xl">
                <MutationStatusIcons
                  status={transfer.status}
                  className="mb-4"
                />
                <div>Transfer has been</div>
                <div className="text-primary">completed</div>
              </div>
              <div className="flex flex-col gap-2 items-center">
                <Button
                  onClick={handleClose}
                  className="px-6 py-2 bg-secondary text-white xl:text-lg font-medium rounded-md"
                >
                  Back to wallet
                </Button>
                {/* <Link to="/wallet" className="text-content/60">
                  View in explorer
                </Link> */}
              </div>
            </div>
          )}
        </>
      )}
    </div>
  );
};

export default Confirm;
