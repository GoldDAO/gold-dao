import { useEffect, useState } from 'react';
import { gldNftAtom } from '@/states/nfts';
import { useAtom } from 'jotai';
import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import { useCanister, useWallet } from '@connect2ic/react';
import { gldNftCanisters } from '@/services/agents';

const MyNfts = ({ data }) => {
  const [gldNfts] = useAtom(gldNftAtom);
  const [nfts, setNfts] = useState([]);
  const [isLoading, setIsLoading] = useState(false);

  const weights = Object.keys(gldNftCanisters);
  const actors = weights.map((w) => useCanister(w)[0]);

  useEffect(() => {
    const getNftStatus = async () => {
      setIsLoading(true);
      const res = await Promise.all(
        gldNfts.map(async (nft, i) => {
          const ind = weights.indexOf(nft.weight + 'g');
          const res = await actors[ind]?.nft_origyn(nft.name);
          console.log(res);
          return {
            weight: nft.weight,
            name: nft.name,
            status: res?.ok?.current_sale[0]?.sale_type.auction.status.open
              ? res?.ok?.current_sale
              : undefined,
          };
        }),
      );
      // console.log(res);
      setNfts(res);
      setIsLoading(false);
    };
    getNftStatus();
  }, [gldNfts]);

  const unlistHandler = async (e) => {
    // console.log(e);
    console.log(e.target.id);
    const [weight, name] = e.target.id.split('_');
    const ind = weights.indexOf(weight + 'g');
    const res = await actors[ind]?.sale_batch_nft_origyn([{ end_sale: name }]);
    console.log(res);
  };

  console.log('nfts:', nfts);
  return (
    <>
      {isLoading ? (
        <p>Loading nfts...</p>
      ) : (
        <ul>
          {nfts.map((nft) => {
            // console.log(nft);
            return (
              <li key={nft.name}>
                {nft.name}, {nft.weight}g, {nft.status ? 'On sale' : 'Not on sale'}
                {nft.status && (
                  <button key={nft.name} id={nft.weight + '_' + nft.name} onClick={unlistHandler}>
                    Unlist
                  </button>
                )}
              </li>
            );
          })}
        </ul>
      )}
    </>
  );
};

export default MyNfts;

const MarketCapContainer = styled(Box)`
  font-size: 1em;
  background-color: #f7f7f7;
  width: fit-content;
  padding: 20px;
  border-radius: 10px;
  display: flex;
  @media (max-width: 940px) {
    font-size: 0.8em;
  }
  @media (max-width: 540px) {
    width: 100%;
  }
`;
