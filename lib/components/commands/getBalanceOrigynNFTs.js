
import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { NftGAgentAtom } from '../../states/agents/GLDNFT';
import { setGetNfts1000Atom, setGetNfts100Atom, setGetNfts10Atom, setGetNfts1Atom, setGetNftsAtom } from '../../states/nfts';
import { appStatusAtom } from '../../states/appStatus';
import { useWallet } from '@connect2ic/react';


const GetOrigynNFTs = () => {
  const [NFtsAgent] = useAtom(NftGAgentAtom)
  const [appStatus] = useAtom(appStatusAtom)
  const [, setNFTs1] = useAtom(setGetNfts1Atom)
  const [, setNFTs10] = useAtom(setGetNfts10Atom)
  const [, setNFTs100] = useAtom(setGetNfts100Atom)
  const [, setNFTs1000] = useAtom(setGetNfts1000Atom)
  const [wallet] = useWallet()

  useEffect(() => {
    console.log('wallet', wallet.principal)
  }, [wallet])

  useEffect(() => {
    if (appStatus === 'connected' && NFtsAgent.NFT_1g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_1g.balance_of_nft_origyn({
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
  }, [NFtsAgent.NFT_1g, appStatus, setNFTs1, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && NFtsAgent.NFT_10g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_10g.balance_of_nft_origyn({
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
  }, [NFtsAgent.NFT_10g, appStatus, setNFTs10, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && NFtsAgent.NFT_100g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_100g.balance_of_nft_origyn({
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
  }, [NFtsAgent.NFT_100g, appStatus, setNFTs100, wallet.principal])

  useEffect(() => {
    if (appStatus === 'connected' && NFtsAgent.NFT_1000g && wallet.principal) {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_1000g.balance_of_nft_origyn({
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
  }, [NFtsAgent.NFT_1000g, appStatus, setNFTs1000, wallet.principal])
  useEffect(() => {
    console.log('wallet', wallet)
  }, [wallet])
  return null;
};

export default GetOrigynNFTs;