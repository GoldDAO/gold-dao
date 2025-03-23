import { Agent, HttpAgent } from "@dfinity/agent";
import { IDL } from "@dfinity/candid";

export interface Canisters {
  [canisterName: string]: {
    canisterId: string;
    idlFactory: IDL.InterfaceFactory;
  };
}

export interface AuthState {
  isConnected: boolean;
  isInitializing: boolean;
  principalId: string;
  unauthenticatedAgent: Agent | HttpAgent | undefined;
  authenticatedAgent: Agent | undefined;
  canisters: Canisters;
}
