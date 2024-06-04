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
  eslint: {
    // Warning: This allows production builds to successfully complete even if
    // your project has ESLint errors.
    ignoreDuringBuilds: true,
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
