import Yumi from '/public/images/partners/yumi.svg';
import dfinity from '/public/images/partners/dfinity.svg';
import KPMG from '/public/images/partners/KPMG.svg';
import loomis from '/public/images/partners/loomis.png';
import metalor from '/public/images/partners/metalor.svg';
import origyn from '/public/images/partners/origyn.svg';

export const content = {
    intro: {
        title: 'A token backed 100% in perpetuity by physical gold',
        content:
            'GLDT is a fractionable token backed by physical gold stored securely in Swiss vaults, also being audited quarterly and insured. Users can buy gold certificates (referred to as GLD NFTs) via the Yumi marketplace. These certificates can then be exchanged for GLDT tokens using the swap app. For example, a GLD NFT representing 1 gram of gold can be swapped for 100 GLDT tokens. Essentially, each GLDT represents a fractional share of actual physical gold, making it far more liquid. This enables the trading of gold outside the traditional banking system.',
    },
    partners: [
        {
            name: 'METALOR',
            url: 'https://metalor.com/',
            img: metalor,
            w: '90px',
        },
        {
            name: 'origyn',
            url: 'https://www.origyn.com/',
            img: origyn,
            w: '100px',
        },
        {
            name: 'KPMG',
            url: 'https://kpmg.com/',
            img: KPMG,
            w: '80px',
        },

        {
            name: 'loomis',
            url: 'https://www.loomis.ch',
            img: loomis,
            w: '65px',
        },
        {
            name: 'icp',
            url: 'https://internetcomputer.org/',
            img: dfinity,
            w: '60px',
        },

        {
            name: 'Yumi',
            url: 'https://tppkg-ziaaa-aaaal-qatrq-cai.raw.ic0.app/',
            img: Yumi,
            w: '105px',
        },
    ],
    tech: {
        title: 'Learn more about the technology',
        content:
            'GLDTs and their underlying smart contracts run entirely on the ICP blockchain. In the future, GLDTs will become cross-platform and multi-chain. This heralds a new era in which physical gold can be transferred using blockchain technology. To learn more about how GLDT and swapping GLD NFTs works, please refer to the FAQ or review the whitepaper.',
        link: {
            label: 'Read the FAQ',
            href: '/FAQ',
        },
    },
    price: {
        title: 'Determining the price of GLDT',
        subTitle: '1g of Gold = 100 gldt',
        content:
            "The price of GLDT is directly correlated with the spot price of physical gold. The market determines the price of gold, which is then used to calculate the value of GLDT. It's important to note that every 100 GLDT equals 1 gram of gold. This system operates 24/7, accessible all around the world with lowest fees.",
    },
};
