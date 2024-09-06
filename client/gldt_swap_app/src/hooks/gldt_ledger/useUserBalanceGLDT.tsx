import { useQuery, keepPreviousData } from "@tanstack/react-query";
import { Principal } from "@dfinity/principal";
import { useWallet, getActor } from "@amerej/artemis-react";

import { canisters } from "@providers/Auth";
import { roundAndFormatLocale, divideBy1e8 } from "@utils/numbers";

const icrc1_balance_of = async ({ owner }: { owner: string }) => {
  const { canisterId, idlFactory } = canisters["gldt_ledger"];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });
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

export const useUserBalanceGLDT = () => {
  const { isConnected, principalId } = useWallet();

  return useQuery({
    queryKey: ["USER_FETCH_BALANCE_GLDT", principalId],
    queryFn: () => icrc1_balance_of({ owner: principalId as string }),
    placeholderData: keepPreviousData,
    enabled: !!isConnected && !!principalId,
  });
};
