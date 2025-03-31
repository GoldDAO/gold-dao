import { atom } from "jotai";

import { AuthState } from "./interfaces";

const authStateAtom = atom<AuthState>({
  isConnected: false,
  isInitializing: false,
  principalId: "",
  unauthenticatedAgent: undefined,
  authenticatedAgent: undefined,
  canisters: {},
});

export default authStateAtom;
