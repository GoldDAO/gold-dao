import BAS from '/public/images/partners/BAS.svg';
import dfinity from '/public/images/partners/dfinity.svg';
import KPMG from '/public/images/partners/KPMG.svg';
import loomis from '/public/images/partners/loomis.jpg';
import metalor from '/public/images/partners/metalor.svg';
import origyn from '/public/images/partners/origyn.svg';

export const content = {
    intro: {
        title: 'A token backed 100% by physical gold',
        content:
            'The GLDT is a token backed by real gold from Metalor. Through the Yumi marketplace, owners can exchange their gold certificates (known as GLD NFTs) for GLDT tokens. For instance, if you have a certificate for 1 gram of gold, you can trade it for 100 GLDT. So, in essence, each GLDT token represents a portion of real gold.',
    },
    partners: [
        {
            name: 'dfinity',
            url: 'https://dfinity.org/',
            img: dfinity,
            w: '60px',
        },
        {
            name: 'origyn',
            url: 'https://www.origyn.com/',
            img: origyn,
            w: '100px',
        },
        {
            name: 'METALOR',
            url: 'https://metalor.com/',
            img: metalor,
            w: '90px',
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
            name: 'BAS',
            url: '#',
            img: BAS,
            w: '110px',
        },
    ],
    tech: {
        title: 'Learn More About the Technology',
        content:
            "The GLDT primarily uses the ICP blockchain for token minting. It's worth noting that the token is cross-platform and multi-chain. To learn more about how minting and the token work, consult the whitepaper.",
        link: {
            label: 'Read the whitepaper',
            href: '/whitepaper',
        },
    },
    price: {
        title: 'Determining the Price of GLDT',
        subTitle: '1g of gold = 100 gldt',
        content:
            "The price of GLDT is directly linked to the spot price of physical gold. The market determines the price of gold, which is then used to calculate the value of GLDT. It's important to note that 100 GLDT is equivalent to 1 gram of gold.",
    },
};
