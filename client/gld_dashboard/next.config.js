// const env = process.env.ENV || "local";
/** @type {import('next').NextConfig} */

const env = process.env.ENV;
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
  eslint: {
    // Warning: This allows production builds to successfully complete even if
    // your project has ESLint errors.
    ignoreDuringBuilds: true,
  },
  env: {
    ENV: env,
  },
};

module.exports = nextConfig;
