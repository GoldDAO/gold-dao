// import { useState } from "react";

import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import {
  GLDT_LEDGER_CANISTER_ID,
  SWAP_CANISTER_ID,
  GLDT_VALUE_1G_NFT,
} from "@constants";
import { idlFactory } from "../idlFactory";
import market_transfer_nft_origyn from "../fn/market_transfer_nft_origyn";
import { IdNFT } from "@services/gld_nft/utils/interfaces";

const useMarketTransferNFTOrigyn = (
  agent: Agent | HttpAgent | undefined,
  options: { collection_name: string }
) => {
  const queryClient = useQueryClient();
  const { collection_name } = options;
  return useMutation({
    mutationKey: ["MARKET_TRANSFER_BATCH_NFT_ORIGYN"],
    mutationFn: async ({
      nft,
      collection_canister_id,
      collection_value,
    }: {
      nft: IdNFT;
      collection_canister_id: string;
      collection_value: number;
    }): Promise<void> => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId: collection_canister_id,
        });
        const GLDT_TX_FEE = 10000000n; // todo fetch it from be
        const GLDT_DECIMAL = 8; // todo fetch it from be
        const TOKEN_SYMBOL = "GLDT"; // todo fetch it from be

        await market_transfer_nft_origyn(actor, {
          token_id: nft.id_string,
          sales_config: {
            broker_id: [],
            pricing: {
              ask: [
                [
                  {
                    token: {
                      ic: {
                        id: [],
                        fee: [GLDT_TX_FEE],
                        decimals: BigInt(GLDT_DECIMAL),
                        canister: Principal.fromText(GLDT_LEDGER_CANISTER_ID),
                        standard: { Ledger: null },
                        symbol: TOKEN_SYMBOL,
                      },
                    },
                  },
                  {
                    buy_now: BigInt(
                      collection_value *
                        GLDT_VALUE_1G_NFT *
                        10 ** GLDT_DECIMAL +
                        2 * Number(GLDT_TX_FEE)
                    ),
                  }, // todo fetch it from get_swap for each tx (tokens_to_mint -> value_with_fee)
                  { notify: [Principal.fromText(SWAP_CANISTER_ID)] },
                  { fee_schema: "com.origyn.royalties.fixed" },
                  {
                    allow_list: [Principal.fromText(SWAP_CANISTER_ID)],
                  },
                  {
                    ending: {
                      timeout: BigInt(3 * 60 * 1000000000), // 3 minutes
                    },
                  },
                ],
              ],
            },
            escrow_receipt: [],
          },
        });
      } catch (err) {
        console.error(err);
        if (err instanceof Error) {
          throw new Error(err.message, { cause: err });
        } else {
          throw new Error("An unknown error occurred", { cause: err });
        }
      }
    },
    onSuccess: (res) => {
      console.log("Market transfer NFT Origyn success");
      console.log(res);
    },
    onSettled: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT", collection_name],
      });
      queryClient.invalidateQueries({
        queryKey: [`FETCH_LEDGER_BALANCE_GLDT`],
      });
    },
  });
};

export default useMarketTransferNFTOrigyn;
