import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { getActor, useWallet } from "@amerej/artemis-react";

import {
  SWAP_CANISTER_ID,
  REVERSE_GLDT_TX_FEE,
  GLDT_DECIMAL,
  GLDT_VALUE_1G_NFT,
} from "@constants";

import { Result_4, Args_2 } from "@canisters/gldt_swap/interfaces";
import {
  ApproveArgs,
  Result_2,
  AllowanceArgs,
  Result_1,
} from "@canisters/ledger/interfaces";

import { canisters } from "@providers/Auth";
import { useNft } from "@context/index";

const icrc2_allowance = async (arg: AllowanceArgs): Promise<Result_1> => {
  const { canisterId, idlFactory } = canisters["gldt_ledger"];
  const actor = await getActor(canisterId, idlFactory, { isAnon: false });
  const result = await actor.icrc2_allowance(arg);
  return result as Result_1;
};

const icrc2_approve = async (arg: ApproveArgs): Promise<Result_2> => {
  const { canisterId, idlFactory } = canisters["gldt_ledger"];
  const actor = await getActor(canisterId, idlFactory, { isAnon: false });
  const result = await actor.icrc2_approve(arg);
  return result as Result_2;
};

const swap_tokens_for_nft = async (token: Args_2): Promise<Result_4> => {
  const actor = await getActor(
    canisters["gldt_swap"].canisterId,
    canisters["gldt_swap"].idlFactory,
    { isAnon: false }
  );
  const result = await actor.swap_tokens_for_nft(token);
  return result as Result_4;
};

export const useReverseSwap = () => {
  const { principalId } = useWallet();
  const { getCollectionSelectedNFTs } = useNft();
  const selected = getCollectionSelectedNFTs();

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
      const swapTasks = icrc2_allowance_args.map(
        async (allowanceArg, index) => {
          try {
            const allowanceResult = await icrc2_allowance(allowanceArg);
            if ("Err" in allowanceResult) {
              console.error("Error icrc2_allowance:", allowanceResult.Err);
              return;
            }

            const approveArg = icrc2_approve_args[index];
            const approveResult = await icrc2_approve(approveArg);
            if ("Err" in approveResult) {
              console.error("Error icrc2_approve:", approveResult.Err);
              return;
            }

            const swapData = swap_tokens_for_nft_data[index];
            const swapResult = await swap_tokens_for_nft(swapData);
            if ("Err" in swapResult) {
              console.error("Error swap_tokens_for_nft:", swapResult.Err);
              return;
            }

            console.log("Success swap:", swapData);
          } catch (error) {
            console.error("Error swap:", error);
          }
        }
      );
      await Promise.all(swapTasks);
    },
  });
};

// import { useMutation } from "@tanstack/react-query";
// import { Principal } from "@dfinity/principal";
// import { getActor } from "@amerej/artemis-react";

// import {
//   SWAP_CANISTER_ID,
//   REVERSE_GLDT_TX_FEE,
//   GLDT_DECIMAL,
//   GLDT_VALUE_1G_NFT,
// } from "@constants";

// import { Result_4, Args_2 } from "@canisters/gldt_swap/interfaces";
// import { ApproveArgs, Result_2 } from "@canisters/ledger/interfaces";

// import { canisters } from "@providers/Auth";

// import { useNft } from "@context/index";

// const swap_tokens_for_nft = async (
//   tokens: Args_2[]
// ): Promise<PromiseSettledResult<Result_4>[]> => {
//   const promises = tokens.map(async (token: Args_2) => {
//     const actor = await getActor(
//       canisters["gldt_swap"].canisterId,
//       canisters["gldt_swap"].idlFactory,
//       {
//         isAnon: false,
//       }
//     );
//     const results = await actor.swap_tokens_for_nft(token);
//     return results as Result_4;
//   });

//   const results = await Promise.allSettled(promises);
//   return results;
// };

// const icrc2_approve = async (
//   args: ApproveArgs[]
// ): Promise<PromiseSettledResult<Result_2>[]> => {
//   const promises = args.map(async (arg: ApproveArgs) => {
//     const { canisterId, idlFactory } = canisters["gldt_ledger"];
//     const actor = await getActor(canisterId, idlFactory, {
//       isAnon: false,
//     });
//     const results = await actor.icrc2_approve(arg);
//     return results as Result_2;
//   });

//   const results = await Promise.allSettled(promises);
//   return results;
// };

// export const useReverseSwap = () => {
//   const { getCollectionSelectedNFTs } = useNft();
//   const selected = getCollectionSelectedNFTs();

//   // * get selected token ids for swap_tokens_for_nft
//   const swap_tokens_for_nft_data = selected.flatMap((nft) =>
//     nft.tokenIds.map((tokenId) => {
//       return {
//         nft_id: tokenId.id_bigint,
//         nft_canister_id: Principal.fromText(nft.canisterId),
//       };
//     })
//   );

//   // * get selected token ids for icrc2_approve_args
//   const icrc2_approve_args = selected.flatMap((nft) =>
//     nft.tokenIds.map((tokenId) => {
//       return {
//         amount: BigInt(
//           nft.value * GLDT_VALUE_1G_NFT * GLDT_DECIMAL + REVERSE_GLDT_TX_FEE
//         ),
//         fee: [],
//         memo: [],
//         expected_allowance: [],
//         created_at_time: [],
//         expires_at: [],
//         spender: {
//           owner: Principal.fromText(SWAP_CANISTER_ID),
//           subaccount: [tokenId.id_byte_array],
//         },
//         from_subaccount: [],
//       };
//     })
//   ) as ApproveArgs[];

//   return useMutation({
//     mutationKey: ["REVERSE_SWAP"],
//     mutationFn: async (): Promise<void> => {
//       const icrc2_approve_results = await icrc2_approve(icrc2_approve_args);
//       console.log(icrc2_approve_results);
//       // if ("Err" in icrc2_approve_results) {
//       //   const error = Object.keys(icrc2_approve_results.Err)[0];
//       //   throw new Error(error);
//       // }

//       // * add intent to swap
//       const swap_tokens_for_nft_results = await swap_tokens_for_nft(
//         swap_tokens_for_nft_data
//       );
//       console.log(swap_tokens_for_nft_results);
//       // if ("Err" in swap_tokens_for_nft_results) {
//       //   const error = Object.keys(swap_tokens_for_nft_results.Err)[0];
//       //   throw new Error(error);
//       // }
//     },
//   });
// };
