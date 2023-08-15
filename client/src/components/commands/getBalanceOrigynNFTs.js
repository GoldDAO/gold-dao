import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { setGetGldNftsAtom } from '@/states/nfts';
import { appStatusAtom } from '@/states/appStatus';
import { useCanister, useWallet } from '@connect2ic/react';
import { gldNftCanisters } from '@/services/agents';

const GetBalanceOrigynNFTs = () => {
  const [appStatus] = useAtom(appStatusAtom);
  const [, setGldNfts] = useAtom(setGetGldNftsAtom);
  const [wallet] = useWallet();

  const weights = Object.keys(gldNftCanisters);
  const actors = weights.map((w) => useCanister(w, { mode: 'anonymous' })[0]);

  useEffect(() => {
    if (appStatus === 'connected') {
      try {
        const getNFTBalance = async () => {
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

  useEffect(() => {
    console.log('wallet', wallet);
  }, [wallet]);

  return null;
};

export default GetBalanceOrigynNFTs;
