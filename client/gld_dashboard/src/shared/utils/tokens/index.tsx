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
  GOLDAO_LEDGER_CANISTER_ID_IC,
  ICP_LEDGER_CANISTER_ID_IC,
  OGY_LEDGER_CANISTER_ID_IC,
  APP_MODE,
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
  | "OGY-IC"
  | "GLDT"
  | "ICP"
  | "ICP-IC"
  | "GOLDAO"
  | "GOLDAO-IC"
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
  display_name: TokenName;
  label: TokenLabel;
  canister_id: string;
  canister_id_ledger_index: string;
}

export const TOKEN_WHITELIST = [
  "GLDT",
  "GLDNFT",
  "GOLDAO",
  ...(APP_MODE === "staging" ? ["GOLDAO-IC"] : []),
  "ICP",
  ...(APP_MODE === "staging" ? ["ICP-IC"] : []),
  "OGY",
  ...(APP_MODE === "staging" ? ["OGY-IC"] : []),
  "WTN",
  "ckUSDT",
];

export const TOKEN_SWAP_WHITELIST = [
  "GLDT",
  "GLDNFT",
  "GOLDAO",
  "ICP",
  "OGY",
  "WTN",
];

export const TOKEN_GLDT: Token = {
  id: "gldt",
  name: "GLDT",
  display_name: "GLDT",
  label: "Gold Token",
  canister_id: GLDT_LEDGER_CANISTER_ID,
  canister_id_ledger_index: GLDT_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_GOLDAO: Token = {
  id: "goldao",
  name: "GOLDAO",
  display_name: "GOLDAO",
  label: "GOLDAO",
  canister_id: GOLDAO_LEDGER_CANISTER_ID,
  canister_id_ledger_index: GOLDAO_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_GOLDAO_IC: Token = {
  id: "goldao",
  name: "GOLDAO",
  display_name: "GOLDAO-IC",
  label: "GOLDAO",
  canister_id: GOLDAO_LEDGER_CANISTER_ID_IC,
  canister_id_ledger_index: GOLDAO_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_ICP: Token = {
  id: "icp",
  name: "ICP",
  display_name: "ICP",
  label: "Internet Computer",
  canister_id: ICP_LEDGER_CANISTER_ID,
  canister_id_ledger_index: ICP_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_ICP_IC: Token = {
  id: "icp",
  name: "ICP",
  display_name: "ICP-IC",
  label: "Internet Computer",
  canister_id: ICP_LEDGER_CANISTER_ID_IC,
  canister_id_ledger_index: ICP_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_OGY: Token = {
  id: "ogy",
  name: "OGY",
  display_name: "OGY",
  label: "Origyn",
  canister_id: OGY_LEDGER_CANISTER_ID,
  canister_id_ledger_index: OGY_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_OGY_IC: Token = {
  id: "ogy",
  name: "OGY",
  display_name: "OGY-IC",
  label: "Origyn",
  canister_id: OGY_LEDGER_CANISTER_ID_IC,
  canister_id_ledger_index: OGY_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_WTN: Token = {
  id: "wtn",
  name: "WTN",
  display_name: "WTN",
  label: "Waterneuron",
  canister_id: WTN_LEDGER_CANISTER_ID,
  canister_id_ledger_index: WTN_LEDGER_INDEX_CANISTER_ID,
};

export const TOKEN_CKUSDT: Token = {
  id: "ckusdt",
  name: "ckUSDT",
  display_name: "ckUSDT",
  label: "ckUSDT",
  canister_id: CKUSDT_LEDGER_CANISTER_ID,
  canister_id_ledger_index: CKUSDT_LEDGER_INDEX_CANISTER_ID,
};

export const TOKENS: Token[] = [
  TOKEN_GLDT,
  TOKEN_GOLDAO,
  ...(APP_MODE === "staging" ? [TOKEN_GOLDAO_IC] : []),
  TOKEN_ICP,
  ...(APP_MODE === "staging" ? [TOKEN_ICP_IC] : []),
  TOKEN_OGY,
  ...(APP_MODE === "staging" ? [TOKEN_OGY_IC] : []),
  TOKEN_WTN,
  TOKEN_CKUSDT,
];

export const getTokenByDisplayName = (name: TokenName): Token | undefined => {
  return TOKENS.find((token) => token.display_name === name);
};
