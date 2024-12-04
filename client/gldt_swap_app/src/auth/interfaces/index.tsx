import { HttpAgent, Agent } from "@dfinity/agent";
import { IDL } from "@dfinity/candid";

export interface Canisters {
  [canisterName: string]: {
    canisterId: string;
    idlFactory: IDL.InterfaceFactory;
  };
}

export interface AuthState {
  isConnected: boolean;
  isConnecting: boolean;
  principalId: string;
  unauthenticatedAgent: HttpAgent | Agent | undefined;
  authenticatedAgent: Agent | undefined;
  canisters: Canisters;
}
