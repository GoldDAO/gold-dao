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
    MAINTENANCE_MODE: process.env.MAINTENANCE_MODE,
  },
  publicRuntimeConfig: {
    NEXT_PUBLIC_MAINTENANCE_MODE: process.env.MAINTENANCE_MODE,
  },
};

module.exports = nextConfig;
