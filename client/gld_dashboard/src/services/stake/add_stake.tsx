import { ActorSubclass } from "@dfinity/agent";
import {
  ManageStakePositionArgs,
  Result_3,
  StakePositionResponse,
  ManageStakePositionError,
} from "@services/stake/idlFactory";
import {
  parseAddStakePositionError,
  parseGeneralError,
} from "@services/stake/utils/parserError";

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

  const result = (await actor.manage_stake_position(args)) as Result_3;

  if ("Ok" in result) {
    return result.Ok;
  } else {
    console.error(result.Err);
    const errorMessage = parseErrors(result.Err);
    throw new Error(errorMessage);
  }
};

export default add_stake;
