import React, { useEffect, useState } from 'react';
import styled from 'styled-components';
import {
  TableRow,
  Table,
  TableCell,
  Checkbox,
  TableHead,
  Typography,
  TableBody,
} from '@mui/material';
import Image from 'next/image';
import { Box } from '@mui/system';
import { useAtom } from 'jotai';
import {
  addAllItemsAtom,
  addCartItemAtom,
  getCartAtom,
  getTotalCartWeightAtom,
  removeAllItemsInCartAtom,
  removeCartItemByIdAtom,
} from '../../../states/cart';
import { getAllNftsAtom, setGetGldNftsAtom } from './../../../states/nfts';

import medium from './../../../../public/images/gold/100g.png';

const tableHead = [
  {
    key: 'weight',
    label: 'Weight',
  },
  {
    key: 'name',
    label: 'name',
  },
];

const NFTTable = () => {
  const [allNfts] = useAtom(setGetGldNftsAtom);
  console.log(allNfts);
  //   const [allNfts] = useAtom(getAllNftsAtom);

  const [cart] = useAtom(getCartAtom);
  const [, removeAllItems] = useAtom(removeAllItemsInCartAtom);
  const [, addAllNFTsInCart] = useAtom(addAllItemsAtom);
  const [isAllSelected, setIsAllSelected] = useState(false);

  useEffect(() => {
    console.log('allNfts', allNfts);
  }, [allNfts]);

  useEffect(() => {
    if (cart.length === allNfts.length) {
      setIsAllSelected(true);
    } else {
      setIsAllSelected(false);
    }
  }, [cart]);

  return (
    <StyledTable size="small">
      <StyledTableHead>
        <StyledTableRow>
          <StyledTableCell padding="checkbox">
            <StyledCheckbox
              onChange={(e) => {
                e.target.checked ? addAllNFTsInCart() : removeAllItems();
              }}
              checked={isAllSelected}
              inputProps={{ 'aria-label': 'select all NFTs' }}
            />
          </StyledTableCell>
          {tableHead.map((e, i) => (
            <StyledTableCell key={e.key}>{e.label}</StyledTableCell>
          ))}
        </StyledTableRow>
      </StyledTableHead>
      <TableBody>
        {allNfts.map((e, i) => (
          <Row row={e} key={i} />
        ))}
      </TableBody>
    </StyledTable>
  );
};

export default NFTTable;

const Row = ({ row }) => {
  const [isInCart, setIsInCart] = useState(false);
  const [cart] = useAtom(getCartAtom);
  const [, setCartItem] = useAtom(addCartItemAtom);
  const [, removeItemFromCart] = useAtom(removeCartItemByIdAtom);

  useEffect(() => {
    const a = cart.find((e) => e?.name === row.name);
    setIsInCart(a ? true : false);
  }, [cart, row]);
  return (
    <StyledTableRow key={row.name}>
      <StyledTableCell padding="checkbox">
        <StyledCheckbox
          onChange={(e) => {
            e.target.checked ? setCartItem(row) : removeItemFromCart(row.name);
          }}
          inputProps={{ 'aria-label': `select NFTs with name ${row.name}` }}
          checked={isInCart}
        />
      </StyledTableCell>
      <StyledTableCell key="weight" padding="checkbox">
        {row.weight} {row.unit}
      </StyledTableCell>
      <StyledTableCell key="item">
        <Box sx={{ display: 'flex', alignItems: 'center' }}>
          <Image src={medium} alt={'NFT IMAGE'} />
          <ItemName>{row.name}</ItemName>
        </Box>
      </StyledTableCell>
    </StyledTableRow>
  );
};

const StyledTableRow = styled(TableRow)``;

const StyledTableCell = styled(TableCell)`
  font-weight: inherit;
`;

const StyledTableHead = styled(TableHead)`
  font-weight: 600;
`;
const StyledCheckbox = styled(Checkbox)``;
const StyledTable = styled(Table)``;
const ItemName = styled(Typography)`
  height: 100%;
  align-items: center;
  display: inline-flex;
  padding-left: 16px;
`;
