import { useMutation } from "@tanstack/react-query";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { getActor } from "@amerej/artemis-react";

import { canisters } from "@providers/Auth";
import {GLDT_TX_FEE, OGY_TX_FEE} from '@constants'

const getFeeByLedger = (ledger:string): bigint => {
  switch(ledger) {
    case "GLDT":
      return BigInt(GLDT_TX_FEE)
    default:
      return BigInt(OGY_TX_FEE)
  }
}

const icrc1_transfer = async ({
  ledger,
  amount,
  to,
}: {
  ledger: string;
  amount: bigint;
  to: string;
}) => {
  const { canisterId, idlFactory } = canisters[`${ledger}_ledger`];
  const actor = await getActor(canisterId, idlFactory, {
    isAnon: false,
  });

  const decodedAccount = decodeIcrcAccount(to);
  const owner = decodedAccount.owner;
  const subaccount = decodedAccount?.subaccount
    ? [decodedAccount.subaccount]
    : [];

  const result = await actor.icrc1_transfer({
    to: { owner: owner, subaccount: subaccount },
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount: amount - getFeeByLedger(ledger),
  });
  return result;
};

export const useLedgerTransfer = ({ ledger = "OGY" }: { ledger: string }) => {
  return useMutation({
    mutationFn: ({
      amount,
      to,
    }: {
      amount: bigint;
      to: string;
    }) =>
      icrc1_transfer({
        amount,
        to,
        ledger: ledger.toLocaleLowerCase(),
      }),
  });
};
