import {
  GeneralError,
  AddStakePositionErrors,
  StartDissolvingErrors,
  DissolveInstantlyRequestErrors,
  ClaimRewardErrors,
  WithdrawRequestErrors,
  WithdrawErrors,
} from "@services/gldt_stake/interfaces/idlFactory";

const extractDeepMessage = (value: string | object): string => {
  if (typeof value === "string") {
    return value;
  }

  if (typeof value === "object" && value !== null) {
    const keys = Object.keys(value);
    if (keys.length === 1) {
      return extractDeepMessage(
        (value as Record<string, string | object>)[keys[0]]
      );
    }

    for (const key of keys) {
      const result = extractDeepMessage(
        (value as Record<string, string | object>)[key]
      );
      if (typeof result === "string") {
        return result;
      }
    }
    return JSON.stringify(value);
  }
  return String(value);
};

const parseWithdrawErrors = (err: WithdrawErrors): string => {
  if ("NoValidDissolveEvents" in err)
    return `NoValidDissolveEvents - ${extractDeepMessage(
      err.NoValidDissolveEvents
    )}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${extractDeepMessage(err.AlreadyProcessing)}`;
  if ("InvalidDissolveInstantlyAmount" in err)
    return `InvalidDissolveInstantlyAmount - ${extractDeepMessage(
      err.InvalidDissolveInstantlyAmount
    )}`;
  if ("InvalidWithdrawAmount" in err)
    return `InvalidWithdrawAmount - ${extractDeepMessage(
      err.InvalidWithdrawAmount
    )}`;
  if ("InvalidDissolveState" in err)
    return `InvalidDissolveState - ${extractDeepMessage(
      err.InvalidDissolveState
    )}`;
  if ("CantWithdrawWithRewardsBalance" in err)
    return `CantWithdrawWithRewardsBalance - ${extractDeepMessage(
      err.CantWithdrawWithRewardsBalance
    )}`;

  return extractDeepMessage(err);
};

export const parseGeneralError = (err: GeneralError): string => {
  if ("TransactionAddError" in err)
    return `TransactionAddError - ${extractDeepMessage(
      err.TransactionAddError
    )}`;
  if ("TransferError" in err)
    return `TransferError - ${extractDeepMessage(err.TransferError)}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${extractDeepMessage(err.AlreadyProcessing)}`;
  if ("TransactionPreparationError" in err)
    return `TransactionPreparationError - ${extractDeepMessage(
      err.TransactionPreparationError
    )}`;
  if ("CannotAddReward" in err)
    return `CannotAddReward - ${extractDeepMessage(err.CannotAddReward)}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${extractDeepMessage(err.InvalidPrincipal)}`;
  if ("BalanceIsLowerThanFee" in err)
    return `BalanceIsLowerThanFee - ${extractDeepMessage(
      err.BalanceIsLowerThanFee
    )}`;
  if ("NotAuthorized" in err)
    return `NotAuthorized - ${extractDeepMessage(err.NotAuthorized)}`;
  if ("BalanceIsLowerThanThreshold" in err)
    return `BalanceIsLowerThanThreshold - ${extractDeepMessage(
      err.BalanceIsLowerThanThreshold
    )}`;
  if ("CallError" in err)
    return `CallError - ${extractDeepMessage(err.CallError)}`;
  if ("ModifyStakeError" in err)
    return `ModifyStakeError - ${extractDeepMessage(err.ModifyStakeError)}`;
  if ("StakePositionNotFound" in err)
    return `StakePositionNotFound - ${extractDeepMessage(
      err.StakePositionNotFound
    )}`;
  if ("BalanceIsZero" in err)
    return `BalanceIsZero - ${extractDeepMessage(err.BalanceIsZero)}`;
  if ("InvalidPercentage" in err)
    return `InvalidPercentage - ${extractDeepMessage(err.InvalidPercentage)}`;

  return extractDeepMessage(err);
};

export const parseAddStakePositionError = (
  err: AddStakePositionErrors
): string => {
  if ("TransferError" in err)
    return `TransferError - ${extractDeepMessage(err.TransferError)}`;
  if ("CapacityExceeded" in err)
    return `CapacityExceeded - ${extractDeepMessage(err.CapacityExceeded)}`;
  if ("StakePositionAlreadyExists" in err)
    return `StakePositionAlreadyExists - ${extractDeepMessage(
      err.StakePositionAlreadyExists
    )}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${extractDeepMessage(err.AlreadyProcessing)}`;
  if ("InvalidStakeAmount" in err)
    return `InvalidStakeAmount - ${extractDeepMessage(err.InvalidStakeAmount)}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${extractDeepMessage(err.InvalidPrincipal)}`;
  if ("CallError" in err)
    return `CallError - ${extractDeepMessage(err.CallError)}`;
  if ("MaxAllowedStakePositions" in err)
    return `MaxAllowedStakePositions - ${extractDeepMessage(
      err.MaxAllowedStakePositions
    )}`;

  return extractDeepMessage(err);
};

