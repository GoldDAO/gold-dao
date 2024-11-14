import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import {
  SWAP_CANISTER_ID,
  REVERSE_GLDT_TX_FEE,
  GLDT_DECIMAL,
  GLDT_VALUE_1G_NFT,
} from "@constants";

import { useAuth } from "@auth/index";

import { Result_4, Args_2 } from "@canisters/gldt_swap/interfaces";
import {
  ApproveArgs,
  Result_2,
  AllowanceArgs,
} from "@canisters/ledger/interfaces";

import { useNft } from "@context/index";
import { DateTime } from "luxon";

export const useReverseSwap = () => {
  const { principalId, createActor } = useAuth();
  const expiresAtApprove =
    BigInt(DateTime.now().plus({ hours: 1 }).toMillis()) * BigInt(1_000_000);

  const { getCollectionSelectedNFTs } = useNft();
  const selected = getCollectionSelectedNFTs();

  const icrc2_approve = async (arg: ApproveArgs): Promise<Result_2> => {
    const actor = createActor("gldt_ledger");
    const result = await actor.icrc2_approve(arg);
    return result as Result_2;
  };

  const swap_tokens_for_nft = async (token: Args_2): Promise<Result_4> => {
    const actor = createActor("gldt_swap");
    const result = await actor.swap_tokens_for_nft(token);
    return result as Result_4;
  };

  const swap_tokens_for_nft_data = selected.flatMap((nft) =>
    nft.tokenIds.map((tokenId) => {
      return {
        nft_id: tokenId.id_bigint,
        nft_canister_id: Principal.fromText(nft.canisterId),
      };
    })
  );

  const icrc2_approve_args = selected.flatMap((nft) =>
    nft.tokenIds.map((tokenId) => {
      return {
        amount: BigInt(
          nft.value * GLDT_VALUE_1G_NFT * GLDT_DECIMAL + REVERSE_GLDT_TX_FEE
        ),
        fee: [],
        memo: [],
        expected_allowance: [],
        created_at_time: [],
        expires_at: [expiresAtApprove],
        spender: {
          owner: Principal.fromText(SWAP_CANISTER_ID),
          subaccount: [tokenId.id_byte_array],
        },
        from_subaccount: [],
      };
    })
  ) as ApproveArgs[];

  const icrc2_allowance_args = selected.flatMap((nft) =>
    nft.tokenIds.map((tokenId) => {
      return {
        account: {
          owner: Principal.fromText(principalId as string),
          subaccount: [],
        },
        spender: {
          owner: Principal.fromText(SWAP_CANISTER_ID),
          subaccount: [tokenId.id_byte_array],
        },
      };
    })
  ) as AllowanceArgs[];

  return useMutation({
    mutationKey: ["REVERSE_SWAP"],
    mutationFn: async (): Promise<void> => {
      // console.log("approve_args:");
      // console.log(icrc2_approve_args);
      const approve = await Promise.allSettled(
        icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      );

      // console.log("approve result:");
      // console.log(approve);

      const approveRejectedErrors = approve.filter(
        (result) => result.status === "rejected"
      );
      const approveFulfilled = approve.filter(
        (result) => result.status === "fulfilled"
      );

      if (
        approveRejectedErrors.length > 0 ||
        approveFulfilled.some((approve) => "Err" in approve.value)
      ) {
        // console.error(approve);
        throw new Error("Approve");
      }

      // console.log("icrc2_allowance_args:");
      // console.log(icrc2_allowance_args);

      const swapTasks = icrc2_allowance_args.map(async (_, index) => {
        const swapData = swap_tokens_for_nft_data[index];
        const swapResult = await swap_tokens_for_nft(swapData);
        return swapResult;
      });

      const swap = await Promise.allSettled(swapTasks);

      console.log("all swaps", swap);
      // console.log("swap result:");
      // console.log(swap);

      const swapRejectedErrors = swap.filter(
        (result) => result.status === "rejected"
      );
      const swapFulfilled = swap.filter(
        (result) => result.status === "fulfilled"
      );

      if (
        swapRejectedErrors.length > 0 ||
        swapFulfilled.some((swap) => "Err" in swap.value)
      ) {
        // console.error(swap);
        const { countErr, countSuccess } = swapFulfilled.reduce(
          (acc, swap) => {
            if ("Err" in swap.value) acc.countErr += 1;
            else if ("Ok" in swap.value) acc.countSuccess += 1;
            return acc;
          },
          { countErr: swapRejectedErrors.length, countSuccess: 0 }
        );
        if (countSuccess === 0) throw new Error("Swap");

        throw new Error(
          `Warning! ${countSuccess} swap${
            countSuccess > 1 ? "s" : ""
          } succeeded and ${countErr} swap${countErr > 1 ? "s" : ""} failed.`
        );
      }
    },
  });
};
