import { useEffect, useState } from 'react';
import { gldNftAtom } from '@/states/nfts';
import { useAtom } from 'jotai';
import { Box } from '@mui/system';
import React from 'react';
import styled from 'styled-components';
import { useCanister, useWallet } from '@connect2ic/react';
import { gldNftCanisters } from '@/services/agents';

import medium from './../../../../public/images/gold/100g.png'
import { Button, Table, TableBody, TableCell, TableRow, Typography } from '@mui/material';
import Image from 'next/image';

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
          console.log(res?.ok?.current_sale[0]?.sale_type.auction.status.open, 'res?.ok?.current_sale[0]?.sale_type.auction.status.open');
          return {
            weight: nft.weight,
            name: nft.name,
            status: res?.ok?.current_sale[0]?.sale_type.auction.status.open === null
              ? res?.ok?.current_sale
              : undefined,
          };
        }),
      );
      setNfts(res);
      setIsLoading(false);
    };
    getNftStatus();
  }, [gldNfts]);

  const unlistHandler = async (token_id, weight) => {
    const ind = weights.indexOf(weight + 'g');
    const res = await actors[ind]?.sale_batch_nft_origyn([{ end_sale: token_id }]);
    console.log(res);
  };

  const Row = ({ row, }) => {
    return (
      <StyledTableRow key={row.name}>
        <StyledTableCell key="weight" padding="checkbox">
          {row.weight} {row.unit}
        </StyledTableCell>
        <StyledTableCell key="item">
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Image src={medium} alt={"NFT IMAGE"} />
            <ItemName>{row.name}</ItemName>
          </Box>
        </StyledTableCell>
        <StyledTableCell>
          {row.status ? 'On sale' : 'Not on sale'}
        </StyledTableCell>
        <StyledTableCell key="item">
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <CancelsaleButton token_id={row.name} weight={row.weight} />
          </Box>
        </StyledTableCell>
      </StyledTableRow>
    )
  }
  const CancelsaleButton = ({ token_id, weight }) => {
    console.log('token_id, weight', token_id, weight)
    return (
      <Button onClick={() => unlistHandler(token_id, weight)}>
        Cancel Sale
      </Button >
    )
  }

  return (
    <>
      {isLoading ? (
        <p>Loading nfts...</p>
      ) : (
        <Box sx={{ width: '100%' }}>
          <StyledTable>
            <TableBody>
              {nfts.map((nft) => {
                // console.log(nft);
                return (
                  <Row
                    row={nft}
                  />
                  // <li key={nft.name}>
                  //   {nft.status && (
                  //     <button key={nft.name} id={nft.weight + '_' + nft.name} onClick={unlistHandler}>
                  //       Unlist
                  //     </button>
                  //   )}
                  // </li>
                );
              })}
            </TableBody>
          </StyledTable >
        </Box>
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
const StyledTableRow = styled(TableRow)`
`

const StyledTableCell = styled(TableCell)`
    font-weight: inherit;
`

const StyledTable = styled(Table)`

`
const ItemName = styled(Typography)`
    height: 100%;
    align-items: center;
    display: inline-flex;
    padding-left: 16px;

`


