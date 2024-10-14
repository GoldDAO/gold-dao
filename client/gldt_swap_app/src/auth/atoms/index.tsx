import { atom } from "jotai";
import { HttpAgent } from "@dfinity/agent";

import { AuthState } from "../interfaces";

const initialState: AuthState = {
  isConnected: false,
  isConnecting: false,
  principalId: "",
  agent: undefined,
  canisters: {},
};

export const stateAtom = atom<AuthState>(initialState);
export const agentAtom = atom<HttpAgent | undefined>(undefined);
