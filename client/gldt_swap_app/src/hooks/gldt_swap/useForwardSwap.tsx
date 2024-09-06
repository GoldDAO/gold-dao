// import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { NftCollection, useNft } from "@context/index";
import { canisters } from "@providers/Auth";
import { getActor } from "@amerej/artemis-react";
import {
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
  GLDT_TX_FEE,
  GLDT_FORWARD_SWAP_FEE,
} from "@constants";
import { Result_3 } from "@canisters/gldt_swap/interfaces";
import { MarketTransferRequest, MarketTransferResult } from "@hooks/gld_nft";

type TransferBatchNft = {
  canister: string;
  data: MarketTransferRequest[];
};

const market_transfer_batch_nft_origyn = async (
  nfts: TransferBatchNft[]
): Promise<PromiseSettledResult<[MarketTransferResult]>[]> => {
  const promises = nfts.map(async (nft: TransferBatchNft) => {
    const actor = await getActor(
      canisters[nft.canister].canisterId,
      canisters[nft.canister].idlFactory,
      {
        isAnon: false,
      }
    );
    const marketTransferResults = await actor.market_transfer_batch_nft_origyn(
      nft.data
    );
    return marketTransferResults as [MarketTransferResult];
  });

  const results = await Promise.allSettled(promises);
  return results;
};

export const useForwardSwap = () => {
  const { getCollectionSelectedNFTs } = useNft();
  const selected = getCollectionSelectedNFTs();

  // * get selected token ids for swap_nft_for_tokens
  const data_swap_nft_for_tokens = selected.flatMap((nft) =>
    nft.tokenIds.map(
      (tokenId) =>
        [tokenId.id_bigint, Principal.fromText(nft.canisterId)] as [
          bigint,
          Principal,
        ]
    )
  );

  // * helper create data for batch transfer
  const createMarketTransferRequest = (
    tokenId: string,
    nft: NftCollection
  ): MarketTransferRequest => {
    const TOKEN_FEE = BigInt(GLDT_FORWARD_SWAP_FEE);
    const TOKEN_DECIMALS = 8n;
    const TOKEN_SYMBOL = "GLDT";
    const ENDING_TIMEOUT = 180000000000n; // 3 minutes

    return {
      token_id: tokenId,
      sales_config: {
        broker_id: [],
        pricing: {
          ask: [
            [
              {
                token: {
                  ic: {
                    id: [],
                    fee: [TOKEN_FEE],
                    decimals: TOKEN_DECIMALS,
                    canister: Principal.fromText(GLDT_LEDGER_CANISTER_ID),
                    standard: { Ledger: null },
                    symbol: TOKEN_SYMBOL,
                  },
                },
              },
              { buy_now: BigInt(nft.value * GLDT_TX_FEE + 20000000) }, // Todo fetch it from get_swap for each tx (tokens_to_mint -> value_with_fee)
              { notify: [Principal.fromText(SWAP_CANISTER_ID)] },
              { fee_schema: "com.origyn.royalties.fixed" },
              { allow_list: [Principal.fromText(SWAP_CANISTER_ID)] },
              {
                ending: {
                  timeout: ENDING_TIMEOUT,
                },
              },
            ],
          ],
        },
        escrow_receipt: [],
      },
    };
  };

  // * create batch transfer
  const data_swap_market_transfer_batch_nft_origyn: TransferBatchNft[] =
    selected.map((nft) => {
      const data = nft.tokenIds.map((tokenId) =>
        createMarketTransferRequest(tokenId.id_string, nft)
      );
      const result = {
        canister: nft.canister,
        data: data,
      };
      return result as TransferBatchNft;
    }) as TransferBatchNft[];

  return useMutation({
    mutationKey: ["GLD_NFT_SWAP_GLDT"],
    mutationFn: async (): Promise<void> => {
      const actorSwap = await getActor(
        canisters["gldt_swap"].canisterId,
        canisters["gldt_swap"].idlFactory,
        {
          isAnon: false,
        }
      );

      // * add intent to swap
      // console.log(data_swap_nft_for_tokens);
      const swap_nft_for_tokens_results = (await actorSwap.swap_nft_for_tokens(
        data_swap_nft_for_tokens
      )) as Result_3;
      console.log(swap_nft_for_tokens_results);

      if ("Err" in swap_nft_for_tokens_results) {
        const error = Object.keys(swap_nft_for_tokens_results.Err)[0];
        throw new Error(error);
      }

      // * initiate swap
      if (swap_nft_for_tokens_results.Ok) {
        // console.log(data_swap_market_transfer_batch_nft_origyn);
        await market_transfer_batch_nft_origyn(
          data_swap_market_transfer_batch_nft_origyn as TransferBatchNft[]
        );
      }
    },
  });
};

