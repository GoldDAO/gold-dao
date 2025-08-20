import { ActorSubclass } from "@dfinity/agent";
import {
  ManageStakePositionArgs,
  Result_4,
  StakePositionResponse,
  ManageStakePositionError,
} from "@services/gldt_stake/interfaces/idlFactory";
import {
  parseWithdrawRequestErrors,
  parseGeneralError,
} from "@services/gldt_stake/utils/parserError";

const parseErrors = (error: ManageStakePositionError): string => {
  if ("WithdrawError" in error)
    return parseWithdrawRequestErrors(error.WithdrawError);
  if ("GeneralError" in error) return parseGeneralError(error.GeneralError);

  return JSON.stringify(error);
};

const withdraw = async (
  actor: ActorSubclass
): Promise<StakePositionResponse> => {
  const args: ManageStakePositionArgs = {
    Withdraw: {},
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

export default withdraw;
