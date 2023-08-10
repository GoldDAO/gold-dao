import { Actor } from '@dfinity/agent';
import { idlFactory } from '../Gold_NFT.did.js';
export { idlFactory };

export const canisterId = process.env.GLDNFT_CANISTER_IDS['1g'];

export const createActor = (canisterId, agent) => {
  return Actor.createActor(idlFactory, {
    agent,
    canisterId,
  });
};
