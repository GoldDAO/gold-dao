import {
  useQuery,
  keepPreviousData,
  UseQueryOptions,
} from "@tanstack/react-query";
import { Buffer } from "buffer";
import { decodeIcrcAccount, encodeIcrcAccount } from "@dfinity/ledger-icrc";

import { useAuth } from "@auth/index";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";
import { Principal } from "@dfinity/principal";

interface UseLedgerAccountBalanceResult {
  e8s: bigint;
  number: number;
  string: string;
}

type UseLedgerAccountBalance = Omit<
  UseQueryOptions<UseLedgerAccountBalanceResult>,
  "queryKey" | "queryFn"
> & {
  ledger?: string;
  owner: string;
  subaccount?: string | undefined;
};

export const useLedgerAccountBalance = ({
  ledger = "GLDT",
  owner,
  subaccount,
  ...queryParams
}: UseLedgerAccountBalance) => {
  const { createActor } = useAuth();

  const icrc1_balance_of = async ({
    owner,
    subaccount,
    ledger,
  }: {
    owner: string;
    subaccount?: string | undefined;
    ledger: string;
  }) => {
    const actor = createActor(`${ledger.toLowerCase()}_ledger`);

    const account = encodeIcrcAccount({
      owner: Principal.fromText(owner),
      subaccount: subaccount
        ? [...Uint8Array.from(Buffer.from(subaccount, "hex"))]
        : [],
    });

    const decodedAccount = decodeIcrcAccount(account);
    const _owner = decodedAccount.owner;
    const _subaccount = decodedAccount?.subaccount
      ? [decodedAccount.subaccount]
      : [];

    const result = (await actor.icrc1_balance_of({
      owner: _owner,
      subaccount: _subaccount,
    })) as bigint;

    const balance = divideBy1e8(result);
    return {
      e8s: result,
      number: balance,
      string: roundAndFormatLocale({ number: balance }),
    } as UseLedgerAccountBalanceResult;
  };

  return useQuery({
    queryKey: [`FETCH_LEDGER_ACCOUNT_BALANCE_${ledger}`, owner, subaccount],
    queryFn: () =>
      icrc1_balance_of({
        owner,
        subaccount,
        ledger: ledger.toLocaleLowerCase(),
      }),
    placeholderData: keepPreviousData,
    enabled: queryParams.enabled !== undefined ? queryParams.enabled : true,
    refetchInterval: queryParams.refetchInterval,
  });
};
