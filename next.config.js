const nextConfig = {
	transpilePackages: [
		'@connect2ic/react',
		"@connect2ic/core",
	],
	compiler: {
		styledComponents: true,
	},
	images: { unoptimized: true },
	async rewrites() {
		return [
			{
				source: "/:path*",
				destination: "/",
			},
		];
	},
};

module.exports = nextConfig



