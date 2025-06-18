import { useState } from "react";
import clsx from "clsx";
import { useAtom } from "jotai";
import { useAuth } from "@auth/index";
import Dialog from "@components/dialogs/Dialog";
import InnerAppLayout from "@shared/components/app-layout/inner-app";
import NeuronsOverview from "./neuron-overview";
import NeuronsList from "./neuron-list";
import { ClaimRewardStateReducerAtom } from "./claim-reward/claim-all/atoms";
import ClaimRewardDisclaimer from "./claim-reward/claim-disclaimer";
import ClaimRewardsConfirm from "./claim-reward/claim-all/Confirm";
import ClaimRewardsDetails from "./claim-reward/claim-all/Details";
import AddNeuronDialog from "./add-neuron";
import BtnConnectWallet from "@shared/components/connect-wallet-btn";
import BtnPrimary from "@shared/components/ui/button/BtnPrimary";

const Govern = () => {
  const { isConnected } = useAuth();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );
  const [openAddNeuronDialog, setOpenAddNeuronDialog] = useState(false);

  return (
    <InnerAppLayout>
      <InnerAppLayout.LeftPanel>
        <div className="text-4xl xl:text-6xl flex flex-col justify-center items-center xl:items-start">
          <div className="font-semibold text-gold">Govern</div>
          <div className="font-light">the DAO</div>
        </div>
        <div className="text-content/60 text-center xl:text-left my-3">
          Stake GOLDAO tokens to participate in the Gold DAO governance and earn
          rewards yielding up to 30% APY.
        </div>
        {!isConnected && <BtnConnectWallet className="mt-auto w-full" />}
      </InnerAppLayout.LeftPanel>
      <InnerAppLayout.RightPanel>
        <NeuronsOverview />
        <div className="relative w-full px-4 xl:pb-16 pb-32">
          <div
            className={clsx(
              "my-4",
              "absolute -top-26 xl:-top-16 left-1/2 xl:my-0 -translate-x-1/2 w-full xl:w-xl px-4"
            )}
          >
            <ClaimRewardDisclaimer />
          </div>
        </div>

        <div className="p-4 xl:p-8">
          <div className="flex items-center justify-between mb-4 xl:mb-8">
            <div>My GOLDAO neurons</div>
            <BtnPrimary
              shape="round"
              onClick={() => setOpenAddNeuronDialog(true)}
              disabled={!isConnected}
            >
              Add neuron
            </BtnPrimary>
          </div>
          <NeuronsList />
        </div>

        {/* CLAIM REWARDS DIALOGS */}
        <Dialog
          open={claimRewardState.is_open_claim_dialog_confirm}
          handleOnClose={() => dispatchClaimReward({ type: "CANCEL" })}
          title="Claim rewards"
        >
          <ClaimRewardsConfirm />
        </Dialog>

        <Dialog
          open={claimRewardState.is_open_claim_dialog_details}
          handleOnClose={() => dispatchClaimReward({ type: "RESET" })}
          title="Claim details"
        >
          <ClaimRewardsDetails />
        </Dialog>

        {/* ADD NEURON DIALOG */}
        <AddNeuronDialog
          open={openAddNeuronDialog}
          handleClose={() => setOpenAddNeuronDialog(false)}
        />
      </InnerAppLayout.RightPanel>
    </InnerAppLayout>
  );
};

export default Govern;
