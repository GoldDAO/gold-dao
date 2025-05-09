import { useMutation } from "@tanstack/react-query";
// import { Principal } from "@dfinity/principal";
import { decodeIcrcAccount } from "@dfinity/ledger-icrc";
// import { AccountIdentifier } from "@dfinity/ledger-icp";
import { ActorSubclass } from "@dfinity/agent";
import { Actor, Agent, HttpAgent } from "@dfinity/agent";
// import { Buffer } from "buffer";

import { idlFactory } from "../idlFactory";

import { Result } from "../interfaces/ledger";

const icrc1_transfer = async (
  actor: ActorSubclass,
  transferArgs: { amount: bigint; to: string }
) => {
  const { amount, to } = transferArgs;
  const decodedAccount = decodeIcrcAccount(to);
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

// const send_dfx = async (
//   actor: ActorSubclass,
//   transferArgs: { amount: bigint; to: string; fee: bigint; memo?: bigint }
// ) => {
//   const { amount, to, fee, memo } = transferArgs;
//   const _to = AccountIdentifier.fromPrincipal({
//     principal: Principal.fromText(to),
//   }).toHex();

//   const result = await actor.send_dfx({
//     to: _to,
//     fee: {
//       e8s: fee,
//     },
//     memo: memo ?? 0n,
//     from_subaccount: [],
//     created_at_time: [],
//     amount: { e8s: amount },
//   });
//   return result;
// };

const useTransfer = (
  canisterId: string,
  agent: Agent | HttpAgent | undefined
) => {
  return useMutation({
    mutationFn: async ({ amount, to }: { amount: bigint; to: string }) => {
      try {
        const actor = Actor.createActor(idlFactory, {
          agent,
          canisterId,
        });

        const icrc1Transfer = (await icrc1_transfer(actor, {
          amount,
          to,
        })) as Result;

        if (Object.keys(icrc1Transfer)[0] === "Err" && "Err" in icrc1Transfer) {
          throw new Error(Object.keys(icrc1Transfer.Err).toString());
        }

        // if (["gldgov", "ogy", "gldt"].includes(ledger)) {
        //   const icrc1Transfer = await icrc1_transfer(actor, {
        //     amount,
        //     to,
        //     fee: get_fee_by_ledger(ledger),
        //   });
        //   console.log(icrc1Transfer);
        // } else if (ledger === "icp") {
        //   const sendDfx = await send_dfx(actor, {
        //     amount,
        //     to,
        //     fee: get_fee_by_ledger(ledger),
        //   });
        //   console.log(sendDfx);
        // }
      } catch (err) {
        console.error(err);
      }
    },
  });
};

export default useTransfer;
