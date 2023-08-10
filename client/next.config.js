const path = require('path');
const env = require('dotenv').config({
  path: path.resolve(__dirname, './../.env'),
});
const CanisterIds = require('./../canister_ids.json');

const canisterKey = process.env.DFX_NETWORK === 'production' ? 'ic' : 'staging';
const GLDNFT_CANISTER_IDS = {
  '1g': CanisterIds.gldnft_backend_1g[canisterKey],
  '10g': CanisterIds.gldnft_backend_10g[canisterKey],
  '100g': CanisterIds.gldnft_backend_100g[canisterKey],
  '1000g': CanisterIds.gldnft_backend_1000g[canisterKey],
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
    YUMI_KYC_CANISTER_ID: CanisterIds.yumi_kyc.ic,
    ICP_LEDGER_CANISTER_ID: CanisterIds.icp_ledger.ic,
    NETWORK: process.env.DFX_NETWORK ? process.env.DFX_NETWORK : 'staging',
  },
  images: { unoptimized: true },
};

module.exports = nextConfig;
