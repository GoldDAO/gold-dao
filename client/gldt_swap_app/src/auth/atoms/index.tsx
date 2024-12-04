import { atom } from "jotai";

import { AuthState } from "../interfaces";

const initialState: AuthState = {
  isConnected: false,
  isConnecting: false,
  principalId: "",
  unauthenticatedAgent: undefined,
  authenticatedAgent: undefined,
  canisters: {},
};

export const stateAtom = atom<AuthState>(initialState);
