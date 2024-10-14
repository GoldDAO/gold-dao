import { useQuery, keepPreviousData } from "@tanstack/react-query";

import { useAuth } from "@auth/index";
import { divideBy1e8 } from "@utils/numbers";

export const useFetchTransferFeeNft = ({
  nftId,
  canister,
}: {
  nftId: bigint | undefined;
  canister: string;
}) => {
  const { isConnected, principalId, createActor } = useAuth();

  const icrc7_transfer_fee = async ({
    nftId,
    canister,
  }: {
    nftId: bigint | undefined;
    canister: string;
  }): Promise<number | undefined> => {
    const actor = createActor(canister);
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
