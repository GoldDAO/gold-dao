import Input from "@components/forms/Input";
import { Button } from "@components/index";
import { useCreateStakePosition } from "@services/gldt_stake";
import { useGetPoolStats } from "@services/gldt_stake/useGetPoolStats";
import { useQueryClient } from "@tanstack/react-query";
import { numberToE8s } from "@utils/numbers";
import { useState } from "react";
import { useNavigate } from "react-router-dom";

import { useAuth } from "@auth/index";

import { GLDT_STAKE_CANISTER_ID, GLDT_LEDGER_CANISTER_ID } from "@constants";

export const CreateStakePosition = () => {
  const { authenticatedAgent, principalId, isConnected } = useAuth();
  const { createPosition, loading } = useCreateStakePosition(
    GLDT_STAKE_CANISTER_ID,
    GLDT_LEDGER_CANISTER_ID,
    authenticatedAgent
  );
  const navigate = useNavigate();
  const [intendedStakeAmount, setIntendedStakeAmount] = useState(0);
  const { total_staked_query } = useGetPoolStats(
    GLDT_STAKE_CANISTER_ID,
    authenticatedAgent,
    { owner: principalId, enabled: !!isConnected }
  );
  const queryClient = useQueryClient();

  const handleCreatePosition = async () => {
    const e8s = numberToE8s(`${intendedStakeAmount}`);
    try {
      const res = await createPosition({ amount: e8s });
      console.log(res);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_TOTAL_STAKED"],
      });
      total_staked_query.refetch();
      navigate(`/stake/my-positions`);
    } catch (e) {
      console.log(e);
      console.log(e);
    }
  };

  return (
    <div className="flex justify-center">
      <div className="border border-border rounded-xl bg-surface-primary p-8 h-full shadow-[10px_10px_60px_-20px_rgba(252,148,88,100)] flex flex-col">
        <Input
          type="number"
          placeholder="Stake amount"
          className="border p-4 rounded-xl mb-4"
          min="10"
          onChange={(e: React.ChangeEvent<HTMLInputElement>) => {
            setIntendedStakeAmount(Number(e.target.value));
          }}
        />
        <p className="text-sm text-center">min stake: 10 GLDT</p>
        <p className="text-sm text-center">Dissolve delay: 1 week</p>
        <Button
          className="mt-2 align-center"
          onClick={handleCreatePosition}
          disabled={loading}
          style={{
            opacity: loading ? 0.5 : 1,
          }}
        >
          Stake
        </Button>
      </div>
    </div>
  );
};
