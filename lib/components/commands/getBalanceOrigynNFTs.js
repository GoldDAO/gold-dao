
import React, { useState } from 'react';
import { useEffect } from 'react';
import { useAtom } from 'jotai';
import { Principal } from '@dfinity/principal';
import { setGetgoldNft1GAgentAtom } from '../../states/agents/goldNft-1';
import { getAllNftsAtom, setGetNfts1000Atom, setGetNfts100Atom, setGetNfts10Atom, setGetNfts1Atom, setGetNftsAtom } from '../../states/nfts';
import { setGetgoldNft10GAgentAtom } from '../../states/agents/goldNft-10';
import { setGetgoldNft1000GAgentAtom } from '../../states/agents/goldNft-1000';
import { setGetgoldNft100GAgentAtom } from '../../states/agents/goldNft-100';

const GetOrigynNFTs = () => {
  const [NFT1Agent] = useAtom(setGetgoldNft1GAgentAtom)
  const [NFT10Agent] = useAtom(setGetgoldNft10GAgentAtom)
  const [NFT100Agent] = useAtom(setGetgoldNft100GAgentAtom)
  const [NFT1000Agent] = useAtom(setGetgoldNft1000GAgentAtom)
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

  useEffect(() => {
    if (NFT1Agent.balance_of_nft_origyn) {
      async function getNFTBalance() {
        const nfts = await NFT1Agent.balance_of_nft_origyn({
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
  }, [NFT1Agent])

  useEffect(() => {
    if (NFT10Agent.balance_of_nft_origyn) {
      async function getNFTBalance() {
        const nfts = await NFT10Agent.balance_of_nft_origyn({
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
  }, [NFT10Agent])

  useEffect(() => {
    if (NFT100Agent.balance_of_nft_origyn) {
      async function getNFTBalance() {
        const nfts = await NFT100Agent.balance_of_nft_origyn({
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
  }, [NFT100Agent])

  useEffect(() => {
    if (NFT1000Agent.balance_of_nft_origyn) {
      async function getNFTBalance() {
        const nfts = await NFT1000Agent.balance_of_nft_origyn({
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
  }, [NFT1000Agent])

  return null;
};

export default GetOrigynNFTs;