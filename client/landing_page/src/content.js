import BAS from '/public/images/partners/BAS.svg';
import dfinity from '/public/images/partners/dfinity.svg';
import KPMG from '/public/images/partners/KPMG.svg';
import loomis from '/public/images/partners/loomis.png';
import metalor from '/public/images/partners/metalor.svg';
import origyn from '/public/images/partners/origyn.svg';

export const content = {
    intro: {
        title: 'A token backed 100% by physical gold',
        content:
            'GLDT is a fungible token backed by physical gold stored in vaults in Switzerland. Through the Yumi marketplace, user can purchase gold certificates (known as GLD NFTs) and then swap them for GLDT tokens in the swap app. For instance, if you have a certificate for 1 gram of gold, you can swap it for 100 GLDT. So, in essence, each GLDT token represents a portion of real phyiscal gold.',
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
            name: 'BAS',
            url: 'https://nbdc5-raaaa-aaaan-qdxfa-cai.icp0.io/',
            img: BAS,
            w: '110px',
        },
    ],
    tech: {
        title: 'Learn more about the technology',
        content:
            'GLDT and its underlying smart contracts run entirely on the ICP blockchain. In the future, the token will also be cross-platform and multi-chain. To learn more about how GLDT and swapping GLD NFT works, read the FAQ or consult the whitepaper.',
        link: {
            label: 'Read the FAQ',
            href: '/FAQ',
        },
    },
    price: {
        title: 'Determining the price of GLDT',
        subTitle: '1g of Gold = 100 gldt',
        content:
            "The price of GLDT is directly linked to the spot price of physical gold. The market determines the price of gold, which is then used to calculate the value of GLDT. It's important to note that 100 GLDT is equivalent to 1 gram of gold.",
    },
};
