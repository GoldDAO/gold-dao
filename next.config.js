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
	images: { unoptimized: true },
};

module.exports = nextConfig



