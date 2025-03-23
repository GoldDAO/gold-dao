import { useState } from "react";
import { useQueryClient } from "@tanstack/react-query";
import { toast } from "react-hot-toast";
import { LoaderSpin, Logo, Button, Dialog } from "@components/index";
import useClaimRewards from "@services/sns_rewards/hooks/useClaimRewards";

import { SNS_REWARDS_CANISTER_ID } from "@constants";
import { useAuth } from "@auth/index";

const ClaimOneTokenRewards = ({
  amount,
  token,
  selected,
  handleSelectToken,
}: {
  amount: string;
  token: "GLDGov" | "ICP" | "OGY";
  selected: boolean;
  handleSelectToken: (token: "GLDGov" | "ICP" | "OGY") => void;
}) => {
  return (
    <button className="w-full" onClick={() => handleSelectToken(token)}>
      <div
        className={`relative border ${
          selected
            ? "border-gold/10 bg-accent/5"
            : "border-border bg-surface-primary"
        } rounded-full p-4`}
      >
        <div className="flex items-center justify-center gap-2">
          <Logo className="flex-none h-4" name={token.toLocaleLowerCase()} />
          <div className="font-semibold">{amount}</div>
          <div className="text-content/60">{token}</div>
        </div>
      </div>
    </button>
  );
};

export const ClaimOneNeuronRewards = ({
  neuronId,
  amountGLDGov,
  amountICP,
  amountOGY,
}: {
  neuronId: string;
  amountGLDGov: string;
  amountICP: string;
  amountOGY: string;
}) => {
  const queryClient = useQueryClient();
  const { authenticatedAgent } = useAuth();

  const [isOpen, setIsOpen] = useState(false);
  const [selectedToken, setIsSelectedToken] = useState<{
    GLDGov: boolean;
    ICP: boolean;
    OGY: boolean;
  }>({
    GLDGov: false,
    ICP: false,
    OGY: false,
  });

  const handleSelectToken = (token: "GLDGov" | "ICP" | "OGY") =>
    setIsSelectedToken((prev) => ({
      ...prev,
      [token]: !prev[token],
    }));

  const claim = useClaimRewards(SNS_REWARDS_CANISTER_ID, authenticatedAgent, {
    neuronIds: [neuronId],
  });

  const handleClaimRewards = () => {
    claim.mutate(
      {
        tokens: Object.entries(selectedToken)
          .filter(([, selected]) => selected)
          .map(([token]) => token as "GLDGov" | "ICP" | "OGY"),
      },
      {
        onSuccess: () => {
          queryClient.invalidateQueries({
            queryKey: ["FETCH_USER_GOLD_DAO_NEURONS"],
          });
          toast.success("Support ticket was created");
        },
        onError: (error) => {
          toast.error(error?.message || "Error");
        },
      }
    );
  };

  return (
    <>
      <Button
        onClick={() => setIsOpen(true)}
        className="flex items-center gap-3 px-8"
      >
        <div className="font-semibold">Claim Rewards</div>
      </Button>

      <Dialog size="md" open={isOpen} handleOnClose={() => setIsOpen(false)}>
        <div className="mb-12">
          <div className="text-2xl font-bold text-accent">Claim</div>
          <div className="text-2xl">your tokens</div>
        </div>

        {!claim.isPending && (
          <section>
            <div className="flex flex-col items-center justify-between gap-4">
              <ClaimOneTokenRewards
                amount={amountGLDGov}
                token="GLDGov"
                selected={selectedToken.GLDGov}
                handleSelectToken={handleSelectToken}
              />
              <ClaimOneTokenRewards
                amount={amountICP}
                token="ICP"
                selected={selectedToken.ICP}
                handleSelectToken={handleSelectToken}
              />
              <ClaimOneTokenRewards
                amount={amountOGY}
                token="OGY"
                selected={selectedToken.OGY}
                handleSelectToken={handleSelectToken}
              />
            </div>

            <div className="flex justify-center mt-6">
              <button
                onClick={() => {
                  const allSelected =
                    selectedToken.GLDGov &&
                    selectedToken.ICP &&
                    selectedToken.OGY;
                  setIsSelectedToken({
                    GLDGov: !allSelected,
                    ICP: !allSelected,
                    OGY: !allSelected,
                  });
                }}
                className="text-sm text-accent"
              >
                {selectedToken.GLDGov && selectedToken.ICP && selectedToken.OGY
                  ? "Unselect All"
                  : "Select All"}
              </button>
            </div>

            <div className="mt-6 mb-6 flex justify-center">
              <Button
                className="flex items-center gap-3 px-8"
                onClick={handleClaimRewards}
                disabled={
                  !selectedToken.GLDGov &&
                  !selectedToken.ICP &&
                  !selectedToken.OGY
                }
              >
                <div className="font-semibold">Claim</div>
              </Button>
            </div>
          </section>
        )}

        {claim.isPending && (
          <div className="py-8 text-center">
            <div className="mb-6">
              Claiming your tokens, this can take few seconds...
            </div>
            <div className="flex justify-center">
              <LoaderSpin />
            </div>
          </div>
        )}
      </Dialog>
    </>
  );
};
