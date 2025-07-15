import { useAtom } from "jotai";
import { KONGSWAP_CANISTER_ID_IC } from "@constants";
import { useAuth } from "@auth/index";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import Dialog from "@shared/ui/dialog/DialogV2";
import Icon from "@shared/ui/icons";
import useFetchSwapAmount from "@shared/hooks/useFetchSwapAmount";
import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import BtnPrimary from "@shared/ui/button/BtnPrimary";

const DisclaimerConfirmHighSlippageDialog = () => {
  const { unauthenticatedAgent } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const swapAmount = useFetchSwapAmount(
    KONGSWAP_CANISTER_ID_IC,
    unauthenticatedAgent,
    {
      from: swapState.token_from.token.name,
      from_canister_id: swapState.token_from.token.canister_id,
      to: swapState.token_to.token.name,
      amount: Number(swapState.send_amount_input),
      enabled: !!unauthenticatedAgent,
    }
  );

  const onClose = () => {
    dispatchSwapState({
      type: "CANCEL",
    });
  };

  const onConfirm = (slippage_with_tx_fee: number) => {
    dispatchSwapState({
      type: "CONFIRM_HIGH_SLIPPAGE",
      value: { slippage_with_tx_fee },
    });
  };

  return (
    <Dialog
      open={swapState.is_open_disclaimer_confirm_high_slippage_dialog}
      onClose={onClose}
    >
      {swapAmount.isSuccess ? (
        <>
          <div className="flex justify-center">
            <div className="flex items-center font-semibold text-lg mt-2 mb-4 gap-2">
              <Icon.Warning width={32} className="text-warning" />
              <div>High slippage</div>
            </div>
          </div>
          <div className="text-center text-content/60 mb-8">
            Slippage is quite high for this purchase.
            <br />
            <div className="inline-block max-w-md mx-auto">
              The current slippage is{" "}
              <span className="text-warning font-semibold">
                <NumberToLocaleString
                  value={swapAmount.data.slippage_with_tx_fee}
                />
                %
              </span>{" "}
              , which exceeds the maximum recommended slippage of{" "}
              {swapState.max_slippage}%.
            </div>
            <br />
            <div className="mt-2">
              Please confirm or consider purchasing another amount.
            </div>
          </div>
          <div className="flex justify-center gap-2">
            <BtnPrimary variant="outlined" onClick={onClose}>
              Cancel
            </BtnPrimary>
            <BtnPrimary
              onClick={() => onConfirm(swapAmount.data.slippage_with_tx_fee)}
            >
              Confirm
            </BtnPrimary>
          </div>
        </>
      ) : (
        <div>Loading...</div>
      )}
    </Dialog>
  );
};

export default DisclaimerConfirmHighSlippageDialog;
