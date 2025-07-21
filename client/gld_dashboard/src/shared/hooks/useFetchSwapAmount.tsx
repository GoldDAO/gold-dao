import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryLedger } from "@services/ledger/idlFactory";
import { idlFactory as idlFactoryKongSwap } from "@services/kongswap/idlFactory";
import { SwapAmountsTxReply } from "@services/kongswap/interfaces";
import swap_amounts from "@services/kongswap//swap_amounts";
import icrc1_decimals from "@services/ledger/icrc1_decimals";

export interface SwapAmountResult {
  pay_amount: bigint;
  receive_amount: bigint;
  slippage_without_tx_fee: number;
  slippage_with_tx_fee: number;
  network_fee: bigint;
  lp_fee: bigint;
  mid_price: number;
  txs: Array<SwapAmountsTxReply>;
}

const useFetchSwapAmount = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: Omit<UseQueryOptions<SwapAmountResult>, "queryKey" | "queryFn"> & {
    from: string;
    from_canister_id: string;
    to: string;
    amount: number;
    key?: string;
  }
) => {
  const {
    enabled = true,
    refetchInterval = false,
    placeholderData = keepPreviousData,
    from,
    to,
    amount,
    key,
    from_canister_id,
    staleTime = 60 * 1000,
    refetchOnMount = true,
    refetchOnWindowFocus = false,
    refetchOnReconnect = true,
    ...queryOptions
  } = options;

  return useQuery({
    queryKey: [`FETCH_SWAP_AMOUNT`, from, to, amount, { ...(key && { key }) }],
    queryFn: async () => {
      try {
        const actorLedger = Actor.createActor(idlFactoryLedger, {
          agent,
          canisterId: from_canister_id,
        });

        const actorKongSwap = Actor.createActor(idlFactoryKongSwap, {
          agent,
          canisterId,
        });

        const decimals = await icrc1_decimals(actorLedger);

        const result = await swap_amounts(actorKongSwap, {
          from,
          to,
          amount: BigInt(Math.round(amount * 10 ** decimals)),
        });

        const network_fee = result.txs.reduce(
          (acc, tx) => acc + tx.gas_fee,
          0n
        );
        const lp_fee = result.txs.reduce((acc, tx) => acc + tx.lp_fee, 0n);

        const ideal_amount =
          Number(result.receive_amount) / (1 - result.slippage / 100);
        const real_amount_of_gldt_without_tx_fee = Number(
          result.receive_amount + network_fee
        );
        const slippage_without_tx_fee =
          ((ideal_amount - real_amount_of_gldt_without_tx_fee) / ideal_amount) *
          100;

        return {
          pay_amount: result.pay_amount,
          receive_amount: result.receive_amount,
          slippage_without_tx_fee,
          slippage_with_tx_fee: result.slippage,
          network_fee,
          lp_fee,
          mid_price: result.mid_price,
          txs: result.txs,
        };
      } catch (err) {
        console.error(err);
        throw new Error(
          `Fetch ${from} to ${to} price error! Please retry later.`
        );
      }
    },
    placeholderData,
    enabled,
    refetchInterval,
    staleTime,
    refetchOnMount,
    refetchOnWindowFocus,
    refetchOnReconnect,
    ...queryOptions,
  });
};

export default useFetchSwapAmount;
