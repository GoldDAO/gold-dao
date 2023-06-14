
import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { NftGAgentAtom } from '../../states/agents/goldNft-1';
import { setGetNfts1000Atom, setGetNfts100Atom, setGetNfts10Atom, setGetNfts1Atom, setGetNftsAtom } from '../../states/nfts';


const GetOrigynNFTs = () => {
  const [NFtsAgent] = useAtom(NftGAgentAtom)
  const [Nft1, setNFTs1] = useAtom(setGetNfts1Atom)
  const [Nft10, setNFTs10] = useAtom(setGetNfts10Atom)
  const [Nft100, setNFTs100] = useAtom(setGetNfts100Atom)
  const [Nft1000, setNFTs1000] = useAtom(setGetNfts1000Atom)

  useEffect(() => {
    console.log('Nft1', Nft1)
  }, [Nft1])

  useEffect(() => {
    console.log('Nft10', Nft10)
  }, [Nft10])

  useEffect(() => {
    console.log('Nft100', Nft100)
  }, [Nft100])

  useEffect(() => {
    console.log('Nft1000', Nft1000)
  }, [Nft1000])

  // TODO REFACTORING NFTS STATES IN SINGLE STATE
  useEffect(() => {
    if (typeof NFtsAgent.NFT_1g.balance_of_nft_origyn === 'function') {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_1g.balance_of_nft_origyn({
          principal: Principal.fromText("4w3fz-5qxvk-ggkjo-kjkfd-sqc6k-jti6p-nbvt5-f6nqy-7foxk-kuboi-nae") // HOLDER TEST ADDRESS 
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
  }, [NFtsAgent.NFT_1g.balance_of_nft_origyn])

  useEffect(() => {
    if (typeof NFtsAgent.NFT_10g.balance_of_nft_origyn === 'function') {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_10g.balance_of_nft_origyn({
          principal: Principal.fromText("4w3fz-5qxvk-ggkjo-kjkfd-sqc6k-jti6p-nbvt5-f6nqy-7foxk-kuboi-nae") // HOLDER TEST ADDRESS
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
  }, [NFtsAgent.NFT_10g.balance_of_nft_origyn])

  useEffect(() => {
    if (typeof NFtsAgent.NFT_100g.balance_of_nft_origyn === 'function') {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_100g.balance_of_nft_origyn({
          principal: Principal.fromText("4w3fz-5qxvk-ggkjo-kjkfd-sqc6k-jti6p-nbvt5-f6nqy-7foxk-kuboi-nae") // HOLDER TEST ADDRESS
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
  }, [NFtsAgent.NFT_100g.balance_of_nft_origyn])

  useEffect(() => {
    if (typeof NFtsAgent.NFT_1000g.balance_of_nft_origyn === 'function') {
      async function getNFTBalance() {
        const nfts = await NFtsAgent.NFT_1000g.balance_of_nft_origyn({
          principal: Principal.fromText("4w3fz-5qxvk-ggkjo-kjkfd-sqc6k-jti6p-nbvt5-f6nqy-7foxk-kuboi-nae") // HOLDER TEST ADDRESS
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
  }, [NFtsAgent.NFT_1000g.balance_of_nft_origyn])

  return null;
};

export default GetOrigynNFTs;