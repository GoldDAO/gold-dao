import { useCanister } from '@connect2ic/react';
import React from 'react';

const NFTsSaleInfos = () => {
    const [actor1g] = useCanister("NFT_1G_CANISTER")
    const [actor10g] = useCanister("NFT_10G_CANISTER")
    const [actor100g] = useCanister("NFT_100G_CANISTER")
    const [actor1000g] = useCanister("NFT_1000G_CANISTER")

    const handleButton = async () => {
        const res1g = await actor1g.sale(goldNft1gCart);
        const res10g = await actor10g.sale_info_batch_nft_origyn(goldNft10gCart);
        const res100g = await actor100g.sale_info_batch_nft_origyn(goldNft100gCart);
        const res1000g = await actor1000g.sale_info_batch_nft_origyn(goldNft1000gCart);
        console.log('res1g', res1g)
        console.log('res10g', res10g)
        console.log('res100g', res100g)
        console.log('res1000g', res1000g)
    };

    return (
        <><button onClick={() => handleButton()}>
            sale info
        </button></>
    );
};

export default SaleInfos;