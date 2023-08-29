import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { setGetGldNftsAtom } from '../../states/nfts';
import { appStatusAtom } from '../../states/appStatus';
import { useCanister, useWallet } from '@connect2ic/react';
import {
  onSaleNft10Atom,
  onSaleNftAtom,
  updateOnsaleNft,
  updateOnsaleNftAtom,
} from '../../states/onSalesNfts';
import { gldNftCanisters } from '@/services/agents';

const GetBalanceOrigynNFTs = () => {
  const [appStatus] = useAtom(appStatusAtom);
  const [gld, setGldNfts] = useAtom(setGetGldNftsAtom);
  const [wallet] = useWallet();
  const weights = Object.keys(gldNftCanisters);
  const [onSalesNfts, setOnSalesNfts] = useAtom(onSaleNftAtom);

  const actors = weights.map((w) => useCanister(w, { mode: 'anonymous' })[0]);

  // useEffect(() => {
  //   console.log('actors', actors);
  // }, [actors]);

  useEffect(() => {
    if (appStatus === 'connected') {
      try {
        const getNFTBalance = async () => {
          // console.log('actors', actors)
          const nft_promises = actors.map((actor) =>
            actor.balance_of_nft_origyn({
              principal: Principal.fromText(wallet.principal),
            }),
          );
          const res = await Promise.all(nft_promises);
          let nfts = [];
          res.forEach((r, i) =>
            nfts.push(...r.ok?.nfts.map((e) => ({ name: e, weight: +weights[i].slice(0, -1) }))),
          );
          setGldNfts(nfts);
        };
        getNFTBalance();
      } catch (e) {
        console.log(e);
      }
    }
  }, [appStatus]);

  // useEffect(() => {

  //   async function getOnSaleNfts(token_id, g) {
  //     const res = await Promise.all(weights.map(async (e, i) => {
  //       if (g === parseInt(e.slice(0, -1))) {
  //         return actors[i].nft_origyn(token_id);
  //       }
  //     }));
  //     const tokenOnSale = res.filter(element => element !== undefined);
  //     if (tokenOnSale[0].ok.current_sale.length > 0) {
  //       return {
  //         name: token_id,
  //         weight: g,
  //         sale_status: Object.keys(tokenOnSale[0].ok.current_sale[0].sale_type.auction.status[0])
  //       }
  //     }
  //   }

  //   async function fetchAndProcessOnSaleNFTs() {
  //     const onSaleNFTs = [];
  //     for (const item of gld) {
  //       const tokenOnSale = await getOnSaleNfts(item.name, item.weight);
  //       onSaleNFTs.push(tokenOnSale);
  //     }
  //     setOnSalesNfts(onSaleNFTs)
  //   }
  //   fetchAndProcessOnSaleNFTs();
  // }, [gld])

  return null;
};

export default GetBalanceOrigynNFTs;
