import { MouseEventHandler } from "react";
import { useAtomValue } from "jotai";
import { useAuth as useAuthIK } from "@nfid/identitykit/react";

import authStateAtom from "./atoms";

export const useAuth = () => {
  const { connect: connectIK, disconnect: disconnectIK } = useAuthIK();

  const state = useAtomValue(authStateAtom);

  const connect: MouseEventHandler<HTMLButtonElement> = (e) => {
    e.preventDefault();
    e.stopPropagation();
    connectIK();
  };

  const disconnect = () => {
    disconnectIK();
  };

  return {
    isConnected: state.isConnected,
    isInitializing: state.isInitializing,
    principalId: state.principalId,
    unauthenticatedAgent: state.unauthenticatedAgent,
    authenticatedAgent: state.authenticatedAgent,
    connect,
    disconnect,
  };
};
