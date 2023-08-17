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
import { useNft } from '@/hooks/useNFTs';
import { setGetUserAtom } from '@/states/user';
import { CancelsaleButton } from '@/components/commands/cancelSale';
import { useAllCanisters } from '@/hooks/useAllCanisters';

const MyNfts = () => {

  const weights = Object.keys(gldNftCanisters);
  const actors = useAllCanisters()
  const nfts = useNft(actors)

  const Row = ({ row }) => {
    return (
      <StyledTableRow>
        <StyledTableCell key="weight" padding="checkbox">
          {row.weight} {row.unit}
        </StyledTableCell>
        <StyledTableCell >
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            <Image src={medium} alt={"NFT IMAGE"} />
            <ItemName>{row.name}</ItemName>
          </Box>
        </StyledTableCell>
        <StyledTableCell>
          {row.status ? 'On sale' : 'Not on sale'}
        </StyledTableCell>
        <StyledTableCell>
          <Box sx={{ display: 'flex', alignItems: 'center' }}>
            {row.status &&
              <CancelsaleButton token_id={row.name} weight={row.weight} />}
          </Box>
        </StyledTableCell>
      </StyledTableRow>
    )
  }
  return (
    <Box sx={{ width: '100%' }}>
      <StyledTable>
        <TableBody>
          {nfts.nfts &&
            nfts.nfts.map((nft, i) => {
              return (
                <Row
                  key={i}
                  row={nft}
                />
              );
            })}
        </TableBody>
      </StyledTable >
    </Box>
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


