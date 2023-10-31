import { Actor } from '@dfinity/agent';
import { idlFactory } from './GLDT_CORE.did.js';
export { idlFactory };

export const createActor = (canisterId, agent) => {
    return Actor.createActor(idlFactory, {
        agent,
        canisterId,
    });
};
