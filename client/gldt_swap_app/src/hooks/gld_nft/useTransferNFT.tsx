import { useMutation } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { OGY_TX_FEE } from "@constants";
import { useAuth } from "@context/auth";

import { TransferArgs, TransferResult } from "@canisters/gld_nft/interfaces";
import { ApproveArgs, Result_2 } from "@canisters/ledger/interfaces";

import { NftCollection } from "@context/index";

export const useTransferNFT = () => {
  const { getActor } = useAuth();

  const icrc2_approve = async (arg: ApproveArgs): Promise<Result_2> => {
    const actor = getActor("ogy_ledger");
    const result = await actor.icrc2_approve(arg);
    return result as Result_2;
  };

  const icrc7_transfer = async (arg: {
    canister: string;
    tokenIds: TransferArgs[];
  }): Promise<TransferResult> => {
    const { canister, tokenIds } = arg;
    const actor = getActor(canister);
    const result = await actor.icrc7_transfer(tokenIds);
    return result as TransferResult;
  };

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
      const icrc2_approve_args = nfts.flatMap((nft) => {
        console.log(`approve_args canisterID: ${nft.canisterId}`);
        return nft.tokenIds.map(() => {
          return {
            amount: BigInt(fee * 10 ** 8 + OGY_TX_FEE),
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
        });
      }) as ApproveArgs[];

      console.log("approve_args:");
      console.log(icrc2_approve_args);
      const approve = await Promise.allSettled(
        icrc2_approve_args.map(async (arg) => await icrc2_approve(arg))
      );
      console.log("approve result:");
      console.log(approve);

      const icrc7_transfer_args = nfts.map((nft) => {
        const tokenIds = nft.tokenIds.map((tokenId) => {
          console.log(`transfer_args to address: ${to}`);
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

      console.log("transfer_args:");
      console.log(icrc7_transfer_args);
      const transfer = await Promise.allSettled(
        icrc7_transfer_args.map(async (arg) => await icrc7_transfer(arg))
      );
      console.log("transfer result:");
      console.log(transfer);
    },
  });
};
