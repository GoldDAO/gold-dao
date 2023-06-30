const path = require('path');
const env = require('dotenv').config({
	path: path.resolve(__dirname, './../.env'),
})
const CanisterIds = require('./../canister_ids.json')

const GLDNFT_CANISTER_IDS =
	process.env.NEXT_PUBLIC_DFX_NETWORK === "local" ?
		CanisterIds.gldnft_backend_staging :
		process.env.NEXT_PUBLIC_DFX_NETWORK === "production" ?
			CanisterIds.gldnft_backend_production :
			process.env.NEXT_PUBLIC_DFX_NETWORK === "staging" ?
				CanisterIds.gldnft_backend_staging :
				CanisterIds.gldnft_backend_staging;

const nextConfig = {
	output: 'export',
	transpilePackages: [
		'@connect2ic/react',
		"@connect2ic/core",
	],
	assetPrefix: '',
	compiler: {
		styledComponents: true,
	},
	env: {
		GLDNFT_CANISTER_IDS,
		YUMI_KYC_CANISTER_ID: CanisterIds.yumi_kyc.ic,
		ICP_LEDGER_CANISTER_ID: CanisterIds.icp_ledger.ic,
		NETWORK: env.parsed.NEXT_PUBLIC_DFX_NETWORK
	},
	images: { unoptimized: true },
};

module.exports = nextConfig



