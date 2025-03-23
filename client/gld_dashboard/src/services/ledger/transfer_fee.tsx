import { ActorSubclass } from "@dfinity/agent";
import { TransferFee } from "./interfaces/ledger_icp";

const transfer_fee = async (actor: ActorSubclass): Promise<bigint> => {
  const result = (await actor.transfer_fee()) as TransferFee;
  return result.transfer_fee.e8s;
};

export default transfer_fee;
