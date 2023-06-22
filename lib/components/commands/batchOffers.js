import React, { useEffect } from 'react';
import { Button } from '@mui/material'
import { useAtom } from 'jotai'
import { cartAtom } from '../../states/cart';
import { NftGAgentAtom } from '../../states/agents/GLDNFT';

const BatchOffers = () => {
    const [cart,] = useAtom(cartAtom)
    const [agent] = useAtom(NftGAgentAtom)

    const payload = [{
        token_id: cart[0].name,
    }]

    return (
        <>
            <Button onClick={() => agent.NFT_1g.market_transfer_batch_nft_origyn(payload)}>send batch offer</Button>
        </>
    );
};

export default BatchOffers;