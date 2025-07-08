import {
  GLDT_LEDGER_CANISTER_ID,
  GLDT_LEDGER_INDEX_CANISTER_ID,
  GOLDAO_LEDGER_CANISTER_ID,
  GOLDAO_LEDGER_INDEX_CANISTER_ID,
  ICP_LEDGER_CANISTER_ID,
  ICP_LEDGER_INDEX_CANISTER_ID,
  OGY_LEDGER_CANISTER_ID,
  OGY_LEDGER_INDEX_CANISTER_ID,
  WTN_LEDGER_CANISTER_ID,
  WTN_LEDGER_INDEX_CANISTER_ID,
  CKUSDT_LEDGER_CANISTER_ID,
  CKUSDT_LEDGER_INDEX_CANISTER_ID,
} from "@constants";

export type TokenID =
  | "ogy"
  | "gldt"
  | "icp"
  | "goldao"
  | "ckusdt"
  | "ckusdc"
  | "wtn";

export type TokenName =
  | "OGY"
  | "GLDT"
  | "ICP"
  | "GOLDAO"
  | "ckUSDT"
  | "ckUSDC"
  | "WTN";

export type TokenLabel =
  | "Origyn"
  | "Gold Token"
  | "Internet Computer"
  | "GOLDAO"
  | "ckUSDT"
  | "ckUSDC"
  | "Waterneuron";

export interface Token {
  id: TokenID;
  name: TokenName;
  label: TokenLabel;
  canister_id: string;
  canister_id_ledger_index: string;
}

export const TOKEN_WHITELIST = [
  "gldt",
  "gldnft",
  "goldao",
  "icp",
  "ogy",
  "wtn",
  "ckusdt",
];

export const TOKEN_SWAP_WHITELIST = [
  "gldt",
  "gldnft",
  "goldao",
  "icp",
  "ogy",
  "wtn",
  "ckusdt",
];

export const TOKEN_GLDT: Token = {
  id: "gldt",
  name: "GLDT",
  label: "Gold Token",
  canister_id: GLDT_LEDGER_CANISTER_ID,
  canister_id_ledger_index: GLDT_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_GOLDAO: Token = {
  id: "goldao",
  name: "GOLDAO",
  label: "GOLDAO",
  canister_id: GOLDAO_LEDGER_CANISTER_ID,
  canister_id_ledger_index: GOLDAO_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_ICP: Token = {
  id: "icp",
  name: "ICP",
  label: "Internet Computer",
  canister_id: ICP_LEDGER_CANISTER_ID,
  canister_id_ledger_index: ICP_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_OGY: Token = {
  id: "ogy",
  name: "OGY",
  label: "Origyn",
  canister_id: OGY_LEDGER_CANISTER_ID,
  canister_id_ledger_index: OGY_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_WTN: Token = {
  id: "wtn",
  name: "WTN",
  label: "Waterneuron",
  canister_id: WTN_LEDGER_CANISTER_ID,
  canister_id_ledger_index: WTN_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_CKUSDT: Token = {
  id: "ckusdt",
  name: "ckUSDT",
  label: "ckUSDT",
  canister_id: CKUSDT_LEDGER_CANISTER_ID,
  canister_id_ledger_index: CKUSDT_LEDGER_INDEX_CANISTER_ID,
};

export const TOKENS: Token[] = [
  TOKEN_GLDT,
  TOKEN_GOLDAO,
  TOKEN_ICP,
  TOKEN_OGY,
  TOKEN_WTN,
  TOKEN_CKUSDT,
];

export const getTokenById = (id: TokenID): Token | undefined => {
  return TOKENS.find((token) => token.id === id);
};
