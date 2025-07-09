import { atom } from "jotai";
import { TOKEN_GLDT, Token } from "@shared/utils/tokens";

export const TokenSelectedAtom = atom<Token>(TOKEN_GLDT);
