// import { useState } from "react";
import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { NftCollection, useNft } from "@context/index";

import {
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
  GLDT_TX_FEE,
  GLDT_VALUE_1G_NFT,
  GLDT_DECIMAL,
} from "@constants";

import { useAuth } from "@auth/index";

import { Result_3 } from "@canisters/gldt_swap/interfaces";
import {
  MarketTransferRequest,
  MarketTransferResult,
} from "@canisters/gld_nft/interfaces";

type TransferBatchNft = {
  canister: string;
  data: MarketTransferRequest[];
};

export const useForwardSwap = () => {
  const { createActor } = useAuth();
  const { getCollectionSelectedNFTs } = useNft();

  return useMutation({
    mutationKey: ["GLD_NFT_SWAP_GLDT"],
    mutationFn: async (): Promise<void> => {
      try {
        const selected = getCollectionSelectedNFTs();
        const actorSwap = createActor("gldt_swap", { authenticated: true });

        const market_transfer_batch_nft_origyn = async (
          nfts: TransferBatchNft[]
        ): Promise<PromiseSettledResult<[MarketTransferResult]>[]> => {
          const promises = nfts.map(async (nft: TransferBatchNft) => {
            const actor = createActor(nft.canister, { authenticated: true });
            const marketTransferResults =
              await actor.market_transfer_batch_nft_origyn(nft.data);
            return marketTransferResults as [MarketTransferResult];
          });

          const results = await Promise.allSettled(promises);
          return results;
        };

        // * get selected token ids for swap_nft_for_tokens
        const data_swap_nft_for_tokens = selected.flatMap((nft) =>
          nft.tokenIds.map(
            (tokenId) =>
              [tokenId.id_bigint, Principal.fromText(nft.canisterId)] as [
                bigint,
                Principal
              ]
          )
        );

        // * helper create data for batch transfer
        const createMarketTransferRequest = (
          tokenId: string,
          nft: NftCollection
        ): MarketTransferRequest => {
          const TOKEN_FEE = BigInt(GLDT_TX_FEE);
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
                    {
                      buy_now: BigInt(
                        nft.value * GLDT_VALUE_1G_NFT * GLDT_DECIMAL +
                          2 * GLDT_TX_FEE
                      ),
                    }, // todo fetch it from get_swap for each tx (tokens_to_mint -> value_with_fee)
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

        // * add intent to swap
        const swap_nft_for_tokens_results =
          (await actorSwap.swap_nft_for_tokens(
            data_swap_nft_for_tokens
          )) as Result_3;

        if ("Err" in swap_nft_for_tokens_results) {
          const swap_nft_for_tokens_errors = Object.keys(
            swap_nft_for_tokens_results.Err
          )[0];

          if (swap_nft_for_tokens_errors === "Retry") {
            throw new Error("retry", {
              cause: {
                retryDelay: Math.round(
                  Number(swap_nft_for_tokens_errors[0]) / 1000
                ),
              },
            });
          } else {
            throw new Error("swap_nft_for_tokens.", {
              cause: swap_nft_for_tokens_errors,
            });
          }
        }

        // * initiate swap
        if ("Ok" in swap_nft_for_tokens_results) {
          const market_transfer_batch_nft_origyn_results =
            await market_transfer_batch_nft_origyn(
              data_swap_market_transfer_batch_nft_origyn as TransferBatchNft[]
            );

          if ("err" in market_transfer_batch_nft_origyn_results) {
            const market_transfer_batch_nft_origyn_errors =
              market_transfer_batch_nft_origyn_results.err;
            throw new Error("market_transfer_batch_nft_origyn", {
              cause: market_transfer_batch_nft_origyn_errors,
            });
          }
        }
      } catch (err) {
        if (err instanceof Error) {
          switch (err.message) {
            case "retry":
              throw err;
            case "swap_nft_for_tokens":
              console.error({ swap_nft_for_tokens: err.cause });
              throw new Error(
                "Error when swapping GLD NFTs! swap_nft_for_tokens failure."
              );
            case "market_transfer_batch_nft_origyn":
              console.error({
                market_transfer_batch_nft_origyn: err.cause,
              });
              throw new Error(
                "Error when swapping GLD NFTs! market_transfer_batch_nft_origyn failure."
              );
            default:
              console.error(err);
              throw new Error(
                "Error when swapping GLD NFTs! Unexpected error."
              );
          }
        }
      }
    },
  });
};
