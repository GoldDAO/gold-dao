import { useAtomValue, useSetAtom, useAtom } from "jotai";
import { RESET } from "jotai/utils";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import E8sToLocaleString from "@shared/components/numbers/E8sToLocaleString";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import MutationStatusIcons from "@components/icons/MutationStatusIcons";
import { TokenSelectedAtom } from "@wallet/shared/atoms/WalletAtom";
import {
  SendTokenStateAtom,
  TransferTokenStateAtom,
} from "@wallet/shared/atoms/TransferTokenAtom";
import useTransfer from "@services/ledger/hooks/useTransfer";
import useFetchTokenData from "@shared/hooks/useFetchTokenData";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Confirm = ({ className }: { className?: string }) => {
  const { principalId, authenticatedAgent, unauthenticatedAgent, isConnected } =
    useAuth();
  const token = useAtomValue(TokenSelectedAtom);
  const [sendState, setSendState] = useAtom(SendTokenStateAtom);
  const setTransferState = useSetAtom(TransferTokenStateAtom);
  const { amount, receive_account, is_principal_standard } = sendState;

  const transfer = useTransfer(token.canisterId, authenticatedAgent, {
    ledger: token.name,
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
    setSendState((state) => ({
      ...state,
      is_send_confirm: false,
    }));
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

            <div>Receiving Account</div>
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
                    <E8sToLocaleString
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
                    <E8sToLocaleString
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

          <BtnPrimary onClick={handleTransfer} className="w-full mt-8">
            Confirm Transfer
          </BtnPrimary>
        </>
      )}
      {!transfer.isIdle && (
        <>
          {transfer.isPending && (
            <div className="flex flex-col gap-1 items-center text-4xl pb-12">
              <MutationStatusIcons status={transfer.status} className="mb-4" />
              <div>Transfer</div>
              <div className="text-gold">in progress...</div>
            </div>
          )}

          {transfer.isError && (
            <div className="flex flex-col items-center gap-12 pb-8">
              <div className="flex flex-col gap-1 items-center text-4xl">
                <MutationStatusIcons
                  status={transfer.status}
                  className="mb-4"
                />
                <div>Transfer has</div>
                <div className="text-gold">failed</div>
              </div>
              <div>
                <BtnPrimary onClick={handleRetry} variant="outlined">
                  Retry
                </BtnPrimary>
                <BtnPrimary onClick={handleClose} className="ml-4">
                  Back to wallet
                </BtnPrimary>
              </div>
            </div>
          )}

          {transfer.isSuccess && (
            <div className="flex flex-col items-center gap-12 pb-8">
              <div className="flex flex-col gap-1 items-center text-4xl">
                <MutationStatusIcons
                  status={transfer.status}
                  className="mb-4"
                />
                <div>Transfer has been</div>
                <div className="text-gold">completed</div>
              </div>
              <div className="flex flex-col gap-2 items-center">
                <BtnPrimary onClick={handleClose}>Back to wallet</BtnPrimary>
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
