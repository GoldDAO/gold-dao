const path = require('path');
const env = require('dotenv').config({
    path: path.resolve(__dirname, './../../.env'),
});
const InternalCanisterIds = require('./../../canister_ids.json');
const GeneratedLocalCanisterIds = process.env.CI
    ? {
          gldt_ledger: {
              local: 'bd3sg-teaaa-aaaaa-qaaba-cai',
          },
      }
    : require('./../../.dfx/local/canister_ids.json');
const ExternalCanisterIds = require('./../../dfx.json');

// The $NETWORK env variable is defined by the CI/CD job

const canisterKey = process.env.NETWORK || 'local';
console.debug(
    `This frontend will be built using canister ids coming from the ${canisterKey} network`,
);

const GLDNFT_CANISTER_IDS = process.env.NETWORK
    ? {
          '1g': ExternalCanisterIds.canisters.gldnft_backend_1g.remote.id[canisterKey],
          '10g': ExternalCanisterIds.canisters.gldnft_backend_10g.remote.id[canisterKey],
          '100g': ExternalCanisterIds.canisters.gldnft_backend_100g.remote.id[canisterKey],
          '1000g': ExternalCanisterIds.canisters.gldnft_backend_1000g.remote.id[canisterKey],
      }
    : {
          /* Uncomment when you'll have locally deployed NFT's !
    '1g': GeneratedLocalCanisterIds.gldnft_backend_1g[canisterKey],
    '10g': GeneratedLocalCanisterIds.gldnft_backend_10g[canisterKey],
    '100g': GeneratedLocalCanisterIds.gldnft_backend_100g[canisterKey],
    '1000g': GeneratedLocalCanisterIds.gldnft_backend_1000g[canisterKey],
*/
      };

const FRONTEND_VERSION = process.env.GIT_COMMIT_REF_NAME || 'local build';

const GLDT_CANISTER_ID = process.env.NETWORK ? InternalCanisterIds.gldt_core[canisterKey] : ''; // GeneratedLocalCanisterIds.gldt_core[canisterKey];

const GLDT_LEDGER_CANISTER_ID = process.env.NETWORK
    ? InternalCanisterIds.gldt_ledger[canisterKey]
    : GeneratedLocalCanisterIds.gldt_ledger[canisterKey];

const YUMI_KYC_CANISTER_ID = process.env.NETWORK
    ? ExternalCanisterIds.canisters.yumi_kyc.remote.id[canisterKey]
    : ''; //GeneratedLocalCanisterIds.yumi_kyc[canisterKey];

const nextConfig = {
    output: 'export',
    transpilePackages: ['@connect2ic/react', '@connect2ic/core'],
    assetPrefix: '',
    compiler: {
        styledComponents: true,
    },
    env: {
        GLDT_CANISTER_ID,
        FRONTEND_VERSION,
        GLDT_LEDGER_CANISTER_ID,
        GLDNFT_CANISTER_IDS,
        YUMI_KYC_CANISTER_ID,
        DFX_NETWORK: canisterKey,
    },
    images: { unoptimized: true },
};

module.exports = nextConfig;
