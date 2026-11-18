import { Principal } from "@dfinity/principal";
import type { Token } from "./index";

// Tokens the user added to the wallet by ledger canister ID. They are kept in
// the browser (localStorage), so everything read back from there is checked
// again before use.

export const CUSTOM_TOKENS_STORAGE_KEY = "gld_dashboard.custom_tokens";

export const MAX_SYMBOL_LENGTH = 20;
export const MAX_NAME_LENGTH = 64;

export interface StoredCustomToken {
  ledger_id: string;
  index_id: string | null;
  symbol: string;
  name: string;
}

export type CustomTokenConflict = "ledger" | "symbol";

const isCanisterId = (value: unknown): value is string => {
  if (typeof value !== "string") return false;
  try {
    return Principal.fromText(value).toText() === value;
  } catch {
    return false;
  }
};

const sameSymbol = (a: string, b: string) =>
  a.trim().toLowerCase() === b.trim().toLowerCase();

// "GLDNFT" is the URL value of the GLD NFT row.
const RESERVED_SYMBOLS = ["GLDNFT"];

// A token conflicts when its ledger is already in the wallet, or when its
// symbol is taken. Symbols must be unique because the wallet selects a token
// by symbol, and a built-in symbol on another ledger is an imitation.
export const findCustomTokenConflict = (
  candidate: { ledger_id: string; symbol: string },
  tokens: Token[]
): CustomTokenConflict | null => {
  if (tokens.some((token) => token.canister_id === candidate.ledger_id)) {
    return "ledger";
  }
  const taken = [
    ...RESERVED_SYMBOLS,
    ...tokens.flatMap((token) => [token.name, token.display_name]),
  ];
  if (taken.some((symbol) => sameSymbol(symbol, candidate.symbol))) {
    return "symbol";
  }
  return null;
};

export const toCustomToken = (stored: StoredCustomToken): Token => ({
  id: "custom",
  name: stored.symbol,
  display_name: stored.symbol,
  label: stored.name,
  canister_id: stored.ledger_id,
  canister_id_ledger_index: stored.index_id ?? "",
  is_custom: true,
});

// Decimals to show an amount with. The built-in tokens keep their 2 decimals.
// A custom token can be worth a lot per unit (ckBTC), so smaller amounts get
// more decimals.
export const amountDisplayDecimals = (token: Token, amount: number): number => {
  if (!token.is_custom) return 2;
  const size = Math.abs(amount);
  if (size >= 1000) return 2;
  if (size >= 1) return 4;
  return 8;
};

const parseEntry = (entry: unknown): StoredCustomToken | null => {
  if (typeof entry !== "object" || entry === null) return null;
  const { ledger_id, index_id, symbol, name } = entry as Record<
    string,
    unknown
  >;
  if (!isCanisterId(ledger_id)) return null;
  if (typeof symbol !== "string" || typeof name !== "string") return null;
  const trimmedSymbol = symbol.trim();
  if (trimmedSymbol === "" || trimmedSymbol.length > MAX_SYMBOL_LENGTH) {
    return null;
  }
  return {
    ledger_id,
    index_id: isCanisterId(index_id) ? index_id : null,
    symbol: trimmedSymbol,
    name: name.trim().slice(0, MAX_NAME_LENGTH) || trimmedSymbol,
  };
};

// Reads the stored list. Entries that are malformed, or that conflict with a
// built-in token or with an earlier entry, are dropped.
export const parseStoredCustomTokens = (
  value: unknown,
  builtinTokens: Token[]
): StoredCustomToken[] => {
  if (!Array.isArray(value)) return [];
  const accepted: StoredCustomToken[] = [];
  for (const entry of value) {
    const parsed = parseEntry(entry);
    if (!parsed) continue;
    const tokens = [...builtinTokens, ...accepted.map(toCustomToken)];
    if (findCustomTokenConflict(parsed, tokens)) continue;
    accepted.push(parsed);
  }
  return accepted;
};
