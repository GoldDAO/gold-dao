import { useMutation, useQueryClient } from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { Ledger } from "@services/ledger/utils/interfaces";
import { idlFactory } from "../idlFactory";
import { SwapResult, SwapReply } from "../interfaces";

const swap = async (
  actor: ActorSubclass,
  swapArgs: {
    receive_token: Ledger;
    pay_token: Ledger;
    pay_amount: bigint;
    receive_address: string;
    max_slippage: number;
  }
): Promise<SwapReply> => {
  const {
    receive_token,
    pay_token,
    pay_amount,
    receive_address,
    max_slippage,
  } = swapArgs;

  const result = (await actor.swap({
    receive_token: receive_token.toUpperCase(),
    max_slippage: [max_slippage],
    pay_amount,
    referred_by: [],
    receive_amount: [],
    receive_address: [receive_address],
    pay_token: pay_token.toUpperCase(),
    pay_tx_id: [],
  })) as SwapResult;

  if ("Err" in result) throw result.Err;

  return result.Ok;
};

const useSwap = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: {
    pay_token: Ledger;
    receive_token: Ledger;
  }
) => {
  const queryClient = useQueryClient();
  const { pay_token, receive_token } = options;
  return useMutation({
    mutationFn: async ({
      pay_amount,
      receive_address,
      max_slippage,
    }: {
      pay_amount: bigint;
      receive_address: string;
      max_slippage: number;
    }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const swapResult = await swap(actor, {
          receive_token,
          pay_token,
          pay_amount,
          receive_address,
          max_slippage,
        });
        return swapResult;
      } catch (err) {
        console.error(err);
        throw new Error(`swap error! Please retry later.`);
      }
    },
    onSuccess: () => {
      // console.log(res);
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", { ledger: pay_token }],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", { ledger: receive_token }],
      });
      //! TODO Add receive_token
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE_V2"],
      });
    },
  });
};

export default useSwap;
