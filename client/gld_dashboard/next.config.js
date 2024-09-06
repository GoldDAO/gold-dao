/** @type {import('next').NextConfig} */

const nextConfig = {
  output: "export",
  images: {
    remotePatterns: [
      {
        protocol: "https",
        hostname: "*",
      },
    ],
    unoptimized: true,
  },
  env: {
    ENV: process.env.ENV,
    VERSION: process.env.VERSION,
  },
  distDir: "dist",
};

module.exports = nextConfig;
