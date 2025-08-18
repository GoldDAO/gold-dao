import {
  GeneralError,
  AddStakePositionErrors,
  StartDissolvingErrors,
  DissolveInstantlyRequestErrors,
  ClaimRewardErrors,
  WithdrawRequestErrors,
} from "@services/gldt_stake/interfaces/idlFactory";

export const parseGeneralError = (err: GeneralError): string => {
  if ("TransactionAddError" in err)
    return `TransactionAddError - ${err.TransactionAddError}`;
  if ("TransferError" in err) return `TransferError - ${err.TransferError}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${err.AlreadyProcessing}`;
  if ("TransactionPreparationError" in err)
    return `TransactionPreparationError - ${err.TransactionPreparationError}`;
  if ("CannotAddReward" in err)
    return `CannotAddReward - ${err.CannotAddReward}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${err.InvalidPrincipal}`;
  if ("NotAuthorized" in err) return `NotAuthorized - ${err.NotAuthorized}`;
  if ("CallError" in err) return `CallError - ${err.CallError}`;
  if ("ModifyStakeError" in err)
    return `ModifyStakeError - ${err.ModifyStakeError}`;
  if ("StakePositionNotFound" in err)
    return `StakePositionNotFound - ${err.StakePositionNotFound}`;
  if ("InvalidPercentage" in err)
    return `InvalidPercentage - ${err.InvalidPercentage}`;

  return JSON.stringify(err);
};

export const parseAddStakePositionError = (
  err: AddStakePositionErrors
): string => {
  if ("TransferError" in err) return `TransferError - ${err.TransferError}`;
  if ("CapacityExceeded" in err)
    return `CapacityExceeded - ${err.CapacityExceeded}`;
  if ("StakePositionAlreadyExists" in err)
    return `StakePositionAlreadyExists - ${err.StakePositionAlreadyExists}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${err.AlreadyProcessing}`;
  if ("InvalidStakeAmount" in err)
    return `InvalidStakeAmount - ${err.InvalidStakeAmount}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${err.InvalidPrincipal}`;
  if ("CallError" in err) return `CallError - ${err.CallError}`;
  if ("MaxAllowedStakePositions" in err)
    return `MaxAllowedStakePositions - ${err.MaxAllowedStakePositions}`;

  return JSON.stringify(err);
};

export const parseStartDissolvingErrors = (
  err: StartDissolvingErrors
): string => {
  if ("DissolvementsLimitReached" in err)
    return `DissolvementsLimitReached - ${err.DissolvementsLimitReached}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${err.AlreadyProcessing}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${err.InvalidPrincipal}`;
  if ("NotFound" in err) return `NotFound - ${err.NotFound}`;
  if ("NotAuthorized" in err) return `NotAuthorized - ${err.NotAuthorized}`;
  if ("InvalidDissolveAmount" in err)
    return `InvalidDissolveAmount - ${err.InvalidDissolveAmount}`;

  return JSON.stringify(err);
};

export const parseDissolveInstantlyError = (
  err: DissolveInstantlyRequestErrors
): string => {
  if ("AlreadyWithdrawnEarly" in err)
    return `AlreadyWithdrawnEarly - ${err.AlreadyWithdrawnEarly}`;
  if ("TransferError" in err) return `TransferError - ${err.TransferError}`;
  if ("AlreadyProcessing" in err)
    return `AlreadyProcessing - ${err.AlreadyProcessing}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${err.InvalidPrincipal}`;
  if ("NotFound" in err) return `NotFound - ${err.NotFound}`;
  if ("WithdrawErrors" in err) return `WithdrawErrors - ${err.WithdrawErrors}`;
  if ("NotAuthorized" in err) return `NotAuthorized - ${err.NotAuthorized}`;
  if ("CallError" in err) return `CallError - ${err.CallError}`;

  return JSON.stringify(err);
};

export const parseWithdrawRequestErrors = (
  err: WithdrawRequestErrors
): string => {
  if ("TransferError" in err) return `TransferError - ${err.TransferError}`;
  if ("AlreadyWithdrawn" in err)
    return `AlreadyWithdrawn - ${err.AlreadyWithdrawn}`;
  if ("InvalidPrincipal" in err)
    return `InvalidPrincipal - ${err.InvalidPrincipal}`;
  if ("NotFound" in err) return `NotFound - ${err.NotFound}`;
  if ("WithdrawErrors" in err) return `WithdrawErrors - ${err.WithdrawErrors}`;
  if ("NotAuthorized" in err) return `NotAuthorized - ${err.NotAuthorized}`;
  if ("CallError" in err) return `CallError - ${err.CallError}`;
  if ("InvalidState" in err) return `InvalidState - ${err.InvalidState}`;

  return JSON.stringify(err);
};

export const parseClaimRewardsErrors = (err: ClaimRewardErrors[]): string => {
  let errorMessage = "";

  err.some((error) => {
    if ("NoTokensProvided" in error) {
      errorMessage = `NoTokensProvided - ${error.NoTokensProvided}`;
      return true;
    }
    if ("TransferError" in error) {
      errorMessage = `TransferError - ${error.TransferError}`;
      return true;
    }
    if ("InvalidRewardToken" in error) {
      errorMessage = `InvalidRewardToken - ${error.InvalidRewardToken}`;
      return true;
    }
    if ("AlreadyProcessing" in error) {
      errorMessage = `AlreadyProcessing - ${error.AlreadyProcessing}`;
      return true;
    }
    if ("InvalidPrincipal" in error) {
      errorMessage = `InvalidPrincipal - ${error.InvalidPrincipal}`;
      return true;
    }
    if ("NotFound" in error) {
      errorMessage = `NotFound - ${error.NotFound}`;
      return true;
    }
    if ("NotAuthorized" in error) {
      errorMessage = `NotAuthorized - ${error.NotAuthorized}`;
      return true;
    }
    if ("CallError" in error) {
      errorMessage = `CallError - ${error.CallError}`;
      return true;
    }
    if ("TokenImbalance" in error) {
      errorMessage = `TokenImbalance - ${error.TokenImbalance}`;
      return true;
    }
    return false;
  });

  return errorMessage || JSON.stringify(err);
};
