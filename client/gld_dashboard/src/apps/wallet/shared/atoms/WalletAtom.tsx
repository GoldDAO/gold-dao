import { atom } from "jotai";
import { TOKEN_GOLDAO, Token } from "@shared/utils/tokens";

export const TokenSelectedAtom = atom<Token>(TOKEN_GOLDAO);
