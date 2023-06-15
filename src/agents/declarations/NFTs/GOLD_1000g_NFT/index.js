import { NFT_1000_CANISTER_ID } from "../../../../constant.js";
import { Actor } from "@dfinity/agent";
import { idlFactory } from "./Gold_1000_NFT.did.js";
export { idlFactory };

export const canisterId = NFT_1000_CANISTER_ID

export const createActor = (canisterId, agent) => {
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
    });
};


