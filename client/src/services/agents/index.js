import { createActor, idlFactory } from './declarations/GLD NFT';
import { createActor as createGLDTLedgerActor, idlFactory as idlGLDTLedgerFactory } from './declarations/GLDT LEDGER';
import { createActor as createGLDTCoreActor, idlFactory as idlGLDTCoreFactory } from './declarations/GLDT CORE';

const GLDNFT_CANISTER_IDS = process.env.GLDNFT_CANISTER_IDS || {};

export const gldNftCanisters = {};
for (const [weight, canisterId] of Object.entries(GLDNFT_CANISTER_IDS)) {
  if (!canisterId) continue;
  gldNftCanisters[weight] = {
    canisterId,
    createActor,
    idlFactory,
  };
}

export const gldtLedgerCanister = {
  canisterId: process.env.GLDT_LEDGER_CANISTER_ID,
  createActor: createGLDTLedgerActor,
  idlFactory: idlGLDTLedgerFactory
};


export const gldtCoreCanister = {
  canisterId: process.env.GLDT_CANISTER_ID,
  createActor: createGLDTCoreActor,
  idlFactory: idlGLDTCoreFactory
}