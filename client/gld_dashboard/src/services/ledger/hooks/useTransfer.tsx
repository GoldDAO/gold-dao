import { useMutation, useQueryClient } from "@tanstack/react-query";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
import { idlFactory } from "../idlFactory";
import { idlFactory as idlFactoryICP } from "../idlFactoryICP";
import { Result } from "../interfaces/ledger";
import { Ledger } from "../utils/interfaces";

const icrc1_transfer = async (
  actor: ActorSubclass,
  transferArgs: { amount: bigint; account: string; fee: bigint }
) => {
  const { amount, account } = transferArgs;
  const decodedAccount = decodeIcrcAccount(account);
  const owner = decodedAccount.owner;
  const subaccount = decodedAccount?.subaccount
    ? [decodedAccount.subaccount]
    : [];

  const result = await actor.icrc1_transfer({
    to: {
      owner,
      subaccount,
    },
    fee: [],
    memo: [],
    from_subaccount: [],
    created_at_time: [],
    amount: amount,
  });
  return result;
};

const send_dfx = async (
  actor: ActorSubclass,
  transferArgs: { amount: bigint; account: string; fee: bigint; memo?: bigint }
) => {
  const { amount, account, memo, fee } = transferArgs;
  const result = await actor.send_dfx({
    to: account,
    fee: {
      e8s: 200000n,
    },
    memo: memo ?? 0n,
    from_subaccount: [],
    created_at_time: [],
    amount: { e8s: amount },
  });
  return result;
};

const useTransfer = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined,
  options: {
    ledger: Ledger;
    is_principal_standard?: boolean;
  }
) => {
  const queryClient = useQueryClient();
  const { ledger, is_principal_standard = true } = options;
  return useMutation({
    mutationFn: async ({
      amount,
      account,
      fee,
    }: {
      amount: bigint;
      account: string;
      fee: bigint;
    }) => {
      try {
        const actorLedger = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const actorLedgerICP = Actor.createActor(idlFactoryICP, {
          agent,
          canisterId,
        });

        if (!is_principal_standard) {
          await send_dfx(actorLedgerICP, {
            amount,
            account,
            fee,
          });
        } else {
          const icrc1Transfer = (await icrc1_transfer(actorLedger, {
            amount,
            account,
            fee,
          })) as Result;
          if (
            Object.keys(icrc1Transfer)[0] === "Err" &&
            "Err" in icrc1Transfer
          ) {
            throw new Error(Object.keys(icrc1Transfer.Err).toString());
          }
        }
      } catch (err) {
        console.error(err);
        throw new Error("Transfer error! Please retry later.");
      }
    },
    onSuccess: () => {
      queryClient.invalidateQueries({
        queryKey: [`FETCH_LEDGER_BALANCE_${ledger.toUpperCase()}`],
      });
      queryClient.invalidateQueries({
        queryKey: [`FETCH_ACCOUNT_TRANSACTIONS_${ledger.toUpperCase()}`],
      });
    },
  });
};

export default useTransfer;
