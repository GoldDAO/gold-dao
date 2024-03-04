import {
  createActor,
  idlFactory,
} from '../../../backend/external_canisters/gld_nft/api/declarations';
import {
  createActor as createGLDTLedgerActor,
  idlFactory as idlGLDTLedgerFactory,
} from '../../../backend/canisters/gldt_ledger/api/declarations';
import {
  createActor as createGLDTCoreActor,
  idlFactory as idlGLDTCoreFactory,
} from '../../../backend/canisters/gldt_core/api/declarations';
import {
  createActor as createLedgerIndexActors,
  idlFactory as idlLedgerIndexerFactory,
} from '../../../backend/canisters/gldt_ledger_indexer/api/declarations';

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
  idlFactory: idlGLDTLedgerFactory,
};

export const gldtCoreCanister = {
  canisterId: process.env.GLDT_CANISTER_ID,
  createActor: createGLDTCoreActor,
  idlFactory: idlGLDTCoreFactory,
};

export const ledgerIndexerCanister = {
  canisterId: process.env.GLDT_LEDGER_INDEXER,
  createActor: createLedgerIndexActors,
  idlFactory: idlLedgerIndexerFactory,
};
