import { useAtom } from "jotai";
import { useIdentityKit } from "@nfid/identitykit/react";
import { Actor, ActorSubclass, Agent } from "@dfinity/agent";

import { stateAtom } from "../atoms";

export const useAuth = () => {
  const { connect, disconnect: disconnectIK } = useIdentityKit();

  const [state, setState] = useAtom(stateAtom);

  const disconnect = () => {
    setState((prevState) => ({
      ...prevState,
      principalId: "",
      isConnected: false,
      isConnecting: false,
      agent: undefined,
    }));
    disconnectIK();
  };

  const createActor = (
    canister:
      | string
      | "gld_nft_1g"
      | "gld_nft_10g"
      | "gld_nft_100g"
      | "gld_nft_1000g"
      | "gldt_swap"
      | "gldt_ledger"
      | "ogy_ledger"
      | "icp_swap"
  ): ActorSubclass => {
    const { canisterId, idlFactory } = state.canisters[canister];

    const actor = Actor.createActor(idlFactory, {
      agent: state.agent as Agent,
      canisterId,
    });

    return actor as ActorSubclass;
  };

  return {
    isConnected: state.isConnected,
    isConnecting: state.isConnecting,
    principalId: state.principalId,
    connect,
    disconnect,
    createActor,
  };
};
