import { ActorSubclass } from "@dfinity/agent";
import {
  ManageStakePositionArgs,
  Result_4,
  StakePositionResponse,
  ManageStakePositionError,
  TokenSymbol,
} from "@services/gldt_stake/interfaces/idlFactory";
import {
  parseClaimRewardsErrors,
  parseGeneralError,
} from "@services/gldt_stake/utils/parserError";

const parseErrors = (error: ManageStakePositionError): string => {
  if ("ClaimRewardError" in error)
    return parseClaimRewardsErrors(error.ClaimRewardError);
  if ("GeneralError" in error) return parseGeneralError(error.GeneralError);

  return JSON.stringify(error);
};

const add_stake = async (
  actor: ActorSubclass,
  tokens: string[]
): Promise<StakePositionResponse> => {
  console.log(tokens);
  const args: ManageStakePositionArgs = {
    ClaimRewards: {
      tokens: tokens.map((token) => ({ [token]: null })) as TokenSymbol[],
    },
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