// const getSwaps = async (actorSwap: ActorSubclass, swapIntents: Result_3[]) => {
//   let allResultsReady = false;

//   while (!allResultsReady) {
//     const swapPromises = swapIntents.map(async (intent: Result_3) => {
//       const swapResult = await actorSwap.get_swap(intent);
//       return swapResult as [[bigint, bigint], SwapInfo];
//     });
//     const results = await Promise.allSettled(swapPromises);

//     allResultsReady = results.every((result) => {
//       if (result.status === "fulfilled") {
//         const swapResult = result.value[0][1];
//         console.log(swapResult);
//         const status = swapResult?.Forward?.status;
//         console.log(status);
//         if (status && ("Init" in status || "Failed" in status)) {
//           return true;
//         }
//       }
//       return false;
//     });

//     if (!allResultsReady) {
//       console.log("Not all swaps are ready, retrying...");
//       await new Promise((resolve) => setTimeout(resolve, 5000));
//     } else {
//       console.log("All swaps are either Init or Failed.");
//     }
//   }
//   console.log("All swap operations have completed.");
// };

// const getInitSwaps = async (
//   actorSwap: ActorSubclass,
//   swapIntents: Result_3[]
// ): Promise<boolean> => {
//   const checkSwaps = async (): Promise<boolean> => {
//     const swapPromises = swapIntents.map(async (intent: Result_3) => {
//       const swapResult = await actorSwap.get_swap(intent);
//       return swapResult as [[[bigint, bigint], SwapInfo]];
//     });

//     const results = await Promise.allSettled(swapPromises);

//     const status = results.map((result) => {
//       if (result.status === "fulfilled") {
//         const swapResult = result.value[0][1];
//         if ("Forward" in swapResult) {
//           const status = swapResult?.Forward?.status;
//           return status;
//         }
//       }
//       return { Null: null };
//     });

//     const allStatusFetched = status.every((s) => "Init" in s || "Failed" in s);

//     if (!allStatusFetched) {
//       await new Promise((resolve) => setTimeout(resolve, 1000));
//       return await checkSwaps();
//     } else {
//       const statusInit = status.every((s) => "Init" in s);
//       if (statusInit) {
//         return true;
//       } else {
//         return false;
//       }
//     }
//   };
//   return await checkSwaps();
// };

// const addIntent2Swap = async (
//   actorSwap: ActorSubclass,
//   nfts: Result_3[]
// ): Promise<PromiseSettledResult<SwapResult>[]> => {
//   const refetch = async (): Promise<PromiseSettledResult<SwapResult>[]> => {
//     const promises = nfts.map(async (nft: Result_3) => {
//       const result = await actorSwap.get_swap(nft);
//       return result as [[[bigint, bigint], SwapInfo]];
//     });

//     const results = await Promise.allSettled(promises);

//     const fulfilled = results.every((result) => result.status === "fulfilled");

//     if (!fulfilled) {
//       await new Promise((resolve) => setTimeout(resolve, 1000));
//       return await refetch();
//     } else {
//       return results;
//     }
//   };
//   return await refetch();
// };
