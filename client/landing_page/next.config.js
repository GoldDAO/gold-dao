const path = require('path');
const env = require('dotenv').config({
    path: path.resolve(__dirname, './../.env'),
});

const FRONTEND_VERSION = process.env.GIT_COMMIT_REF_NAME || 'local build';

const nextConfig = {
    output: 'export',
    assetPrefix: '',
    compiler: {
        styledComponents: true,
    },
    env: {
        FRONTEND_VERSION,
        DFX_NETWORK: process.env.DFX_NETWORK,
    },
    images: { unoptimized: true },
};

module.exports = nextConfig;
