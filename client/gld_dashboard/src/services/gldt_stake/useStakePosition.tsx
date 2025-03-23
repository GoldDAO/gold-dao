import { useState } from "react";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory } from "./idlFactory";

export const useStakePosition = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);
  const [loading_positions, set_position_loading] = useState<bigint[]>([]);

  const actor = Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });

  const addLoadingPosition = (id: bigint) => {
    set_position_loading((prev) => [...prev, id]);
  };

  const removeLoadingPosition = (id: bigint) => {
    set_position_loading((prev) =>
      prev.filter((positionId) => positionId !== id)
    );
  };

  const isPositionLoading = (id: bigint) => loading_positions.includes(id);

  const startDissolving = async (position_id: bigint) => {
    setLoading(true);
    setError("");
    addLoadingPosition(position_id);
    try {
      const res = await actor.start_dissolving(position_id);
      console.log(res);
    } catch (err) {
      console.log(err);
      throw err;
    } finally {
      removeLoadingPosition(position_id);

      setLoading(false);
    }
  };

  const claimReward = async (position_id: bigint, token: string) => {
    setLoading(true);
    addLoadingPosition(position_id);
    setError("");
    try {
      const res = await actor.claim_reward({ id: position_id, token });
      console.log(res);
    } catch (err) {
      console.log(err);
      throw err;
    } finally {
      removeLoadingPosition(position_id);
      setLoading(false);
    }
  };
  const unstake = async (position_id: bigint) => {
    setLoading(true);
    setError("");
    addLoadingPosition(position_id);
    try {
      const res = await actor.unstake(position_id);
      console.log(res);
    } catch (err) {
      console.log(err);
      throw err;
    } finally {
      removeLoadingPosition(position_id);
      setLoading(false);
    }
  };
  const unstakeEarly = async (position_id: bigint) => {
    setLoading(true);
    setError("");
    addLoadingPosition(position_id);
    try {
      const res = await actor.unstake_early(position_id);
      console.log(res);
    } catch (err) {
      console.log(err);
      throw err;
    } finally {
      removeLoadingPosition(position_id);
      setLoading(false);
    }
  };

  return {
    startDissolving,
    unstakeEarly,
    unstake,
    claimReward,
    isPositionLoading,
    loading,
    error,
  };
};
