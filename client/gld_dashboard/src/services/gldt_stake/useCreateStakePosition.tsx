import { useState } from "react";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";

import { idlFactory as LedgerIDL } from "@services/ledger/idlFactory";
import { idlFactory as StakeIDL } from "@services/gldt_stake/idlFactory";

import { Result_2 } from "@services/ledger/interfaces/ledger";
import { Result_2 as CreateStakePositionResponse } from "@services/gldt_stake/interfaces";
import { ApproveArgs } from "@services/ledger/interfaces/ledger";
import { GLDT_STAKE_CANISTER_ID } from "@constants";

interface CreateStakePositionArgs {
  amount: bigint;
}

export const useCreateStakePosition = (
  canisterIdGLDTStake: string,
  canisterIdGLDTLedger: string,
  agent: Agent | HttpAgent | undefined
) => {
  const [error, setError] = useState("");
  const [loading, setLoading] = useState(false);

  const icrc2_approve = async (
    args: CreateStakePositionArgs
  ): Promise<Result_2> => {
    const icrc2_approve_args = {
      amount: args.amount + BigInt(10_000_000),
      fee: [],
      memo: [],
      expected_allowance: [],
      created_at_time: [],
      expires_at: [],
      spender: {
        owner: Principal.fromText(GLDT_STAKE_CANISTER_ID),
        subaccount: [],
      },
      from_subaccount: [],
    } as ApproveArgs;
    const actor = Actor.createActor(LedgerIDL, {
      agent,
      canisterId: canisterIdGLDTLedger,
    });
    console.log(actor);
    const result = await actor.icrc2_approve(icrc2_approve_args);
    return result as Result_2;
  };

  const create_stake_position = async (args: CreateStakePositionArgs) => {
    const actor = Actor.createActor(StakeIDL, {
      agent,
      canisterId: canisterIdGLDTStake,
    });
    const args_with_fee = { amount: args.amount + BigInt(10_000_000) };
    const res = await actor.create_stake_position(args_with_fee);
    return res as CreateStakePositionResponse;
  };

  const createPosition = async (args: CreateStakePositionArgs) => {
    setLoading(true);
    setError("");
    try {
      const approve_result = await icrc2_approve(args);
      console.log(approve_result);
      const create_position_result = await create_stake_position(args);
      console.log(create_position_result);
      //   setData(res);
    } catch (err) {
      //   setError(err);
      console.log(err);
      throw err;
    } finally {
      setLoading(false);
    }
  };

  return { createPosition, loading, error };
};
