import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "@services/nft/idlFactory";
import icrc37_approve_tokens from "@services/nft/icrc37_approve_tokens";
import { ApproveTokenArg } from "@services/nft/interfaces";

const useApproveNFT = (
  canister_id: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({
      token_ids,
      spender,
    }: {
      token_ids: bigint[];
      spender: { owner: string; subaccount?: Uint8Array<ArrayBufferLike> | [] };
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId: canister_id,
        });

        const params = token_ids.map((token_id) => {
          const approval_info = {
            memo: [],
            from_subaccount: [],
            created_at_time: BigInt(Date.now()) * 1_000_000n,
            expires_at: [],
            spender: {
              owner: Principal.fromText(spender.owner),
              subaccount: [],
            },
          };
          return {
            token_id,
            approval_info,
          };
        });

        const result = await icrc37_approve_tokens(
          actor,
          params as ApproveTokenArg[]
        );
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`icrc37_approve error! Please retry later.`);
      }
    },
  });
};

export default useApproveNFT;
