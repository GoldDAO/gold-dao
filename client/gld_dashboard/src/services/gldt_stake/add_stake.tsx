import { ActorSubclass } from "@dfinity/agent";
import {
  ManageStakePositionArgs,
  Result_4,
  StakePositionResponse,
  ManageStakePositionError,
} from "@services/gldt_stake/interfaces/idlFactory";
import {
  parseAddStakePositionError,
  parseGeneralError,
} from "@services/gldt_stake/utils/parserError";

const parseErrors = (error: ManageStakePositionError): string => {
  if ("AddStakeError" in error)
    return parseAddStakePositionError(error.AddStakeError);
  if ("GeneralError" in error) return parseGeneralError(error.GeneralError);

  return JSON.stringify(error);
};

const add_stake = async (
  actor: ActorSubclass,
  amount: bigint
): Promise<StakePositionResponse> => {
  const args: ManageStakePositionArgs = {
    AddStake: { amount },
  };

  const result = (await actor.manage_stake_position(args)) as Result_4;

  if ("Ok" in result) {
    return result.Ok;
  } else {
    console.error(result.Err);
    const errorMessage = parseErrors(result.Err);
    throw new Error(errorMessage);
  }
};

export default add_stake;
