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
};

export default nextConfig;
