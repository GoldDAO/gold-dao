import Link from 'next/link';
export const faq = [
    {
        q: 'What is a GLDT?',
        r: 'GLDT stands for “Gold Token”, being a fungible token backed by GLD NFTs. GLD NFTs are NFTs that represent irrefutable ownership of physical gold bars. 1 GLDT represents exactly 0.01g of gold, and GLD NFTs can be swapped at a ratio of 1g for 100 GLDTs.',
    },
    {
        q: 'How are GLDT minted?',
        r: 'GLDT are minted when someone swaps a GLD NFT for GLDT. The swapping ratio is 1g of GLD NFT for 100 GLDT.',
    },
    {
        q: 'Who can participate in the swap?',
        r: (
            <p>
                Everybody who owns GLD NFT. See&nbsp;
                {
                    <Link
                        style={{
                            textDecoration: 'underline',
                        }}
                        href="https://yumi.io/gold/about"
                    >
                        here
                    </Link>
                }
                &nbsp;for details of purchasing GLD NFTs.
            </p>
        ),
    },
    {
        q: 'How does the swap work?',
        r: 'You must own at least one GLD NFT and connect to the swap app. Then you will be able to select the GLD NFT which you want to swap to GLDT. After validating the transaction, the GLD NFT will be swapped against GLDT that you will see in your balance/wallet.',
    },
    {
        q: 'How is the GLDT price determined?',
        r: '1g of gold = 100 GLDT. Or 1 GLDT = 0.01g of gold. E.g.: You have 5g of GLD NFTs. You can swap these at a ratio of 1g to 100GLDT. So when swapping 5g of GLD NFT, you will receive 500 GLDT.',
    },
    {
        q: 'How and when will I know if my swap is final?',
        r: 'Once the transaction is completed, you can find GLDT in your wallet balance.',
    },
    {
        q: 'What are the commissions for swapping GLDT?',
        r: 'Commissions are paid to Originator- BAS (0.4%), Authenticator- BAS (0.1%), ORIGYN (0.5%) and are a total of 1%. These fees are integrated inside the GLD NFT. The swap fees are refunded for the first 100 million GLDT. In addition to the swap fees, there is an inherent transaction fee of 0.0001 GLDT to avoid spamming of the network.',
    },
    {
        q: 'What are the commissions on trading?',
        r: 'There are no fees on trading. However, some exchanges can apply their own exchange fees.',
    },
    {
        q: 'How and when can I swap back my GLDT? Can I sell my GLDT on another platform?',
        r: 'You can swap your GLDT back to GLD NFT on the platform soon. This is currently under development. You may also visit any exchange that supports GLDT and trade it for other cryptocurrencies.',
    },
    {
        q: 'Where can I find more information about GLD NFTs?',
        r: 'Go to yumi.io/gold to learn more about GLD NFTs, which back GLDTs.',
    },
];
