
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
    if (appStatus === 'connected' && actor1g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await actor1g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        return nfts;
      }
      getNFTBalance()
        .then(result => {
          setNFTs1(result.ok.nfts.map((e) => ({ name: e, weight: 1 })))
        })
        .catch(error => {
          console.error(error);
        });
    }
  }, [actor1g, appStatus, setNFTs1, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && actor10g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await actor10g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        return nfts;
      }
      getNFTBalance()
        .then(result => {
          setNFTs10(result.ok.nfts.map((e) => ({ name: e, weight: 10 })))
        })
        .catch(error => {
          console.error(error);
        });
    }
  }, [actor10g, appStatus, setNFTs10, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && actor100g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await actor100g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        return nfts;
      }
      getNFTBalance()
        .then(result => {
          setNFTs100(result.ok.nfts.map((e) => ({ name: e, weight: 100 })))
        })
        .catch(error => {
          console.error(error);
        });
    }
  }, [actor100g, appStatus, setNFTs100, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && actor1000g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await actor1000g.balance_of_nft_origyn({
          principal: Principal.fromText(wallet.principal)
        });
        return nfts;
      }
      getNFTBalance()
        .then(result => {
          setNFTs1000(result.ok.nfts.map((e) => ({ name: e, weight: 1000 })))
        })
        .catch(error => {
          console.error(error);
        });
    }
  }, [actor1000g, appStatus, setNFTs1000, wallet.principal])
  useEffect(() => {
    console.log('wallet', wallet)
  }, [wallet])
  return null;
};

export default GetBalanceOrigynNFTs;