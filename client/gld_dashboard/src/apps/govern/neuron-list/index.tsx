import { useEffect, useMemo, useState } from "react";
import { ColumnDef } from "@tanstack/react-table";
import { useAtom } from "jotai";
import clsx from "clsx";
import { useAuth } from "@auth/index";
import useFetchUserNeuronsList from "@services/sns_governance/hooks/useFetchUserNeuronsList";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import { NeuronUser } from "@services/sns_governance/utils/interfaces";
import { Button, Logo, Table } from "@components/index";
import { BadgeNeuronState } from "@components/badges/BadgeNeuronState";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import {
  GOLDAO_LEDGER_CANISTER_ID,
  SNS_GOVERNANCE_CANISTER_ID,
} from "@constants";
import Address from "@components/strings/Address";
import useGetOneNeuronRewards from "../utils/useGetOneNeuronRewards";
import useRewardsFee from "@shared/hooks/useRewardsFee";
import { ClaimRewardStateReducerAtom } from "../claim-reward/claim-one/atoms";
import Dialog from "@components/dialogs/Dialog";
import ClaimRewardsConfirm from "../claim-reward/claim-one/Confirm";
import ClaimRewardsDetails from "../claim-reward/claim-one/Details";

const ClaimRewardsBtn = ({ neuronId }: { neuronId: string }) => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const [, dispatchClaimReward] = useAtom(ClaimRewardStateReducerAtom);
  const [enableClaim, setEnableClaim] = useState(false);
  const [isSuccess, setIsSuccess] = useState(false);

  const rewards = useGetOneNeuronRewards({
    agent: unauthenticatedAgent,
    owner: principalId,
    enabled: isConnected && !!unauthenticatedAgent,
    neuronId,
  });

  const rewardsFee = useRewardsFee(unauthenticatedAgent, {
    enabled: isConnected && !!unauthenticatedAgent,
  });

  useEffect(() => {
    if (rewards.isSuccess && rewardsFee.isSuccess) {
      const enabled = rewards.data.some((reward) => {
        const found = rewardsFee.data.find(
          (rewardFee) => rewardFee.id === reward.id
        );
        return found ? reward.amount >= found.fee : false;
      });
      setEnableClaim(enabled);
      setIsSuccess(true);
    }
  }, [rewards.data, rewards.isSuccess, rewardsFee.data, rewardsFee.isSuccess]);

  return (
    <Button
      className={clsx(
        "px-2 py-1 rounded-md shrink-0",
        "border border-border text-black dark:text-white text-sm"
      )}
      disabled={!enableClaim}
      onClick={() =>
        dispatchClaimReward({
          type: "OPEN_DIALOG_CONFIRM",
          value: { neuron_id: neuronId },
        })
      }
    >
      {isSuccess ? (
        <div className="flex items-center gap-2">
          <div className="flex items-center gap-1">
            <Logo name="goldao" className="h-4 w-4" />
            <Logo name="icp" className="h-4 w-4" />
            <Logo name="ogy" className="h-4 w-4" />
          </div>
          Claim rewards
        </div>
      ) : (
        "Loading..."
      )}
    </Button>
  );
};

const List = () => {
  const { unauthenticatedAgent, isConnected, principalId } = useAuth();
  const [claimRewardState, dispatchClaimReward] = useAtom(
    ClaimRewardStateReducerAtom
  );

  const { status, data } = useFetchUserNeuronsList(
    SNS_GOVERNANCE_CANISTER_ID,
    unauthenticatedAgent,
    {
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected && !!principalId,
    }
  );

  const decimals = useFetchDecimals(
    GOLDAO_LEDGER_CANISTER_ID,
    unauthenticatedAgent,
    {
      ledger: "goldao",
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const columns = useMemo<ColumnDef<NeuronUser>[]>(
    () => [
      {
        accessorKey: "id",
        id: "id",
        cell: ({ getValue }) => <Address>{getValue() as string}</Address>,
        header: "ID",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "staked_amount",
        id: "staked_amount",
        cell: ({ getValue }) => (
          <div className="flex items-center gap-2">
            {!decimals.isSuccess ? (
              <div>Loading...</div>
            ) : (
              <>
                <Logo name="goldao" className="h-4" />
                <TokenValueToLocaleString
                  value={getValue() as bigint}
                  tokenDecimals={decimals.data}
                />
              </>
            )}
          </div>
        ),
        header: "GOLDAO Staked",
        meta: {
          className: "text-left",
        },
      },
      {
        accessorKey: "state",
        id: "state",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <BadgeNeuronState
              state={
                getValue() as "not dissolving" | "dissolving" | "dissolved"
              }
            />
          </div>
        ),
        header: "State",
      },
      {
        accessorKey: "dissolve_delay",
        id: "dissolve_delay",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <div>{getValue() as string}</div>
          </div>
        ),
        header: "Dissolve Delay",
      },
      {
        accessorKey: "age",
        id: "age",
        cell: ({ getValue }) => (
          <div className="flex items-center justify-center">
            <div>{getValue() as string}</div>
          </div>
        ),
        header: "Age",
      },
      {
        cell: ({ row }) => (
          <div className="flex items-center justify-center">
            <ClaimRewardsBtn neuronId={row.original.id} />
          </div>
        ),
        header: "Actions",
      },
    ],
    [decimals.isSuccess, decimals.data]
  );

  const renderDisconnectedPlaceholder = () => {
    return (
      <div className="flex flex-col gap-4 relative">
        {[...Array(2)].map((_, index) => (
          <div key={index}>
            <div
              className={clsx(
                "@container",
                "shrink-0",
                "rounded-md xl:rounded-xl border border-surface-secondary p-4 cursor-pointer"
              )}
            >
              <div className="flex justify-between items-center p-2">
                <div className="flex items-center gap-2">
                  <div className="h-5 w-5 bg-surface-secondary rounded-full" />
                  <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
                </div>
                <div className="h-5 w-[20cqw] bg-surface-secondary rounded-sm" />
              </div>
            </div>
          </div>
        ))}
        <div className="absolute bottom-0 left-0 right-0 h-24 bg-gradient-to-t from-background to-transparent" />
      </div>
    );
  };

  if (!isConnected) {
    return renderDisconnectedPlaceholder();
  }

  if (status === "success" && data?.length === 0) {
    return (
      <div className="p-4 flex justify-center border border-border rounded-lg">
        <div>No neurons found</div>
      </div>
    );
  }

  return (
    <>
      <div className="bg-surface-primary rounded-xl p-2 border border-border">
        {status === "success" ? (
          <Table
            columns={columns}
            data={data}
            pagination={{
              pageIndex: 0,
              pageSize: 100,
            }}
            serverSide={false}
          />
        ) : (
          <div className="flex justify-center items-center p-4 xl:p-8">
            {status === "pending" && <div>Loading...</div>}
            {status === "error" && <div>Error</div>}
          </div>
        )}
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
    </>
  );
};

export default List;
