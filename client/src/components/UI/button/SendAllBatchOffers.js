import React, { useEffect } from 'react';
import { Button } from '@mui/material';
import { useAtom } from 'jotai';
import { cartAtom } from '@/states/cart';
import { Principal } from '@dfinity/principal';
import { useCanister, useWallet } from '@connect2ic/react';
import MainButton from '@/components/UI/button/Buttons';
import { gldNftCanisters } from '@/services/agents';
import { useAllCanisters } from '@/components/hooks/useAllCanisters';

export const SendBatchOffersButton = () => {
  const [wallet] = useWallet();
  const [cart] = useAtom(cartAtom);

  const weights = Object.keys(gldNftCanisters);
  const actors = useAllCanisters()

  const salePrice = 100 * 10 ** 8;

  const gldNftCart = {};

  const YUMI_KYC_CANISTER_ID = process.env.YUMI_KYC_CANISTER_ID;
  const ICP_LEDGER_CANISTER_ID = process.env.ICP_LEDGER_CANISTER_ID;

  const payload = (e) => {
    return {
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
              { notify: [Principal.fromText(process.env.GLDT_CANISTER_ID)] },
              {
                token: {
                  ic: {
                    standard: { Ledger: null },
                    canister: Principal.fromText(process.env.GLDT_LEDGER_CANISTER_ID),
                    decimals: 8,
                    fee: [10000],
                    symbol: 'GLDT',
                    id: [],
                  },
                },
              },
            ],
          ],
        },
      },
    };
  };

  cart.map((e) => {
    if (gldNftCart[e.weight]) {
      gldNftCart[e.weight].push(payload(e));
    } else {
      gldNftCart[e.weight] = [payload(e)];
    }
  });

  const handleButton = async () => {
    const res = await Promise.all(
      weights.map((w, i) => {
        const w_int = +w.slice(0, -1);
        if (gldNftCart[w_int]) return actors[i].market_transfer_batch_nft_origyn(gldNftCart[w_int]);
        else return undefined;
      }),
    );
    console.log('res', res);
  };

  return <MainButton label="Confirm" action={handleButton} />;
};
