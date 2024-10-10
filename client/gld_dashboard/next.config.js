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
    VERSION: process.env.VERSION,
  },
  distDir: process.env.NODE_ENV === 'production' ? 'dist' : '.next',
};

module.exports = nextConfig;
