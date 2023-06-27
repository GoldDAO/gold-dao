import React, { useEffect } from 'react';
import { Button } from '@mui/material';
import { useAtom } from 'jotai';
import { cartAtom } from '../../states/cart';
import { NftGAgentAtom } from '../../states/agents/GLDNFT';
import { Principal } from '@dfinity/principal';
import { plugAgentAtom } from '../../states/plugAgent';
import { NFT_1_CANISTER_ID } from '../../../src/constant';
import { useCanister, useWallet } from '@connect2ic/react';

const ICP_LEDGER_CANISTER_ID = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
const YUMI_KYC_CANISTER_ID = 'ucs6g-wiaaa-aaaah-abwpa-cai';
const GLDT_CANISTER_ID = 'dgeb6-wyaaa-aaaap-abeha-cai'; // this is wrong, to be replaced

const BatchOffers = () => {
    const [wallet] = useWallet()
    const [cart] = useAtom(cartAtom);
    const [agent] = useAtom(NftGAgentAtom);
    const [plugActor, setPlugActor] = useAtom(plugAgentAtom)
    const salePrice = 100 * 10 ** 8;
    const brokerId = 'gnfh7-n3zkc-7ihoh-vp4my-pu5zn-ssgxf-issex-akzuf-dvub3-dqu5x-zae';

    const [actor1g] = useCanister("NFT_1G_CANISTER")
    const [actor10g] = useCanister("NFT_10G_CANISTER")
    const [actor100g] = useCanister("NFT_100G_CANISTER")
    const [actor1000g] = useCanister("NFT_1000G_CANISTER")


    const goldNft1gCart = []
    const goldNft10gCart = []
    const goldNft100gCart = []
    const goldNft1000gCart = []

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


    useEffect(() => {
        console.log('goldNft1gCart', goldNft1gCart)
        console.log('goldNft10gCart', goldNft10gCart)
        console.log('goldNft100gCart', goldNft100gCart)
        console.log('goldNft1000gCart', goldNft1000gCart)
    }, [goldNft1gCart,
        goldNft10gCart,
        goldNft100gCart,
        goldNft1000gCart])

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
        <Button onClick={() => handleButton()}>send batch offer</Button>
    );
};

export default BatchOffers;
