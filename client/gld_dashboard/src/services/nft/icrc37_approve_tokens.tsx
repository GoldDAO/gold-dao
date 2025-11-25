import { ActorSubclass } from "@dfinity/agent";
import {
  Result_9,
  ApproveTokenArg,
  ApproveTokenError,
} from "@services/nft/interfaces";

const parseApproveTokenError = (err: ApproveTokenError): string => {
  if ("GenericError" in err) {
    return `GenericError: ${err.GenericError.message} (code: ${err.GenericError.error_code})`;
  }
  if ("InvalidSpender" in err) return "InvalidSpender";
  if ("NonExistingTokenId" in err) return "NonExistingTokenId";
  if ("Unauthorized" in err) return "Unauthorized";
  if ("CreatedInFuture" in err)
    return `CreatedInFuture: ledger_time=${err.CreatedInFuture.ledger_time}`;
  if ("GenericBatchError" in err) {
    return `GenericBatchError: ${err.GenericBatchError.message} (code: ${err.GenericBatchError.error_code})`;
  }
  if ("TooOld" in err) return "TooOld";
  return "Unknown error";
};

const getFirstApproveTokensError = (result: Result_9): string | null => {
  if ("Err" in result) {
    return parseApproveTokenError(result.Err);
  }
  if ("Ok" in result) {
    for (const item of result.Ok) {
      if (item && item[0] && "Err" in item[0]) {
        return parseApproveTokenError(item[0].Err);
      }
    }
  }
  return null;
};

const icrc37_approve_tokens = async (
  actor: ActorSubclass,
  params: ApproveTokenArg[]
): Promise<Result_9> => {
  const result = (await actor.icrc37_approve_tokens(params)) as Result_9;
  const error = getFirstApproveTokensError(result);
  if (error) {
    throw new Error(error);
  }
  return result;
};

export default icrc37_approve_tokens;
