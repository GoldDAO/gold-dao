import { useMutation } from "@tanstack/react-query";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";

import { useAuth } from "@auth/index";

import { GLDT_TX_FEE, OGY_TX_FEE } from "@constants";

const getFeeByLedger = (ledger: string): bigint => {
  switch (ledger) {
    case "GLDT":
      return BigInt(GLDT_TX_FEE);
    default:
      return BigInt(OGY_TX_FEE);
  }
};

export const useLedgerTransfer = ({ ledger = "OGY" }: { ledger: string }) => {
  const { createActor } = useAuth();
  const icrc1_transfer = async ({
    ledger,
    amount,
    to,
  }: {
    ledger: string;
    amount: bigint;
    to: string;
  }) => {
    const actor = createActor(`${ledger}_ledger`);

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

  return useMutation({
    mutationFn: async ({ amount, to }: { amount: bigint; to: string }) =>
      icrc1_transfer({
        amount,
        to,
        ledger: ledger.toLocaleLowerCase(),
      }),
  });
};
