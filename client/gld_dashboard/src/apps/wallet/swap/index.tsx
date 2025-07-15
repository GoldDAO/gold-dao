import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useSwapTokensSearchParams from "@wallet/swap/hooks/useSwapTokensSearchParams";
import FormDialog from "@wallet/swap/components/form-dialog";
import ConfirmDialog from "@wallet/swap/components/confirm-dialog";
import DisclaimerConfirmHighSlippageDialog from "@wallet/swap/components/disclaimer-confirm-high-slippage-dialog";
import DetailsDialog from "@wallet/swap/components/details-dialog";

const SwapToken = () => {
  const { isConnected } = useAuth();
  const [, dispatchSwapState] = useAtom(SwapStateReducerAtom);
  const { isSwap, tokenFrom, tokenTo, deleteSwapTokensSearchParams } =
    useSwapTokensSearchParams();

  useEffect(() => {
    if (isConnected && isSwap) {
      dispatchSwapState({
        type: "OPEN_DIALOG_FORM",
        value: { token_from: tokenFrom, token_to: tokenTo },
      });
      deleteSwapTokensSearchParams();
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [isConnected, isSwap]);

  return (
    <>
      <FormDialog />
      <ConfirmDialog />
      <DisclaimerConfirmHighSlippageDialog />
      <DetailsDialog />
    </>
  );
};

export default SwapToken;
