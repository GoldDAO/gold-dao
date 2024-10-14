import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";

import { useAuth } from "@auth/index";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";

export const useLedgerUserBalance = ({
  ledger = "OGY",
}: {
  ledger: string;
}) => {
  const { isConnected, principalId, createActor } = useAuth();
  const queryKeyName = `USER_FETCH_BALANCE_${ledger}`;

  const icrc1_balance_of = async ({
    owner,
    ledger,
  }: {
    owner: string;
    ledger: string;
  }) => {
    const actor = createActor(`${ledger}_ledger`);
    const result = (await actor.icrc1_balance_of({
      owner: Principal.fromText(owner),
      subaccount: [],
    })) as number;

    const balance = divideBy1e8(result);
    return {
      e8s: result,
      number: balance,
      string: roundAndFormatLocale({ number: balance }),
    };
  };

  return useQuery({
    queryKey: [queryKeyName, principalId],
    queryFn: () =>
      icrc1_balance_of({
        owner: principalId as string,
        ledger: ledger.toLocaleLowerCase(),
      }),
    placeholderData: keepPreviousData,
    enabled: !!isConnected && !!principalId,
  });
};
