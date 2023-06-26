import React, { useEffect } from 'react';
import { Button } from '@mui/material'
import { useAtom } from 'jotai'
import { cartAtom } from '../../states/cart';
import { NftGAgentAtom } from '../../states/agents/GLDNFT';

const BatchOffers = () => {
    const [cart,] = useAtom(cartAtom)
    const [agent] = useAtom(NftGAgentAtom)

    /* 
    market_transfer_nft_origyn(
  #ask({
    token_id = "1" : text;
    sales_config = {
        escrow_receipt = null;
        broker_id = null; //can be an opt Principal
        pricing = #ask (opt vec {
          #reserve(100 * 10 ** 8), //reserve price below you do not want to sell
          #token(#ic({
            canister = Principal.fromActor(dfx); //the principal from the ledger you want to transact in
            standard =  #Ledger;
            decimals = 8;
            symbol = "GLDT";
            fee = ?10000;
            id = null; //null unless you are on a multi-token ledger
          })),
          #buy_now(500 * 10 ** 8),  //the sale price for listings -- remove for an auctions style sale
          #start_price(1 * 10 ** 8), //set this equal to the buy now price if doing a classic listing
          #ending(#date(get_time() + DAY_LENGTH)), //if you omit this the sale will last 1 minute and the token will be locked
          #min_increase(#amount(10*10**8)), //not necessary for buy it now
          #notify([Principal.fromActor(a_wallet),
          Principal.fromActor(b_wallet)]) //list of principals to notify - max 5;
          });
    };
  }));
  */

    // market_transfer_nft_origyn_batch([token_id, sales_config = ask([notify[GLDT], buy_now(Price)])
    const payload = [{
        token_id: 'token_id',
        sales_config: {
            escrow_receipt: null,
            broker_id: null,
            pricing: {
                ask: {
                    reserve: (100 * 10 ** 8),
                    start_price: 0,
                    token: {
                        ic: {
                            standard: { ledger: null },
                            canister: '',
                            decimals: 8,
                            fee: 10000,
                            symbol: 'GLDT',
                            id: null,
                        },
                    },
                    reserve: [100 * 10 ** 8],
                    start_date: BigInt(+new Date() * 1e6),
                    notify: '',
                },
                buy_now: (500 * 10 ** 8),  //the sale price for listings -- remove for an auctions style sale
                start_price: (1 * 10 ** 8), //set this equal to the buy now price if doing a classic listing
                // ending: , //if you omit this the sale will last 1 minute and the token will be locked
                // notify: ([Principal.fromActor(a_wallet),
                // Principal.fromActor(b_wallet)]) //list of principals to notify - max 5;
            },
        },
    }]



    return (
        <>
            <Button onClick={() => agent.NFT_1g.market_transfer_batch_nft_origyn(payload)}>send batch offer</Button>
        </>
    );
};

export default BatchOffers;