/** @type {import('next').NextConfig} */

const nextConfig = {
  output: 'export',
  images: {
    remotePatterns: [
      {
        protocol: 'https',
        hostname: '*',
      },
    ],
    unoptimized: true,
  },
  env: {
    ENV: process.env.ENV,
  },
};

module.exports = nextConfig;
