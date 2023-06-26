import React, { useEffect } from 'react';
import { Button } from '@mui/material';
import { useAtom } from 'jotai';
import { cartAtom } from '../../states/cart';
import { NftGAgentAtom } from '../../states/agents/GLDNFT';
import { Principal } from '@dfinity/principal';

import BigNumber from 'bignumber.js';

const ICP_LEDGER_CANISTER_ID = 'ryjl3-tyaaa-aaaaa-aaaba-cai';
const YUMI_KYC_CANISTER_ID = 'ucs6g-wiaaa-aaaah-abwpa-cai';
const GLDT_CANISTER_ID = 'dgeb6-wyaaa-aaaap-abeha-cai'; // this is wrong, to be replaced

const BatchOffers = () => {
  const [cart] = useAtom(cartAtom);
  const [agent] = useAtom(NftGAgentAtom);

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

  const salePrice = 100 * 10 ** 8;
  const brokerId = 'gnfh7-n3zkc-7ihoh-vp4my-pu5zn-ssgxf-issex-akzuf-dvub3-dqu5x-zae';

  const payload = [
    {
      token_id: 'gold-012750',
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
    },
  ];
  //   const payload = [
  //     {
  //       token_id: 'gold-012750',
  //       sales_config: {
  //         escrow_receipt: [],
  //         broker_id: [],
  //         pricing: {
  //           ask: [
  //             [
  //               {
  //                 kyc: Principal.fromText(YUMI_KYC_CANISTER_ID),
  //                 start_price: salePrice,
  //                 reserve: salePrice,
  //                 buy_now: salePrice,
  //                 notify: [Principal.fromText(GLDT_CANISTER_ID)],
  //                 token: {
  //                   ic: {
  //                     standard: { ledger: [] },
  //                     canister: Principal.fromText(ICP_LEDGER_CANISTER_ID),
  //                     decimals: 8,
  //                     fee: [10000],
  //                     symbol: 'ICP',
  //                     id: [],
  //                   },
  //                 },
  //               },
  //             ],
  //           ],
  //         },
  //       },
  //     },
  //   ];
  const handleButton = async () => {
    console.log('listing NFT');
    console.log(payload[0]);
    const res = await agent.NFT_1g.market_transfer_nft_origyn(payload[0]);
    console.log(res);
  };

  return (
    <>
      {/* <Button onClick={() => agent.NFT_1g.market_transfer_batch_nft_origyn(payload)}> */}
      <Button onClick={() => handleButton()}>send batch offer</Button>
    </>
  );
};

export default BatchOffers;

// const payload = [
//     {
//       token_id: 'gold-012750',
//       sales_config: {
//         escrow_receipt: [],
//         broker_id: [],
//         pricing: {
//           ask: [
//             [
//               {
//                 kyc: Principal.fromText(YUMI_KYC_CANISTER_ID),
//               },
//               {
//                 start_price: salePrice,
//               },
//               {
//                 reserve: salePrice,
//               },
//               {
//                 buy_now: salePrice,
//               },
//               { notfiy: [Principal.fromText(GLDT_CANISTER_ID)] },
//               //   {
//               //     token: {
//               //       ic: {
//               //         standard: { ledger: [] },
//               //         canister: Principal.fromText(ICP_LEDGER_CANISTER_ID),
//               //         decimals: 8,
//               //         fee: [10000],
//               //         symbol: 'ICP',
//               //         id: [],
//               //       },
//               //     },
//               //   },
//             ],
//           ],
//         },
//       },
//     },
//   ];
