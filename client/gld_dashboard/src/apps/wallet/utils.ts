import { Ledger } from "@services/ledger/utils/interfaces";

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
} from "@constants";

export interface Token {
  id: Ledger;
  name: string;
  label: string;
  canisterId: string;
  canister_id_ledger_index: string;
}

export const TokensWhitelist = ["gldt", "nft", "goldao", "icp", "ogy", "wtn"];

export const GLDT_INDEX = 0;

export const TokensList: Token[] = [
  {
    id: "gldt",
    name: "GLDT",
    label: "Gold Token",
    canisterId: GLDT_LEDGER_CANISTER_ID,
    canister_id_ledger_index: GLDT_LEDGER_INDEX_CANISTER_ID,
  },
  {
    id: "goldao",
    name: "GOLDAO",
    label: "GOLDAO",
    canisterId: GOLDAO_LEDGER_CANISTER_ID,
    canister_id_ledger_index: GOLDAO_LEDGER_INDEX_CANISTER_ID,
  },
  {
    id: "icp",
    name: "ICP",
    label: "Internet Computer",
    canisterId: ICP_LEDGER_CANISTER_ID,
    canister_id_ledger_index: ICP_LEDGER_INDEX_CANISTER_ID,
  },
  {
    id: "ogy",
    name: "OGY",
    label: "Origyn",
    canisterId: OGY_LEDGER_CANISTER_ID,
    canister_id_ledger_index: OGY_LEDGER_INDEX_CANISTER_ID,
  },
  {
    id: "wtn",
    name: "WTN",
    label: "Waterneuron",
    canisterId: WTN_LEDGER_CANISTER_ID,
    canister_id_ledger_index: WTN_LEDGER_INDEX_CANISTER_ID,
  },
];
