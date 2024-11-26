import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Buffer } from "buffer";
import { encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Principal } from "@dfinity/principal";

type UseGetFullAccount = Omit<
  UseQueryOptions<string>,
  "queryKey" | "queryFn"
> & {
  owner: string;
  subaccount?: string | undefined;
};

export const useGetFullAccount = ({
  owner,
  subaccount,
  ...queryParams
}: UseGetFullAccount) => {
  return useQuery({
    queryKey: [`GET_FULL_ACCOUNT`, owner, subaccount],
    queryFn: async () => {
      try {
        const account = encodeIcrcAccount({
          owner: Principal.fromText(owner),
          subaccount: subaccount
            ? [...Uint8Array.from(Buffer.from(subaccount, "hex"))]
            : [],
        });

        return account;
      } catch (err) {
        console.error(err);
        throw new Error(
          "Get full account error! Please refresh page and/or retry later."
        );
      }
    },
    placeholderData: keepPreviousData,
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval,
  });
};
