const CanisterIds = require('./../canister_ids.json')

const GLDNFT_CANISTER_IDS =
	process.env.NEXT_PUBLIC_DFX_NETWORK === "local" ?
		CanisterIds.gldnft_backend_staging :
		process.env.NEXT_PUBLIC_DFX_NETWORK === "production" ?
			CanisterIds.gldnft_backend_production :
			CanisterIds.gldnft_backend_staging

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
		GLDNFT_CANISTER_IDS
	},
	images: { unoptimized: true },
};

module.exports = nextConfig



