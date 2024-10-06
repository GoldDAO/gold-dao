import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import {
  SWAP_CANISTER_ID,
  REVERSE_GLDT_TX_FEE,
  GLDT_DECIMAL,
  GLDT_VALUE_1G_NFT,
} from "@constants";

import { useAuth } from "@context/auth";

import { Result_4, Args_2 } from "@canisters/gldt_swap/interfaces";
import {
  ApproveArgs,
  Result_2,
  AllowanceArgs,
} from "@canisters/ledger/interfaces";

import { useNft } from "@context/index";

export const useReverseSwap = () => {
  const { state, getActor } = useAuth();
  const { principalId } = state;

  const { getCollectionSelectedNFTs } = useNft();
  const selected = getCollectionSelectedNFTs();

  const icrc2_approve = async (arg: ApproveArgs): Promise<Result_2> => {
    const actor = getActor("gldt_ledger");
    const result = await actor.icrc2_approve(arg);
    return result as Result_2;
  };

  const swap_tokens_for_nft = async (token: Args_2): Promise<Result_4> => {
    const actor = getActor("gldt_swap");
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
        expires_at: [],
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
      await Promise.allSettled(
        icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      );
      // if (walletSelected === "bitfinity") {
      //   const bitfinity_adapter = walletList.find(
      //     (adaptor) => adaptor.id === "bitfinity"
      //   );
      //   bitfinity_adapter?.adapter.batchTransactions(
      //     icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      //   );
      // } else {
      //   await Promise.allSettled(
      //     icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      //   );
      // }

      const swapTasks = icrc2_allowance_args.map(async (_, index) => {
        try {
          const swapData = swap_tokens_for_nft_data[index];
          const swapResult = await swap_tokens_for_nft(swapData);
          if ("Err" in swapResult) {
            console.error("Error swap_tokens_for_nft:", swapResult.Err);
            return;
          }
        } catch (error) {
          console.error("Error swap:", error);
        }
      });
      await Promise.allSettled(swapTasks);
    },
  });
};
