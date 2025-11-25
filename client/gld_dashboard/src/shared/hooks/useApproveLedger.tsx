import { useMutation } from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/ledger/idlFactory";
import icrc2_approve from "@services/ledger/icrc2_approve";

const useApproveLedger = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({
      amount,
      spender,
    }: {
      amount: bigint;
      spender: { owner: string; subaccount?: Uint8Array<ArrayBufferLike> | [] };
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const icrc2Approve = await icrc2_approve(actor, {
          amount,
          spender,
        });
        return icrc2Approve;
      } catch (err) {
        console.error(err);
        throw new Error(`Ledger approve error! Please retry later.`);
      }
    },
  });
};

export default useApproveLedger;
