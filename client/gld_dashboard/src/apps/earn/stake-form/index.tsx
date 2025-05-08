import { useAuth } from "@auth/index";
import { useAtom } from "jotai";
import { GLDTToken } from "../utils";
import { Button, Logo } from "@components/index";
import Dialog from "@components/dialogs/Dialog";
import TokenValueToLocaleString from "@components/numbers/TokenValueToLocaleString";
import useFetchUserBalance from "@services/ledger/hooks/useFetchUserBalance";
import useFetchDecimals from "@services/ledger/hooks/useFetchDecimals";
import useFetchTransferFee from "@services/ledger/hooks/useFetchTransferFee";
import { MIN_STAKE_AMOUNT } from "./utils";
import Form from "./Form";
import Confirm from "./Confirm";
import Details from "./Details";

import { StakeStateReducerAtom } from "./atoms";

const StakeForm = () => {
  const { principalId, unauthenticatedAgent, isConnected } = useAuth();
  const [stakeState, dispatchStake] = useAtom(StakeStateReducerAtom);

  const balance = useFetchUserBalance(
    GLDTToken.canisterId,
    unauthenticatedAgent,
    {
      ledger: GLDTToken.id,
      owner: principalId,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  const fee = useFetchTransferFee(GLDTToken.canisterId, unauthenticatedAgent, {
    ledger: GLDTToken.id,
    enabled: !!unauthenticatedAgent && isConnected,
  });

  const decimals = useFetchDecimals(
    GLDTToken.canisterId,
    unauthenticatedAgent,
    {
      ledger: GLDTToken.id,
      enabled: !!unauthenticatedAgent && isConnected,
    }
  );

  if (
    !isConnected ||
    !decimals.isSuccess ||
    !balance.isSuccess ||
    !fee.isSuccess
  ) {
    return (
      <div className="relative">
        <div
          className={`p-4 bg-surface-primary border border-border rounded-xl`}
        >
          <div>Stake GLDT</div>
          <div className="p-4 text-center mt-4 xl:mt-6 flex justify-center items-center gap-2 bg-surface-secondary border border-border rounded-md">
            <div className="xl:text-xl font-semibold text-content/60">
              {MIN_STAKE_AMOUNT} GLDT
            </div>
            <div className="flex items-center justify-center rounded-full bg-surface-secondary h-10 w-10 shrink-0 aspect-square">
              <Logo name="gldt" className="p-1" />
            </div>
          </div>
          <div className="mt-4 inline-flex flex-col gap-2">
            <div className="flex items-center gap-2 px-2 py-1 bg-surface-secondary rounded-md">
              <div className="text-content/60 text-sm">
                Min Stake: {MIN_STAKE_AMOUNT} GLDT
              </div>
              <Logo name="gldt" className="w-4 h-4" />
            </div>
            <div className="flex items-center gap-2 px-2 py-1 bg-surface-secondary rounded-md">
              <div className="text-content/60 text-sm">
                Unlock Delay: 1 week
              </div>
            </div>
          </div>
          {isConnected && (
            <Button
              disabled={true}
              className="mt-4 w-full px-4 h-[52px] bg-secondary rounded-md"
            >
              <div className="flex justify-center items-center gap-2">
                <div className="sr-only">Loading...</div>
                <div className="h-2 w-2 bg-white rounded-full animate-bounce [animation-delay:-0.3s]" />
                <div className="h-2 w-2 bg-white rounded-full animate-bounce [animation-delay:-0.15s]" />
                <div className="h-2 w-2 bg-white rounded-full animate-bounce" />
              </div>
            </Button>
          )}
        </div>
        {!isConnected && (
          <div className="absolute bottom-0 w-full h-100 bg-gradient-to-t from-background xl:from-surface-primary to-transparent" />
        )}
      </div>
    );
  }

  return (
    <>
      <Form
        className="p-4 bg-surface-primary border border-border rounded-xl"
        balance={balance.data}
        fee={fee.data}
        decimals={decimals.data}
      />
      <div className="mt-4 flex justify-center">
        <div className="px-2 py-1 flex items-center gap-2 border border-border rounded-md bg-surface-secondary">
          <div className="text-content/60 text-sm">
            Your balance:{" "}
            <TokenValueToLocaleString
              value={balance.data}
              tokenDecimals={decimals.data}
            />{" "}
            GLDT
          </div>
          <Logo name="gldt" className="w-4 h-4" />
        </div>
      </div>

      <Dialog
        open={stakeState.is_open_stake_dialog_confirm}
        handleOnClose={() => dispatchStake({ type: "CANCEL" })}
        title="Confirm stake"
      >
        <Confirm />
      </Dialog>

      <Dialog
        open={stakeState.is_open_stake_dialog_details}
        handleOnClose={() => dispatchStake({ type: "RESET" })}
        title="Stake details"
      >
        <Details />
      </Dialog>
    </>
  );
};

export default StakeForm;
