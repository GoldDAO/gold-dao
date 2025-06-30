import { useMutation, useQueryClient } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory as idlFactoryNFT } from "@services/gld_nft/idlFactory";
import { TransferError, TransferResult } from "@services/gld_nft/interfaces";

const getTransferErrorMessage = (error: TransferError): string => {
  if ("GenericError" in error) {
    return error.GenericError.message;
  }
  if ("Duplicate" in error) {
    return "Duplicate";
  }
  if ("NonExistingTokenId" in error) {
    return "NonExistingTokenId";
  }
  if ("Unauthorized" in error) {
    return "Unauthorized";
  }
  if ("CreatedInFuture" in error) {
    return "CreatedInFuture";
  }
  if ("TooOld" in error) {
    return "TooOld";
  }
  return "Unknown error";
};

const useTransferNFT = (
  canisterId: string,
  nftCollectionName: string,
  agent: Agent | HttpAgent | undefined
) => {
  const queryClient = useQueryClient();
  return useMutation({
    mutationFn: async ({ to, token_id }: { to: string; token_id: bigint }) => {
      try {
        const actor = Actor.createActor(idlFactoryNFT, {
          agent,
          canisterId,
        });

        const result = (await actor.icrc7_transfer([
          {
            to: {
              owner: Principal.fromText(to),
              subaccount: [],
            },
            token_id,
            memo: [],
            from_subaccount: [],
            created_at_time: [],
          },
        ])) as TransferResult;

        if (
          result[0][0] &&
          "transfer_result" in result[0][0] &&
          result[0][0].transfer_result &&
          "Err" in result[0][0].transfer_result
        ) {
          const error = getTransferErrorMessage(
            result[0][0].transfer_result.Err
          );
          throw new Error(`Transfer NFT error! ${error}`);
        }
        return result;
      } catch (err) {
        console.error(err);
        throw new Error(`Transfer NFT error!`);
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT", nftCollectionName],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_USER_NFT_METRICS"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "GLDT"],
      });
      queryClient.invalidateQueries({
        queryKey: ["FETCH_LEDGER_BALANCE", "OGY"],
      });
    },
  });
};

export default useTransferNFT;
