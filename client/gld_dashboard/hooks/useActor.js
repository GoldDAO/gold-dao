"use client";

import { Actor, AnonymousIdentity, HttpAgent } from "@dfinity/agent";

import { canisters } from "../utils/canisters";
import { useSession } from "./useSession";

const useActor = (canisterName, anonymous = false) => {
  const { identity } = useSession();

  const canister = canisters[canisterName];
  if (!canister) return;

  const agent = new HttpAgent({
    identity: anonymous ? new AnonymousIdentity() : identity,
    host: "https://identity.ic0.app",
  });

  const actor = Actor.createActor(canister.idlFactory, {
    agent,
    canisterId: canister.canisterId,
  });

  return [actor];
};

export default useActor;