export const parseStartDissolvingErrors = (
  err: StartDissolvingErrors
): string => {
  if ("DissolvementsLimitReached" in err)
    return `DissolvementsLimitReached - ${extractDeepMessage(
      err.DissolvementsLimitReached
    )}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${extractDeepMessage(err.AlreadyProcessing)}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${extractDeepMessage(err.InvalidPrincipal)}`;
  if ("NotFound" in err)
    return `NotFound - ${extractDeepMessage(err.NotFound)}`;
  if ("NotAuthorized" in err)
    return `NotAuthorized - ${extractDeepMessage(err.NotAuthorized)}`;
  if ("InvalidDissolveAmount" in err)
    return `InvalidDissolveAmount - ${extractDeepMessage(
      err.InvalidDissolveAmount
    )}`;

  return extractDeepMessage(err);
};

export const parseDissolveInstantlyError = (
  err: DissolveInstantlyRequestErrors
): string => {
  if ("AlreadyWithdrawnEarly" in err)
    return `AlreadyWithdrawnEarly - ${extractDeepMessage(
      err.AlreadyWithdrawnEarly
    )}`;
  if ("TransferError" in err)
    return `TransferError - ${extractDeepMessage(err.TransferError)}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${extractDeepMessage(err.AlreadyProcessing)}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${extractDeepMessage(err.InvalidPrincipal)}`;
  if ("NotFound" in err)
    return `NotFound - ${extractDeepMessage(err.NotFound)}`;
  if ("WithdrawErrors" in err)
    return `WithdrawErrors - ${parseWithdrawErrors(err.WithdrawErrors)}`;
  if ("NotAuthorized" in err)
    return `NotAuthorized - ${extractDeepMessage(err.NotAuthorized)}`;
  if ("CallError" in err)
    return `CallError - ${extractDeepMessage(err.CallError)}`;

  return extractDeepMessage(err);
};

export const parseWithdrawRequestErrors = (
  err: WithdrawRequestErrors
): string => {
  if ("TransferError" in err)
    return `TransferError - ${extractDeepMessage(err.TransferError)}`;
  if ("AlreadyWithdrawn" in err)
    return `AlreadyWithdrawn - ${extractDeepMessage(err.AlreadyWithdrawn)}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${extractDeepMessage(err.InvalidPrincipal)}`;
  if ("NotFound" in err)
    return `NotFound - ${extractDeepMessage(err.NotFound)}`;
  if ("WithdrawErrors" in err)
    return `WithdrawErrors - ${parseWithdrawErrors(err.WithdrawErrors)}`;
  if ("NotAuthorized" in err)
    return `NotAuthorized - ${extractDeepMessage(err.NotAuthorized)}`;
  if ("CallError" in err)
    return `CallError - ${extractDeepMessage(err.CallError)}`;
  if ("InvalidState" in err)
    return `InvalidState - ${extractDeepMessage(err.InvalidState)}`;

  return extractDeepMessage(err);
};

export const parseClaimRewardsErrors = (
  errors: ClaimRewardErrors[]
): string => {
  const error = errors[0];
  if ("NoTokensProvided" in error)
    return `NoTokensProvided - ${extractDeepMessage(error.NoTokensProvided)}`;
  if ("TransferError" in error)
    return `TransferError - ${extractDeepMessage(error.TransferError)}`;
  if ("InvalidRewardToken" in error)
    return `InvalidRewardToken - ${extractDeepMessage(
      error.InvalidRewardToken
    )}`;
  if ("AlreadyProcessing" in error)
    return `AlreadyProcessing - ${extractDeepMessage(error.AlreadyProcessing)}`;
  if ("InvalidPrincipal" in error)
    return `InvalidPrincipal - ${extractDeepMessage(error.InvalidPrincipal)}`;
  if ("NotFound" in error)
    return `NotFound - ${extractDeepMessage(error.NotFound)}`;
  if ("NotAuthorized" in error)
    return `NotAuthorized - ${extractDeepMessage(error.NotAuthorized)}`;
  if ("CallError" in error)
    return `CallError - ${extractDeepMessage(error.CallError)}`;
  if ("TokenImbalance" in error)
    return `TokenImbalance - ${extractDeepMessage(error.TokenImbalance)}`;

  return extractDeepMessage(error);
};
