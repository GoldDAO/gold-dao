import { Agent, HttpAgent } from "@dfinity/agent";
import {
  fetch_ledger_token_info,
  is_index_of_ledger,
} from "@services/ledger/fetch_ledger_token_info";
import { Token } from "@shared/utils/tokens";
import {
  MAX_NAME_LENGTH,
  MAX_SYMBOL_LENGTH,
  StoredCustomToken,
  findCustomTokenConflict,
} from "@shared/utils/tokens/custom";

export interface CustomTokenLookup {
  token: StoredCustomToken;
  decimals: number;
  logo: string | null;
}

// Reads a token from its ledger and checks that the wallet can take it. The
// messages of the errors thrown here are shown to the user.
const lookupCustomToken = async ({
  agent,
  ledgerId,
  indexId,
  walletTokens,
}: {
  agent: Agent | HttpAgent;
  ledgerId: string;
  // Index canister entered by the user, or "" to use the one of the ledger.
  indexId: string;
  walletTokens: Token[];
}): Promise<CustomTokenLookup> => {
  if (walletTokens.some((token) => token.canister_id === ledgerId)) {
    throw new Error("This token is already in your wallet.");
  }

  const info = await fetch_ledger_token_info(agent, ledgerId).catch((err) => {
    console.error(err);
    throw new Error(
      "No ICRC-1 ledger answered at this canister ID. Check the ID and try again."
    );
  });

  const symbol = info.symbol.trim();
  if (symbol === "" || symbol.length > MAX_SYMBOL_LENGTH) {
    throw new Error("This ledger reports a symbol the wallet cannot show.");
  }
  if (findCustomTokenConflict({ ledger_id: ledgerId, symbol }, walletTokens)) {
    throw new Error(
      `Your wallet already has a token with the symbol ${symbol}, on another ledger. Tokens with the same symbol cannot be told apart, so this one cannot be added.`
    );
  }

  let index_id: string | null = null;
  if (indexId !== "") {
    if (!(await is_index_of_ledger(agent, indexId, ledgerId))) {
      throw new Error("The index canister ID does not belong to this ledger.");
    }
    index_id = indexId;
  } else if (
    info.index_id &&
    (await is_index_of_ledger(agent, info.index_id, ledgerId))
  ) {
    index_id = info.index_id;
  }

  return {
    token: {
      ledger_id: ledgerId,
      index_id,
      symbol,
      name: info.name.trim().slice(0, MAX_NAME_LENGTH) || symbol,
    },
    decimals: info.decimals,
    logo: info.logo,
  };
};

export default lookupCustomToken;
