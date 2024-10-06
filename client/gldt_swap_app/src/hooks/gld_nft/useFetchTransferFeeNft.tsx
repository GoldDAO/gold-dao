import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { useAuth } from "@context/auth";
import { divideBy1e8 } from "@utils/numbers";

export const useFetchTransferFeeNft = ({
  nftId,
  canister,
}: {
  nftId: bigint | undefined;
  canister: string;
}) => {
  const { state: authState, getActor } = useAuth();
  const { isConnected, principalId } = authState;

  const icrc7_transfer_fee = async ({
    nftId,
    canister,
  }: {
    nftId: bigint | undefined;
    canister: string;
  }): Promise<number | undefined> => {
    const actor = getActor(canister);
    const result = (await actor.icrc7_transfer_fee(
      BigInt(nftId as bigint)
    )) as [bigint];
    return divideBy1e8(result[0]) ?? undefined;
  };

  return useQuery({
    queryKey: ["FETCH_NFT_TRANSFER_FEE", principalId],
    queryFn: () => icrc7_transfer_fee({ nftId, canister }),
    placeholderData: keepPreviousData,
    enabled: !!isConnected && !!nftId,
  });
};
