import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";

import { TransferArgs, TransferResult } from "@canisters/gld_nft/interfaces";
import { ApproveArgs, Result_2 } from "@canisters/ledger/interfaces";

import { NftCollection } from "@context/index";

export const useTransferNFT = () => {
  const { createActor } = useAuth();

  return useMutation({
    mutationKey: ["TRANSFER_NFT"],
    mutationFn: async ({
      to,
      nfts,
      fee,
    }: {
      to: string;
      nfts: NftCollection[];
      fee: number;
    }): Promise<void> => {
      const icrc2_approve = async (arg: ApproveArgs): Promise<Result_2> => {
        const actor = createActor("ogy_ledger", { authenticated: true });
        const result = await actor.icrc2_approve(arg);
        return result as Result_2;
      };

      const icrc7_transfer = async (arg: {
        canister: string;
        tokenIds: TransferArgs[];
      }): Promise<TransferResult> => {
        const { canister, tokenIds } = arg;
        const actor = createActor(canister, { authenticated: true });
        const result = await actor.icrc7_transfer(tokenIds);
        return result as TransferResult;
      };

      const icrc2_approve_args = nfts.flatMap((nft) => {
        const amount = BigInt(nft.tokenIds.length * Math.floor(fee * 10 ** 8));
        return {
          amount,
          fee: [],
          memo: [],
          expected_allowance: [],
          created_at_time: [],
          expires_at: [],
          spender: {
            owner: Principal.fromText(nft.canisterId),
            subaccount: [],
          },
          from_subaccount: [],
        };
      }) as ApproveArgs[];

      const icrc7_transfer_args = nfts.map((nft) => {
        const tokenIds = nft.tokenIds.map((tokenId) => {
          return {
            to: {
              owner: Principal.fromText(to),
              subaccount: [],
            },
            token_id: tokenId.id_bigint,
            memo: [],
            from_subaccount: [],
            created_at_time: [],
          };
        });
        return {
          canister: nft.canister,
          tokenIds: tokenIds as TransferArgs[],
        };
      });

      // console.groupCollapsed(
      //   "Transfer fn w/ icrc2_approve & icrc7_transfer args"
      // );
      // console.log({ icrc2_approve_args });
      // console.log({ icrc7_transfer_args });
      // console.groupEnd();

      const approve = await Promise.allSettled(
        icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      );
      const approveErrors = approve.filter(
        (result) => result.status === "rejected"
      );
      if (approveErrors.length > 0) {
        console.error({ approveErrors });
        throw new Error(
          "Transfer error! One or more approve transactions failed."
        );
      }
      // console.log({ "Approve results": approve });

      const transfer = await Promise.allSettled(
        icrc7_transfer_args.map(async (arg) => await icrc7_transfer(arg))
      );
      const transferErrors = transfer.filter(
        (result) => result.status === "rejected"
      );
      if (transferErrors.length > 0) {
        console.error({ transferErrors });
        throw new Error(
          "Transfer error! One or more transfer transactions failed."
        );
      }
      // console.log({ "Transfer results": transfer });
    },
  });
};
