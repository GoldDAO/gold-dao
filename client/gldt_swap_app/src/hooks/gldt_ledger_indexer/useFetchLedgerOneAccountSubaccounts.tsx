import {
  keepPreviousData,
  useQuery,
  UseQueryOptions,
} from "@tanstack/react-query";
import { ActorSubclass } from "@dfinity/agent";
import { Principal } from "@dfinity/principal";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";
import { Buffer } from "buffer";

import { useAuth } from "@auth/index";
import { SubAccount } from "@canisters/gldt_ledger_indexer/interface";

export interface FetchSubaccountsParams {
  start?: string;
  owner: string;
}

export const fetchSubaccounts = async ({
  actor,
  owner,
  start,
}: FetchSubaccountsParams & {
  actor: ActorSubclass;
}): Promise<string[]> => {
  const account = encodeIcrcAccount({
    owner: Principal.fromText(owner),
    subaccount: [],
  });

  const decodedAccount = decodeIcrcAccount(account);
  const _owner = decodedAccount.owner;

  const results = (await actor.list_subaccounts({
    start: start ? [BigInt(start)] : [],
    owner: _owner,
  })) as Array<SubAccount>;

  const subaccounts = results.map((s) => {
    return Buffer.from(s).toString("hex");
  });

  return subaccounts;
};

export const useFetchLedgerOneAccountSubaccounts = ({
  owner,
  start = undefined,
  ...queryParams
}: Omit<UseQueryOptions<string[]>, "queryKey" | "queryFn"> &
  FetchSubaccountsParams) => {
  const { createActor } = useAuth();

  return useQuery({
    queryKey: ["FETCH_LEDGER_ONE_ACCOUNT_SUBACCOUNTS", owner],
    queryFn: async (): Promise<string[]> => {
      const actor = createActor("gldt_ledger_indexer");
      try {
        const results = await fetchSubaccounts({
          actor,
          owner,
          start,
        });
        return results;
      } catch (err) {
        console.error(err);
        throw new Error(
          "Fetch account subaccounts error! Please refresh page and/or retry later."
        );
      }
    },
    placeholderData: keepPreviousData,
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval ?? undefined,
  });
};
