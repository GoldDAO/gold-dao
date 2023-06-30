const CanisterIds = require('./../canister_ids.json')
// const fs = require('fs');


let NETWORK

// fs.readFile('./../.env', 'utf8', (err, data) => {
// 	if (err) {
// 		console.error('An error occured when reading .env file:', err);
// 		return;
// 	}


// 	const envVariables = data.split('\n');
// 	envVariables.forEach((envVariable) => {
// 		const [key, value] = envVariable.split('=');
// 		if (key && value) {
// 			const formatedValue = value.trim()
// 			const formatedKey = key.trim()
// 			console.log('NEXT_PUBLIC_DFX_NETWORK' === formatedKey)
// 		};

// 	});

// });

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
		NETWORK
	},
	images: { unoptimized: true },
};

module.exports = nextConfig



