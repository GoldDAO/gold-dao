
import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { setGetNfts1000Atom, setGetNfts100Atom, setGetNfts10Atom, setGetNfts1Atom, setGetNftsAtom } from '../../states/nfts';
import { appStatusAtom } from '../../states/appStatus';
import { useCanister, useWallet } from '@connect2ic/react';


const GetBalanceOrigynNFTs = () => {
  const [appStatus] = useAtom(appStatusAtom)
  const [, setNFTs1] = useAtom(setGetNfts1Atom)
  const [, setNFTs10] = useAtom(setGetNfts10Atom)
  const [, setNFTs100] = useAtom(setGetNfts100Atom)
  const [, setNFTs1000] = useAtom(setGetNfts1000Atom)
  const [wallet] = useWallet()

  const [actor1g] = useCanister("NFT_1G_CANISTER", { mode: "anonymous" })
  const [actor10g] = useCanister("NFT_10G_CANISTER", { mode: "anonymous" })
  const [actor100g] = useCanister("NFT_100G_CANISTER", { mode: "anonymous" })
  const [actor1000g] = useCanister("NFT_1000G_CANISTER", { mode: "anonymous" })

  useEffect(() => {
    if (appStatus === 'connected') {
      async function getNFTBalance() {
        const nfts1 = await actor1g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        const nfts10 = await actor10g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        const nfts100 = await actor100g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        const nfts1000 = await actor1000g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        return { nfts1, nfts10, nfts100, nfts1000 };
      }
      getNFTBalance()

        .then(result => {
          console.log('result', result)
          setNFTs1(result.nfts1.ok.nfts.map((e) => ({ name: e, weight: 1 })))
          setNFTs10(result.nfts10.ok.nfts.map((e) => ({ name: e, weight: 10 })))
          setNFTs100(result.nfts100.ok.nfts.map((e) => ({ name: e, weight: 100 })))
          setNFTs1000(result.nfts1000.ok.nfts.map((e) => ({ name: e, weight: 1000 })))
        })
        .catch(error => {
          console.error(error);
        });
    }
  }, [appStatus])

  useEffect(() => {
    console.log('wallet', wallet)
  }, [wallet])

  return null;
};

export default GetBalanceOrigynNFTs;