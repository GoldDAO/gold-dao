import React, { useEffect } from 'react';
import { Button } from '@mui/material';
import { useAtom } from 'jotai';
import { cartAtom } from '../../states/cart';
import { Principal } from '@dfinity/principal';
import { useCanister, useWallet } from '@connect2ic/react';
import MainButton from '../UI/button/Buttons';




export const SendBatchOffersButton = () => {
    const [wallet] = useWallet()
    const [cart] = useAtom(cartAtom);

    const [actor1g] = useCanister("NFT_1G_CANISTER")
    const [actor10g] = useCanister("NFT_10G_CANISTER")
    const [actor100g] = useCanister("NFT_100G_CANISTER")
    const [actor1000g] = useCanister("NFT_1000G_CANISTER")

    const salePrice = 100 * 10 ** 8;

    const goldNft1gCart = []
    const goldNft10gCart = []
    const goldNft100gCart = []
    const goldNft1000gCart = []

    const YUMI_KYC_CANISTER_ID = process.env.YUMI_KYC_CANISTER_ID
    const ICP_LEDGER_CANISTER_ID = process.env.ICP_LEDGER_CANISTER_ID


    const payload = (e) => {
        return (
            {
                token_id: e.name,
                sales_config: {
                    escrow_receipt: [],
                    broker_id: [],
                    pricing: {
                        ask: [
                            [
                                {
                                    kyc: Principal.fromText(YUMI_KYC_CANISTER_ID),
                                },
                                {
                                    start_price: salePrice,
                                },
                                {
                                    reserve: salePrice,
                                },
                                {
                                    buy_now: salePrice,
                                },
                                { notify: [Principal.fromText(GLDT_CANISTER_ID)] },
                                {
                                    token: {
                                        ic: {
                                            standard: { Ledger: null },
                                            canister: Principal.fromText(ICP_LEDGER_CANISTER_ID),
                                            decimals: 8,
                                            fee: [10000],
                                            symbol: 'ICP',
                                            id: [],
                                        },
                                    },
                                },
                            ],
                        ],

                    },
                },
            }
        )
    }

    cart.map((e) => {
        e.weight === 1 && goldNft1gCart.push(payload(e))
        e.weight === 10 && goldNft10gCart.push(payload(e))
        e.weight === 100 && goldNft100gCart.push(payload(e))
        e.weight === 1000 && goldNft1000gCart.push(payload(e))
    })

    const handleButton = async () => {
        const res1g = await actor1g.market_transfer_batch_nft_origyn(goldNft1gCart);
        const res10g = await actor10g.market_transfer_batch_nft_origyn(goldNft10gCart);
        const res100g = await actor100g.market_transfer_batch_nft_origyn(goldNft100gCart);
        const res1000g = await actor1000g.market_transfer_batch_nft_origyn(goldNft1000gCart);
        console.log('res1g', res1g)
        console.log('res10g', res10g)
        console.log('res100g', res100g)
        console.log('res1000g', res1000g)
    };

    return (
        <MainButton label="Confirm" action={handleButton} />
    );
};

