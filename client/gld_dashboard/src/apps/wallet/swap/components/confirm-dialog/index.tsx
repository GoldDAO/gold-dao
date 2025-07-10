// import { useEffect } from "react";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
// import { Token, TOKENS } from "@shared/utils/tokens";
import { SwapStateReducerAtom } from "@wallet/swap/atoms";
import useFetchLedgerBalance from "@shared/hooks/useFetchLedgerBalance";
// import BalanceAvailable from "../balance-available";
// import NumberToLocaleString from "@shared/components/numbers/NumberToLocaleString";
import Dialog from "@shared/ui/dialog/DialogV2";
import Icon from "@shared/ui/icons";

const ConfirmDialog = ({
  open,
  onClose,
}: {
  open: boolean;
  onClose: () => void;
}) => {
  const { unauthenticatedAgent, principalId } = useAuth();
  const [swapState, dispatchSwapState] = useAtom(SwapStateReducerAtom);

  const balance = useFetchLedgerBalance(
    swapState.token_from.token.canister_id,
    unauthenticatedAgent,
    {
      ledger: swapState.token_from.token.name,
      owner: principalId,
      enabled: !!unauthenticatedAgent,
    }
  );

  return (
    <Dialog open={open} onClose={onClose}>
      <div className="flex items-center justify-between mb-4">
        <Icon.Chevron
          width={16}
          className="rotate-90"
          onClick={() => dispatchSwapState({ type: "BACK_DIALOG_CONFIRM" })}
        />
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      {balance.isSuccess ? <div>Balance: {balance.data.balance}</div> : null}
      <div>Swap confirm</div>
    </Dialog>
  );
};

export default ConfirmDialog;
