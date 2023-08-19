const path = require('path');
const env = require('dotenv').config({
  path: path.resolve(__dirname, './../.env'),
});
const InternalCanisterIds = require('./../canister_ids.json');
const GeneratedLocalCanisterIds = require('./../.dfx/local/canister_ids.json');
const ExternalCanisterIds = require('./../dfx.json');

const LOCAL_TESTING = false;

const canisterKey = process.env.DFX_NETWORK || 'staging'; // takes "staging" for develop and staging environment and "ic" for production environment
const GLDNFT_CANISTER_IDS = {
  '1g': ExternalCanisterIds.canisters.gldnft_backend_1g.remote.id[canisterKey],
  '10g': ExternalCanisterIds.canisters.gldnft_backend_10g.remote.id[canisterKey],
  '100g': ExternalCanisterIds.canisters.gldnft_backend_100g.remote.id[canisterKey],
  '1000g': ExternalCanisterIds.canisters.gldnft_backend_1000g.remote.id[canisterKey],
};

let GLDT_CANISTER_ID = InternalCanisterIds.gldt_core[canisterKey];
let GLDT_LEDGER_CANISTER_ID = InternalCanisterIds.gldt_ledger[canisterKey];
// let YUMI_KYC_CANISTER_ID = InternalCanisterIds.yumi_kyc[canisterKey];
let YUMI_KYC_CANISTER_ID = ExternalCanisterIds.canisters.yumi_kyc.remote.id[canisterKey];
if (LOCAL_TESTING) {
  GLDT_CANISTER_ID = GeneratedLocalCanisterIds.gldt_core['local'];
  GLDT_LEDGER_CANISTER_ID = GeneratedLocalCanisterIds.gldt_ledger['local'];
  YUMI_KYC_CANISTER_ID = GeneratedLocalCanisterIds.yumi_kyc['local'];
}

const nextConfig = {
  output: 'export',
  transpilePackages: ['@connect2ic/react', '@connect2ic/core'],
  assetPrefix: '',
  compiler: {
    styledComponents: true,
  },
  env: {
    GLDT_CANISTER_ID,
    GLDT_LEDGER_CANISTER_ID,
    GLDNFT_CANISTER_IDS,
    YUMI_KYC_CANISTER_ID,
    DFX_NETWORK: process.env.DFX_NETWORK,
  },
  images: { unoptimized: true },
};

module.exports = nextConfig;
