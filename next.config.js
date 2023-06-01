// const nextConfig = {
// 	reactStrictMode: true,
// 	output: "export",
// 	transpilePackages: [
// 		'@connect2ic/react',
// 		"@connect2ic/core",
// 		"@connect2ic/core/providers/astrox",
// 		"@astrox", "ic-stoic-identity",
// 		"ic-stoic-identit",
// 		"@connect2ic/core/providers/stoic-wallet"
// 	],
// 	compiler: {
// 		styledComponents: true,
// 	},
// 	images: { unoptimized: true },
// };

// module.exports = nextConfig

const withTM = require('next-transpile-modules')(['@connect2ic/react',
	"@connect2ic/core",]); // pass the modules you would like to see transpiled

module.exports = withTM({
	reactStrictMode: true,
	output: "export",
	compiler: {
		styledComponents: true,
	},
	images: { unoptimized: true },
})
