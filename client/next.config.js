const path = require('path');
const env = require('dotenv').config({
  path: path.resolve(__dirname, './../.env'),
});
const ExternalCanisterIds = require('./../dfx.json');

const canisterKey = process.env.DFX_NETWORK || 'staging'; // takes "staging" for develop and staging environment and "ic" for production environment
const GLDNFT_CANISTER_IDS = {
  '1g': ExternalCanisterIds.canisters.gldnft_backend_1g.remote.id[canisterKey],
  '10g': ExternalCanisterIds.canisters.gldnft_backend_10g.remote.id[canisterKey],
  '100g': ExternalCanisterIds.canisters.gldnft_backend_100g.remote.id[canisterKey],
  '1000g': ExternalCanisterIds.canisters.gldnft_backend_1000g.remote.id[canisterKey],
};

const nextConfig = {
  output: 'export',
  transpilePackages: ['@connect2ic/react', '@connect2ic/core'],
  assetPrefix: '',
  compiler: {
    styledComponents: true,
  },
  env: {
    GLDNFT_CANISTER_IDS,
    YUMI_KYC_CANISTER_ID: ExternalCanisterIds.canisters.yumi_kyc.remote.id.ic,
    ICP_LEDGER_CANISTER_ID: ExternalCanisterIds.canisters.icp_ledger.remote.id.ic,
    DFX_NETWORK: process.env.DFX_NETWORK,
  },
  images: { unoptimized: true },
};

module.exports = nextConfig;
