import { useEffect } from "react";
import { useAtom } from "jotai";
import Dialog from "@shared/ui/dialog/DialogV2";
import { ClaimRewardsStateReducerAtom } from "./atoms";
import FormDialogContent from "./components/form-dialog-content";
import DetailsDialogContent from "./components/details-dialog-content";

const ClaimRewards = () => {
  const [claimRewardsState, dispatchClaimRewardsState] = useAtom(
    ClaimRewardsStateReducerAtom
  );

  useEffect(() => {
    if (!claimRewardsState.is_open_dialog) {
      const timeoutId = setTimeout(() => {
        dispatchClaimRewardsState({
          type: "RESET",
        });
      }, 300);

      return () => clearTimeout(timeoutId);
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, [claimRewardsState.is_open_dialog]);

  const onClose = () => {
    dispatchClaimRewardsState({
      type: "SET_IS_OPEN_DIALOG",
      value: false,
    });
  };

  return (
    <Dialog open={claimRewardsState.is_open_dialog} onClose={onClose}>
      <div className="flex items-center justify-end mb-4">
        <Dialog.CloseBtn onClick={onClose} />
      </div>
      {claimRewardsState.is_step_form && <FormDialogContent />}
      {claimRewardsState.is_step_details && <DetailsDialogContent />}
    </Dialog>
  );
};

export default ClaimRewards;
