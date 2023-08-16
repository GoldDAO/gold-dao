const path = require('path');
const env = require('dotenv').config({
  path: path.resolve(__dirname, './../.env'),
});
const InternalCanisterIds = require('./../canister_ids.json');
const ExternalCanisterIds = require('./../dfx.json');

const canisterKey = process.env.DFX_NETWORK || 'staging'; // takes "staging" for develop and staging environment and "ic" for production environment
const GLDNFT_CANISTER_IDS = {
  '1g': ExternalCanisterIds.canisters.gldnft_backend_1g.remote.id[canisterKey],
  '10g': ExternalCanisterIds.canisters.gldnft_backend_10g.remote.id[canisterKey],
  '100g': ExternalCanisterIds.canisters.gldnft_backend_100g.remote.id[canisterKey],
  '1000g': ExternalCanisterIds.canisters.gldnft_backend_1000g.remote.id[canisterKey],
};

const GLDT_CANISTER_ID = InternalCanisterIds.gldt_core[canisterKey];
const GLDT_LEDGER_CANISTER_ID = InternalCanisterIds.gldt_ledger[canisterKey];

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
    YUMI_KYC_CANISTER_ID: 'ucs6g-wiaaa-aaaah-abwpa-cai',
    ICP_LEDGER_CANISTER_ID: 'ryjl3-tyaaa-aaaaa-aaaba-cai',
    DFX_NETWORK: process.env.DFX_NETWORK,
  },
  images: { unoptimized: true },
};

module.exports = nextConfig;
