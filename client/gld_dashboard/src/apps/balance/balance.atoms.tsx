import { atom } from "jotai";

import { TokensList, Token } from "./balance.utils";

export const TokenSelectedAtom = atom<Token>(TokensList[0]); // default to GLDT
